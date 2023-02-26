use crate::core::indexer::Indexer;

pub type Uri = str;

use error_stack::Result;
use std::error::Error;

#[derive(Debug)]
pub struct StringDownloaderError;

impl std::fmt::Display for StringDownloaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to download to string")
    }
}

impl Error for StringDownloaderError {}

pub trait StringDownloader {
    fn download_uri(&self, uri: &Uri) -> Result<String, StringDownloaderError>;
    fn download_indexer(&self, indexer: &Indexer) -> Result<String, StringDownloaderError>;

    #[deprecated(note = "Use download_indexer in a loop instead")]
    fn download_indexers(&self, indexers: &[Indexer]) -> Result<Vec<String>, StringDownloaderError>;
}

pub enum Destination {
    GivenFolderGivenFileBasename(String /* folder */, String /* file basename */),
    GivenFolderGuessFileBasename(String /* folder */),
    DefaultFolderGivenFileBasename(String /* file basename */),
    Default,
}

#[derive(Debug)]
pub struct FileDownloaderError;

impl std::fmt::Display for FileDownloaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to download to file")
    }
}

impl Error for FileDownloaderError {}

pub trait FileDownloader {
    fn download_uri_to_file(
        &self,
        uri: &Uri,
        dest: &Destination,
    ) -> Result<(), FileDownloaderError>;
    fn download_indexer_to_file(
        &self,
        indexer: &Indexer,
        dest: &Destination,
    ) -> Result<(), FileDownloaderError>;

    #[deprecated(note = "Use download_indexer_to_file in a loop instead")]
    fn download_indexers_to_file(
        &self,
        indexers: &[Indexer],
        dest: &Destination,
    ) -> Result<(), FileDownloaderError>;
}
