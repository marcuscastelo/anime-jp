use reqwest::{
    header::{HeaderMap, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};

use error_stack::{IntoReport, Report, Result, ResultExt};
use std::error::Error;

#[derive(Deserialize, Serialize, Debug)]
pub struct Torrent {
    hash: String,
    name: String,
    size: i64,
    state: String,
}

impl Torrent {
    pub fn finished(&self) -> bool {
        self.state == "uploading"
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TorrentList(pub Vec<Torrent>);

//TODO: mover para um arquivo de configuração
const API_URL: &str = "http://127.0.0.1:8080/api/v2/";

//TODO: mover
macro_rules! format_endpoint {
    ($endpoint:expr) => {
        format!("{}{}", API_URL, $endpoint)
    };
}

#[derive(Debug)]
pub struct TorrentAddError;

impl std::fmt::Display for TorrentAddError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to add torrent")
    }
}

impl Error for TorrentAddError {}

#[derive(Debug)]
pub struct TorrentInfoError;

impl std::fmt::Display for TorrentInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to add torrent")
    }
}

impl Error for TorrentInfoError {}

//TODO: múltiplos torrents
pub async fn add(uri: &str) -> Result<String, TorrentAddError> {
    let client = Client::new();

    // Monta a URL da API do QBitTorrent para adicionar um novo torrent
    let api_url = format_endpoint!("torrents/add");

    // Define os parâmetros da solicitação POST
    let mut headers = HeaderMap::new();

    let content_type = "application/x-www-form-urlencoded"
        .parse()
        .into_report()
        .attach_printable("Failed to create content type header")
        .change_context(TorrentAddError)?;

    headers.insert(CONTENT_TYPE, content_type);

    let mut params = std::collections::HashMap::new();
    params.insert("urls", uri);
    params.insert("savepath", ".");

    let req = client
        .post(&api_url)
        .headers(headers)
        .form(&params)
        .basic_auth("username", Some("password"));

    // Envia a solicitação POST para a API do QBitTorrent
    let res = req
        .send()
        .await
        .into_report()
        .attach_printable("Failed to send request")
        .change_context(TorrentAddError)?;

    // Lê a resposta da API do QBitTorrent
    let body = res
        .text()
        .await
        .into_report()
        .attach_printable("Failed to read response body")
        .change_context(TorrentAddError)?;

    println!("{}", body);

    Ok(body)
}

pub async fn info() -> Result<TorrentList, TorrentInfoError> {
    let client = Client::new();

    // Monta a URL da API do QBitTorrent para adicionar um novo torrent
    let api_url = format_endpoint!("torrents/info");

    // Envia a solicitação POST para a API do QBitTorrent
    let res = client
        .get(&api_url)
        .basic_auth("username", Some("password"))
        .send()
        .await
        .into_report()
        .attach_printable("Failed to send request")
        .change_context(TorrentInfoError)?;

    // Check if the request was successful
    if !res.status().is_success() {
        return Err(Report::new(TorrentInfoError)
            .attach_printable(format!("Response failed with status: {}", res.status())));
    }

    let body: TorrentList = res
        .json::<TorrentList>()
        .await
        .into_report()
        .attach_printable("Failed to read response body")
        .change_context(TorrentInfoError)?;

    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_add_torrent_ok() {
        let runtime = Runtime::new().unwrap();
        let uri = "magnet:?xt=urn:btih:4936206e05d1bb04084f50032d6b3704f271eff7&dn=%5BOhys-Raws%5D%20Bougyoryoku%202%20-%2006%20%28AT-X%201280x720%20x264%20AAC%29.mp4&tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce";
        let body = runtime.block_on(add(uri)).unwrap();
        assert_eq!(body, "Ok.");
    }

    #[test]
    fn test_add_torrent_wrong_torrent() {
        let runtime = Runtime::new().unwrap();
        let uri = "__BROKEN__MAGNET__LINK__";
        let body = runtime.block_on(add(uri)).unwrap();
        assert_eq!(body, "Fails.");
    }

    #[test]
    fn test_info() {
        let runtime = Runtime::new().unwrap();
        let body = runtime.block_on(info()).unwrap();
        dbg!(&body);
    }
}
