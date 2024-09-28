use std::error::Error;
use bytes::Bytes;
use rand::Rng;
use reqwest::{header, Client};
use scraper::{Html, Selector};
use thiserror::Error as ThisError;
use crate::downloader::DownloaderError::{DownloadUrlNotFound, HtmlParseError};
use crate::utils;

pub struct Downloader {
    url :String,
    client: Client,
    download_url :Option<String>,
    version :Option<String>
}

#[derive(Debug, ThisError)]
enum DownloaderError {
    #[error("Parsing of download URL failed.")]
    HtmlParseError,
    #[error("Download link not fetched.")]
    DownloadUrlNotFound
}

impl Downloader {
    pub fn new(url :&str) -> Result<Self, Box<dyn Error>> {
        let mut headers = header::HeaderMap::new();
        headers.insert("Accept-Encoding", header::HeaderValue::from_str("identity").unwrap());
        headers.insert("Accept-Language", header::HeaderValue::from_str("en").unwrap());

        let mut r = rand::thread_rng();
        let random_int = r.gen::<i64>();
        let user_agent = format!("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.33 (KHTML, like Gecko) Chrome/90.0.{}.212 Safari/537.33", random_int);

        let client = Client::builder()
            .user_agent(user_agent)
            .default_headers(headers)
            .build()?;

        Ok(Self {
            url: url.to_string(),
            client,
            download_url: None,
            version: None
        })
    }

    pub async fn fetch(&mut self) -> Result<(), Box<dyn Error>>{
        let resp = self.client
            .get(&self.url)
            .send()
            .await?;
        let text = resp.text().await?;

        if let Some(text) = self.grep(text, "bin-linux") {
            let url = self.remove_html(text).unwrap();
            self.download_url = Some(url.clone());
            self.version = Some(utils::parse_version(url));
            Ok(())
        } else {
            Err(HtmlParseError.into())
        }
    }

    pub fn is_update(&self, version :String) -> bool {
        !self.version.clone().is_some_and(|a| a == version)
    }

    pub fn get_online_version(&self) -> Option<String> {
        self.version.clone()
    }

    pub async fn download(&mut self) -> Result<Bytes, Box<dyn Error>> {
        if let Some(url) = self.download_url.clone() {
            let response = reqwest::get(url).await?;
            let bytes = response.bytes().await?;
            Ok(bytes)
        } else {
            Err(DownloadUrlNotFound.into())
        }
    }


    fn grep(&self, content :String, pattern: &str) -> Option<String> {
        for line in content.lines() {
            if line.contains(pattern) {
                return Some(line.to_string());
            }
        }
        None
    }

    fn remove_html(&self, original :String) -> Option<String> {
        let html = Html::parse_fragment(original.as_str());
        let a_html = Selector::parse("a").unwrap();
        if let Some(a) = html.select(&a_html).next() {
            a.attr("href").map(|s| s.to_string())
        } else {
            None
        }
    }
}