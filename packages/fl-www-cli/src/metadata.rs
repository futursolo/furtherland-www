// use futures::future::Future;
use std::path::Path;

use async_trait::async_trait;
use chrono::NaiveDate;
use tokio::fs::{read_dir, File};
use tokio::io::AsyncReadExt;

use crate::prelude::*;
pub(crate) use metadata::*;

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

                let mut content = String::new();
                File::open(m.path())
                    .await?
                    .read_to_string(&mut content)
                    .await?;

                let mut title = content
                    .split_once("\n")
                    .map(|m| m.0)
                    .unwrap_or("")
                    .trim()
                    .to_string();

                while title.starts_with('#') {
                    title.remove(0);
                }

                title.trim().to_string();

                writings.push(WritingMetadata {
                    slug,
                    lang,
                    date,
                    title,
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

// fn load_writings(&mut self) {
//     for path in crate::tmpfs::Writings::iter() {
//         // {language}/{date}/{slug}.md
//         let path_buf = PathBuf::from(path.as_ref());

//         if path_buf.extension() != Some("md".as_ref()) {
//             continue;
//         }

//         let mut path_iter = path_buf.iter();

//         let lang = match path_iter
//             .next()
//             .and_then(|m| m.to_str())
//             .and_then(|m| m.parse::<Language>().ok())
//         {
//             Some(m) => m,
//             None => continue,
//         };

//         let date = match path_iter
//             .next()
//             .and_then(|m| m.to_str())
//             .and_then(|m| NaiveDate::parse_from_str(m, "%Y-%m-%d").ok())
//         {
//             Some(m) => m,
//             None => continue,
//         };

//         let slug = match path_iter
//             .next()
//             .and_then(|m| m.to_str())
//             .and_then(|m| m.rsplit_once('.'))
//         {
//             Some(m) => m.0.to_owned(),
//             None => continue,
//         };

//         let content = match crate::tmpfs::Writings::get(&path) {
//             Some(m) => m.data,
//             None => continue,
//         };

//         self.writings.push({
//             WritingMetadata {
//                 slug,
//                 lang,
//                 date,
//                 content,
//             }
//         });
//     }

//     // Sort by date.
//     self.writings.sort_by(|a, b| a.date.cmp(&b.date));
// }
