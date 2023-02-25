use crate::core::indexer::Indexer;

use super::downloader::{StringDownloader, StringDownloaderError};

use error_stack::{IntoReport, Result, ResultExt};
pub struct ReqwestDownloader;
//TODO: use Client instead of blocking::get

impl ReqwestDownloader {
    pub fn new() -> Self {
        ReqwestDownloader
    }
}

impl StringDownloader for ReqwestDownloader {
    fn download_uri(&self, uri: &str) -> Result<String, StringDownloaderError> {
        //TODO: use Client instead of blocking::get
        let response = reqwest::blocking::get(uri)
            .into_report()
            .attach_printable_lazy(|| format!("Failed to download uri: {}", uri))
            .change_context(StringDownloaderError)?;

        let response_text = response
            .text()
            .into_report()
            .attach_printable("Failed to get response text")
            .change_context(StringDownloaderError)?; 

        return Ok(response_text);
    }

    fn download_indexer(&self, indexer: &Indexer) -> Result<String, StringDownloaderError> {
        let uri = &indexer.uri();
        self.download_uri(uri)
    }

    fn download_indexers(
        &self,
        indexers: &[Indexer],
    ) -> Result<Vec<String>, StringDownloaderError> {
        let mut results = Vec::new();
        for indexer in indexers {
            let result = self.download_indexer(indexer)?;
            results.push(result);
        }
        return Ok(results);
    }
}
