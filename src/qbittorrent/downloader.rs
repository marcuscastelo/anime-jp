use crate::core::download::downloader::{Destination, FileDownloader};
use crate::core::indexer::Indexer;
use std::error::Error;
use tokio::runtime::Runtime;

use super::api;
use super::api::torrents::TorrentList;

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
    fn download_uri_to_file(&self, uri: &str, dest: &Destination) -> Result<(), Box<dyn Error>> {
        let future = async {
            api::torrents::add(uri).await?;
            let has_unfinished_torrents =
                |torrents: TorrentList| torrents.0.iter().any(|t| !t.finished());

            while has_unfinished_torrents(api::torrents::info().await?) {
                println!("Waiting for torrent to finish...");
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }

            Ok(()) as Result<(), Box<dyn Error>>
        };

        self.runtime.block_on(future)?;
        Ok(())
    }

    fn download_indexer_to_file(
        &self,
        indexer: &Indexer,
        dest: &Destination,
    ) -> Result<(), Box<dyn Error>> {
        todo!();
    }

    fn download_indexers_to_file(
        &self,
        indexers: &[Indexer],
        dest: &Destination,
    ) -> Result<(), Box<dyn Error>> {
        todo!();
    }
}

#[test]
fn test() {
    let qbt = QBitTorrentDownloader::new();
    let uri = "magnet:?xt=urn:btih:4936206e05d1bb04084f50032d6b3704f271eff7&dn=%5BOhys-Raws%5D%20Bougyoryoku%202%20-%2006%20%28AT-X%201280x720%20x264%20AAC%29.mp4&tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce";
    let res = qbt.download_uri_to_file(uri, &Destination::Default);
    assert!(res.is_ok());
}
