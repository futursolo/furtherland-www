use std::fs::OpenOptions;
use std::path::Path;

use atom_syndication::{
    EntryBuilder, Feed as Channel, FeedBuilder as ChannelBuilder, GeneratorBuilder, LinkBuilder,
    Text,
};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use metadata::Metadata;

use crate::prelude::*;

#[derive(Debug)]
pub struct Feed {
    zh_chn: Channel,
    en_chn: Channel,
}

impl Feed {
    pub fn from_metadata(metadata: &Metadata) -> Feed {
        let mut zh_writings = Vec::new();
        let mut en_writings = Vec::new();

        let mut latest = NaiveDate::from_ymd(1970, 1, 1);

        for writing in metadata.writings() {
            if writing.date > latest {
                latest = writing.date.to_owned();
            }

            match writing.lang {
                Language::Chinese => zh_writings.push(writing),
                Language::English => en_writings.push(writing),
            };
        }

        let time = NaiveTime::from_hms(0, 0, 0);
        let latest_datetime = NaiveDateTime::new(latest, time);
        let latest_datetime: DateTime<FixedOffset> =
            DateTime::from_utc(latest_datetime, FixedOffset::east(0));

        let zh_entries = zh_writings
            .iter()
            .map(|m| {
                let time = NaiveTime::from_hms(0, 0, 0);
                let datetime = NaiveDateTime::new(m.date.to_owned(), time);
                let datetime: DateTime<FixedOffset> =
                    DateTime::from_utc(datetime, FixedOffset::east(0));

                EntryBuilder::default()
                    .title(Text::plain(&m.title))
                    .link(
                        LinkBuilder::default()
                            .href(format!("https://www.futures.moe/zh/writings/{}", m.slug))
                            .hreflang(m.lang.as_str().to_owned())
                            .mime_type(Some("text/html".to_owned()))
                            .rel("alternate")
                            .title(m.title.to_string())
                            .build(),
                    )
                    .summary(Text::plain(&m.summary))
                    .updated(datetime)
                    .build()
            })
            .collect::<Vec<_>>();

        let en_entries = en_writings
            .iter()
            .map(|m| {
                let time = NaiveTime::from_hms(0, 0, 0);
                let datetime = NaiveDateTime::new(m.date.to_owned(), time);
                let datetime: DateTime<FixedOffset> =
                    DateTime::from_utc(datetime, FixedOffset::east(0));

                EntryBuilder::default()
                    .title(Text::plain(&m.title))
                    .link(
                        LinkBuilder::default()
                            .href(format!("https://www.futures.moe/zh/writings/{}", m.slug))
                            .hreflang(m.lang.as_str().to_owned())
                            .mime_type(Some("text/html".to_owned()))
                            .rel("alternate")
                            .title(m.title.to_string())
                            .build(),
                    )
                    .summary(Text::plain(&m.summary))
                    .updated(datetime)
                    .build()
            })
            .collect::<Vec<_>>();

        let zh_chn = ChannelBuilder::default()
            .title("?????????????????????")
            .entries(zh_entries)
            .link(
                LinkBuilder::default()
                    .href("https://www.futures.moe/zh/")
                    .hreflang(Language::Chinese.as_str().to_owned())
                    .mime_type(Some("text/html".to_owned()))
                    .rel("alternate")
                    .title(Some("?????????????????????".to_string()))
                    .build(),
            )
            .generator(
                GeneratorBuilder::default()
                    .value("Furtherland")
                    .uri(Some(
                        "https://github.com/futursolo/furtherland-www".to_string(),
                    ))
                    .version(Some(env!("CARGO_PKG_VERSION").to_string()))
                    .build(),
            )
            .updated(latest_datetime)
            .build();

        let en_chn = ChannelBuilder::default()
            .title("Hoshikawa's Secret Room")
            .entries(en_entries)
            .link(
                LinkBuilder::default()
                    .href("https://www.futures.moe/en/")
                    .hreflang(Language::Chinese.as_str().to_owned())
                    .mime_type(Some("text/html".to_owned()))
                    .rel("alternate")
                    .title(Some("Hoshikawa's Secret Room".to_string()))
                    .build(),
            )
            .generator(
                GeneratorBuilder::default()
                    .value("Furtherland")
                    .uri(Some(
                        "https://github.com/futursolo/furtherland-www".to_string(),
                    ))
                    .version(Some(env!("CARGO_PKG_VERSION").to_string()))
                    .build(),
            )
            .updated(latest_datetime)
            .build();

        Feed { zh_chn, en_chn }
    }

    pub fn write_feeds<P: AsRef<Path>>(&self, dir: P) -> anyhow::Result<()> {
        let dir = dir.as_ref();

        let en_feed_path = dir.join("feed-en.xml");
        let zh_feed_path = dir.join("feed-zh.xml");

        let mut zh_f = OpenOptions::new()
            .write(true)
            .create(true)
            .open(zh_feed_path)?;
        let mut en_f = OpenOptions::new()
            .write(true)
            .create(true)
            .open(en_feed_path)?;

        self.en_chn.write_to(&mut en_f)?;
        self.zh_chn.write_to(&mut zh_f)?;

        Ok(())
    }
}
