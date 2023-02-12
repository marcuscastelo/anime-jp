use std::error::Error;

use crate::core::indexer::Indexer;

pub trait Downloader {
    fn new() -> Self;
    fn download(&self, uri: &str) -> Result<String, Box<dyn Error>>;

    fn download_indexer(&self, indexer: &Indexer) -> Result<String, Box<dyn Error>> {
        self.download(&indexer.url)
    }

    fn download_indexers(&self, indexers: &[Indexer]) -> Result<Vec<String>, Box<dyn Error>> {
        indexers
            .iter()
            .map(|indexer| self.download_indexer(indexer))
            .collect()
    }
}