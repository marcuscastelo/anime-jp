use std::error::Error;

use crate::core::indexer::Indexer;

use super::downloader::RawDownloader;

pub struct ReqwestDownloader;

impl ReqwestDownloader {
    pub fn new() -> Self {
        ReqwestDownloader
    }
}

impl RawDownloader for ReqwestDownloader {
    fn download_uri(&self, uri: &str) -> Result<String, Box<dyn Error>> {
        let response = reqwest::blocking::get(uri)?;
        let response_text = response.text()?;
        return Ok(response_text);
    }

    fn download_indexer(&self, indexer: &Indexer) -> Result<String, Box<dyn Error>> {
        let uri = &indexer.uri();
        self.download_uri(uri)
    }

    fn download_indexers(&self, indexers: &[Indexer]) -> Result<Vec<String>, Box<dyn Error>> {
        let mut results = Vec::new();
        for indexer in indexers {
            let result = self.download_indexer(indexer)?;
            results.push(result);
        }
        return Ok(results);
    }
}