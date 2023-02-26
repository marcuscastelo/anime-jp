use crate::core::download::downloader::{Destination, FileDownloader, FileDownloaderError};
use crate::core::indexer::Indexer;
use tokio::runtime::Runtime;

use super::api;
use super::api::torrents::TorrentList;

use error_stack::{Result, ResultExt};

pub struct QBitTorrentDownloader {
    runtime: Runtime,
}

impl QBitTorrentDownloader {
    pub fn new() -> Self {
        QBitTorrentDownloader {
            runtime: Runtime::new().expect("Failed to create a new runtime"),
        }
    }
}

impl FileDownloader for QBitTorrentDownloader {
    fn download_uri_to_file(
        &self,
        uri: &str,
        _dest: &Destination,
    ) -> Result<(), FileDownloaderError> {
        let future = async {
            let api = api::QBitTorrentApi::new();
            api.add(uri)
                .await
                .attach_printable_lazy(|| format!("Failed to add torrent with uri: {}", uri))
                .change_context(FileDownloaderError)?;

            let has_unfinished_torrents =
                |torrents: TorrentList| torrents.0.iter().any(|t| !t.finished());

            loop {
                log::info!("Waiting for torrents to finish");
                let info = api.info()
                    .await
                    .attach_printable("Failed to get torrent info")
                    .change_context(FileDownloaderError)?;

                if !has_unfinished_torrents(info) {
                    break;
                }

                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }

            Ok(()) as Result<(), FileDownloaderError>
        };

        self.runtime.block_on(future)
    }

    fn download_indexer_to_file(
        &self,
        _indexer: &Indexer,
        _dest: &Destination,
    ) -> Result<(), FileDownloaderError> {
        todo!();
    }

    fn download_indexers_to_file(
        &self,
        _indexers: &[Indexer],
        _dest: &Destination,
    ) -> Result<(), FileDownloaderError> {
        todo!();
    }
}

#[test]
fn test_download_uri_to_file() {
    // let qbt = QBitTorrentDownloader::new();
    // let uri = "magnet:?xt=urn:btih:4936206e05d1bb04084f50032d6b3704f271eff7&dn=%5BOhys-Raws%5D%20Bougyoryoku%202%20-%2006%20%28AT-X%201280x720%20x264%20AAC%29.mp4&tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce";
    // qbt.download_uri_to_file(uri, &Destination::Default).unwrap();
}
