use crate::core::download::downloader::{FileDownloader, Uri, Destination, FileDownloaderError};
use crate::core::download::prelude::*;
use crate::core::indexer::Indexer;
use crate::qbittorrent::downloader::{QBitTorrentDownloader};

use error_stack::{IntoReport, Result, ResultExt};

//TODO: folder to save the files should be configurable

pub struct AnimeRawDownloader {
    inner_downloader: Box<dyn FileDownloader>,
    // default_folder: String,
}

impl AnimeRawDownloader {
    pub fn new() -> Self {
        Self {
            inner_downloader: Box::new(QBitTorrentDownloader::new()),
        }
    }
}

impl FileDownloader for AnimeRawDownloader {
    fn download_uri_to_file(
        &self,
        uri: &Uri,
        dest: &Destination,
    ) -> Result<(), FileDownloaderError> {
        self.inner_downloader
            .download_uri_to_file(uri, dest)
    }

    fn download_indexer_to_file(
        &self,
        indexer: &Indexer,
        dest: &Destination,
    ) -> Result<(), FileDownloaderError> {
        self.inner_downloader
            .download_indexer_to_file(indexer, dest)
    }

    fn download_indexers_to_file(
        &self,
        indexers: &[Indexer],
        dest: &Destination,
    ) -> Result<(), FileDownloaderError> {
        self.inner_downloader
            .download_indexers_to_file(indexers, dest)
    }
}