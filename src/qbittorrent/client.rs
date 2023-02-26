use std::error::Error;

use crate::core::download::downloader::Destination;
use crate::core::indexer::Indexer;
use tokio::runtime::Runtime;

use super::api;
use super::api::torrents::TorrentList;

use error_stack::{Result, ResultExt};

pub struct QBitTorrentClient {
    api: api::QBitTorrentApi,
}

#[derive(Debug)]
pub struct QBitTorrentClientError;

impl std::fmt::Display for QBitTorrentClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Problem in QBitTorrent operation")
    }
}

impl Error for QBitTorrentClientError {}

impl QBitTorrentClient {
    pub fn new() -> Self {
        QBitTorrentClient {
            api: api::QBitTorrentApi::new(),
        }
    }

    pub fn wait_for_completion<'a>(&self, on_update: impl Fn(&TorrentList) + Send + Sync + 'a) -> Result<(), QBitTorrentClientError>{
        let has_unfinished_torrents =
            |torrents: TorrentList| torrents.0.iter().any(|t| !t.finished());

        let rt = Runtime::new().unwrap();

        rt.block_on(async move {
            loop {
                log::debug!("Waiting for torrents to finish");

                let info = self.api
                    .info()
                    .await
                    .attach_printable("Failed to get torrent info")
                    .change_context(QBitTorrentClientError)?;

                log::trace!("Torrent info: {:?}", info);

                on_update(&info);

                if !has_unfinished_torrents(info) {
                    break;
                }

                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }

            Ok(()) as Result<(), QBitTorrentClientError>
        })
    }

    pub async fn add_uri(&self, uri: String, _dest: Destination) -> Result<(), QBitTorrentClientError> {
        self.api.add(uri.as_str())
            .await
            .attach_printable_lazy(|| format!("Failed to add torrent with uri: {}", uri))
            .change_context(QBitTorrentClientError)?;

        Ok(())
    }
}

#[test]
fn test_download_uri_to_file() {
    // let qbt = QBitTorrentDownloader::new();
    // let uri = "magnet:?xt=urn:btih:4936206e05d1bb04084f50032d6b3704f271eff7&dn=%5BOhys-Raws%5D%20Bougyoryoku%202%20-%2006%20%28AT-X%201280x720%20x264%20AAC%29.mp4&tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce";
    // qbt.download_uri_to_file(uri, &Destination::Default).unwrap();
}
