// use futures::future::Future;
use std::path::Path;

use async_trait::async_trait;
use chrono::NaiveDate;
use fl_www_core::markdown::HtmlCreator;
pub(crate) use metadata::*;
use pulldown_cmark::Parser;
use tokio::fs::{read_dir, File};
use tokio::io::AsyncReadExt;
use unicode_segmentation::UnicodeSegmentation;

use crate::prelude::*;

#[async_trait]
pub(crate) trait MetadataExt: Sized {
    async fn from_path<P: AsRef<Path> + Sync + Send>(path: P) -> anyhow::Result<Self>;
}

#[async_trait]
impl MetadataExt for Metadata {
    async fn from_path<P: AsRef<Path> + Send + Sync>(path: P) -> anyhow::Result<Self> {
        let path = path.as_ref();

        // loading writings
        let writing_dir = path.join("writings");

        async fn load_writings_by_date(
            dir: &Path,
            lang: Language,
            date: NaiveDate,
        ) -> anyhow::Result<Vec<WritingMetadata>> {
            let mut entries = read_dir(dir).await?;
            let mut writings = Vec::new();

            // pub struct WritingMetadata {
            //     pub slug: String,
            //     pub lang: Language,
            //     pub date: NaiveDate,
            //     pub title: String,
            // }

            while let Some(m) = entries.next_entry().await? {
                let file_name = match m.file_name().to_str() {
                    Some(m) => m.to_owned(),
                    None => continue,
                };

                let slug = match file_name.rsplit_once('.') {
                    Some(m) => m.0.to_owned(),
                    None => continue,
                };

                let mut full_content = String::new();
                File::open(m.path())
                    .await?
                    .read_to_string(&mut full_content)
                    .await?;

                let (title, content) = full_content.split_once('\n').unwrap_or((&full_content, ""));
                let mut title = title.to_string();
                let content = content.trim().to_string();

                let root = HtmlCreator::new(Parser::new(&content)).into_root_node();

                while title.starts_with('#') {
                    title.remove(0);
                }
                let title = title.trim().to_string();

                let summary: String =
                    root.to_text()
                        .graphemes(true)
                        .take(200)
                        .fold(String::new(), |mut s, c| {
                            if c != " " || !s.ends_with(' ') {
                                s.push_str(c);
                            }

                            s
                        });

                writings.push(WritingMetadata {
                    slug,
                    lang,
                    date,
                    title,
                    summary,
                });
            }

            Ok(writings)
        }

        async fn load_writings_by_lang(
            dir: &Path,
            lang: Language,
        ) -> anyhow::Result<Vec<WritingMetadata>> {
            let mut entries = read_dir(dir).await?;
            let mut writings = Vec::new();

            while let Some(m) = entries.next_entry().await? {
                let file_name = match m.file_name().to_str() {
                    Some(m) => m.to_owned(),
                    None => continue,
                };

                let d = match NaiveDate::parse_from_str(&file_name, "%Y-%m-%d") {
                    Ok(m) => m,
                    Err(_) => continue,
                };

                writings.extend(load_writings_by_date(&m.path(), lang, d).await?);
            }

            Ok(writings)
        }

        let mut writings = Vec::new();

        // {language}/{date}/{slug}.md
        let mut entries = read_dir(writing_dir).await?;
        while let Some(m) = entries.next_entry().await? {
            let file_name = match m.file_name().to_str() {
                Some(m) => m.to_owned(),
                None => continue,
            };
            let lang = match file_name.parse::<Language>() {
                Ok(m) => m,
                Err(_) => continue,
            };

            writings.extend(load_writings_by_lang(&m.path(), lang).await?);
        }

        writings.sort_by_key(|m| m.date);
        writings.reverse();

        Ok(Metadata::builder().writings(writings).build())
    }
}
