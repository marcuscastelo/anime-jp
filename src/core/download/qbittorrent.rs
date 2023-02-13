use std::error::Error;

use crate::core::indexer::Indexer;

use super::downloader::RawDownloader;

pub struct QBitTorrentDownloader;

impl QBitTorrentDownloader {
    pub fn new() -> Self {
        QBitTorrentDownloader
    }
}

impl RawDownloader for QBitTorrentDownloader {
    fn download_uri(&self, uri: &str) -> Result<String, Box<dyn Error>> {
        todo!();
    }
    
    fn download_indexer(&self, indexer: &Indexer) -> Result<String, Box<dyn Error>> {
        todo!();
    }
    
    fn download_indexers(&self, indexers: &[Indexer]) -> Result<Vec<String>, Box<dyn Error>> {
        todo!();
    }
}