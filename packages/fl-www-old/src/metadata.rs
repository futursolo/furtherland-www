use std::borrow::Cow;
use std::path::PathBuf;

use chrono::NaiveDate;
use once_cell::sync::Lazy;

use crate::prelude::*;

use i18n::Language;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct WritingMetadata {
    pub slug: String,
    pub lang: Language,
    pub date: NaiveDate,
    pub content: Cow<'static, [u8]>,
}

impl WritingMetadata {
    pub fn get_title(&self) -> String {
        let mut title = String::from_utf8_lossy(self.content.as_ref())
            .split_once("\n")
            .map(|m| m.0)
            .unwrap_or("")
            .trim()
            .to_string();

        while title.starts_with('#') {
            title.remove(0);
        }

        title.trim().to_string()
    }
    pub fn get_content(&self) -> String {
        String::from_utf8_lossy(self.content.as_ref())
            .split_once("\n")
            .map(|m| m.1)
            .unwrap_or("")
            .trim()
            .to_string()
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) struct Metadata {
    writings: Vec<WritingMetadata>,
}

impl Metadata {
    fn new() -> Self {
        let mut self_ = Self::default();
        self_.load_writings();

        self_
    }

    pub fn writings(&self) -> &Vec<WritingMetadata> {
        &self.writings
    }

    fn load_writings(&mut self) {
        for path in crate::tmpfs::Writings::iter() {
            // {language}/{date}/{slug}.md
            let path_buf = PathBuf::from(path.as_ref());

            if path_buf.extension() != Some("md".as_ref()) {
                continue;
            }

            let mut path_iter = path_buf.iter();

            let lang = match path_iter
                .next()
                .and_then(|m| m.to_str())
                .and_then(|m| m.parse::<Language>().ok())
            {
                Some(m) => m,
                None => continue,
            };

            let date = match path_iter
                .next()
                .and_then(|m| m.to_str())
                .and_then(|m| NaiveDate::parse_from_str(m, "%Y-%m-%d").ok())
            {
                Some(m) => m,
                None => continue,
            };

            let slug = match path_iter
                .next()
                .and_then(|m| m.to_str())
                .and_then(|m| m.rsplit_once('.'))
            {
                Some(m) => m.0.to_owned(),
                None => continue,
            };

            let content = match crate::tmpfs::Writings::get(&path) {
                Some(m) => m.data,
                None => continue,
            };

            self.writings.push({
                WritingMetadata {
                    slug,
                    lang,
                    date,
                    content,
                }
            });
        }

        // Sort by date.
        self.writings.sort_by(|a, b| a.date.cmp(&b.date));
    }

    pub fn get() -> &'static Self {
        static METADATA: Lazy<Metadata> = Lazy::new(Metadata::new);

        &METADATA
    }
}
