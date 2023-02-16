use std::error::Error;

use crate::core::indexer::Indexer;

pub type Uri = str;

pub trait RawDownloader {
    fn download_uri(&self, uri: &Uri) -> Result<String, Box<dyn Error>>;
    fn download_indexer(&self, indexer: &Indexer) -> Result<String, Box<dyn Error>>;
    fn download_indexers(&self, indexers: &[Indexer]) -> Result<Vec<String>, Box<dyn Error>>;
}

#[allow(dead_code)]
pub enum Destination {
    GivenFolderGivenFileBasename(String /* folder */, String /* file basename */),
    GivenFolderGuessFileBasename(String /* folder */),
    DefaultFolderGivenFileBasename(String /* file basename */),
    Default,
}

pub trait FileDownloader {
    fn download_uri_to_file(&self, uri: &Uri, dest: &Destination) -> Result<(), Box<dyn Error>>;
    fn download_indexer_to_file(
        &self,
        indexer: &Indexer,
        dest: &Destination,
    ) -> Result<(), Box<dyn Error>>;
    fn download_indexers_to_file(
        &self,
        indexers: &[Indexer],
        dest: &Destination,
    ) -> Result<(), Box<dyn Error>>;
}
