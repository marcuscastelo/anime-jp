use reqwest::Client;

pub struct QBitTorrentApi {
    http_client: Client,
}

macro_rules! format_endpoint {
    ($endpoint:expr) => {
        format!("{}{}", API_URL, $endpoint)
    };
}

impl QBitTorrentApi {
    pub fn new() -> Self {
        QBitTorrentApi {
            http_client: Client::new(),
        }
    }
}

pub mod torrents;