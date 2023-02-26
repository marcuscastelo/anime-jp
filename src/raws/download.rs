use std::sync::Arc;

use crate::core::download::downloader::{Destination, FileDownloader, FileDownloaderError, Uri};
use crate::core::download::prelude::*;
use crate::core::indexer::Indexer;
use crate::qbittorrent::api::torrents::TorrentList;
use crate::qbittorrent::client::{QBitTorrentClient, QBitTorrentClientError};

use error_stack::{IntoReport, Result, ResultExt};
use tokio::runtime::Runtime;

//TODO: folder to save the files should be configurable

pub struct AnimeRawDownloader {
    inner_downloader: Arc<QBitTorrentClient>,
    runtime: Runtime,
    // default_folder: String,
}

impl AnimeRawDownloader {
    pub fn new() -> Self {
        Self {
            inner_downloader: Arc::new(QBitTorrentClient::new()),
            runtime: Runtime::new().unwrap(),
        }
    }

    pub fn wait_for_completion<'a>(&self, on_update: impl Fn(&TorrentList) + Send + Sync + 'a) -> Result<(), QBitTorrentClientError>{
        self.inner_downloader.wait_for_completion(on_update)
    }
}

impl FileDownloader for AnimeRawDownloader {
    fn download_uri_to_file(
        &self,
        uri: &Uri,
        _dest: &Destination,
    ) -> Result<(), FileDownloaderError> {
        let inner_downloader = self.inner_downloader.clone();
        let uri = uri.to_owned();
        let future = async move {
            inner_downloader.add_uri(uri, Destination::Default).await
        }; 
        self.runtime.spawn(future);
        Ok(())
    }

    fn download_indexer_to_file(
        &self,
        indexer: &Indexer,
        dest: &Destination,
    ) -> Result<(), FileDownloaderError> {
        self.download_uri_to_file(indexer.uri(), dest)
    }

    fn download_indexers_to_file(
        &self,
        _indexers: &[Indexer],
        _dest: &Destination,
    ) -> Result<(), FileDownloaderError> {
        unimplemented!()
    }
}
