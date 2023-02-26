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
        log::debug!("Downloading from uri: {}", uri);
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

        log::debug!("Download complete from uri: {}", uri);
        log::trace!("Downloaded text: {}", response_text);

        return Ok(response_text);
    }

    fn download_indexer(&self, indexer: &Indexer) -> Result<String, StringDownloaderError> {
        let uri = &indexer.uri();

        log::debug!("Downloading from indexer: {:?}", indexer);
        let result = self.download_uri(uri);
        log::debug!("Download complete from indexer: {:?}", indexer);

        result
    }

    fn download_indexers(
        &self,
        indexers: &[Indexer],
    ) -> Result<Vec<String>, StringDownloaderError> {
        log::debug!("Downloading from indexers: {:?}", indexers);
        let mut results = Vec::new();
        for indexer in indexers {
            log::debug!("Downloading from indexer: {:?}", indexer);
            let result = self.download_indexer(indexer)?;
            results.push(result);
        }
        return Ok(results);
    }
}
