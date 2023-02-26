use reqwest::{
    header::{HeaderMap, InvalidHeaderValue, CONTENT_TYPE},
    Client, Method,
};

use serde::{Deserialize, Serialize};

use error_stack::{IntoReport, Report, Result, ResultExt};
use std::{collections::HashMap, error::Error};

use super::{QBitTorrentApi};

//--------------------- Types ---------------------

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

//--------------------- Functions ---------------------

//TODO: mover para um arquivo de configuração
const API_URL: &str = "http://127.0.0.1:8080/api/v2/";

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

//--------------------- API ---------------------

fn create_form_content_type() -> Result<reqwest::header::HeaderValue, InvalidHeaderValue> {
    let content_type = "application/x-www-form-urlencoded"
        .parse()
        .into_report()
        .attach_printable("Failed to create content type header")?;

    Ok(content_type)
}

fn create_defaulted_headers() -> Result<HeaderMap, InvalidHeaderValue> {
    let mut headers = HeaderMap::new();

    let content_type = create_form_content_type()?;

    headers.insert(CONTENT_TYPE, content_type);

    Ok(headers)
}

fn create_request(
    http_client: &Client,
    api_url: &str,
    method: Method,
    headers: HeaderMap,
    params: &std::collections::HashMap<&str, &str>,
) -> reqwest::RequestBuilder {
    http_client
        .request(method, api_url)
        .headers(headers)
        .form(params)
        .basic_auth("username", Some("password"))
}

async fn get_body_string(res: reqwest::Response) -> Result<String, reqwest::Error> {
    let status = res.status();
    let content_len = res.content_length();
    let body = res.text().await.into_report().attach_printable_lazy(|| {
        format!(
            "Failed to get body as text, status: {}, content len: {:?}",
            status, content_len
        )
    })?;

    Ok(body)
}

async fn get_body_json<T>(res: reqwest::Response) -> Result<T, reqwest::Error>
where
    T: for<'de> Deserialize<'de>,
{
    let status = res.status();
    let content_len = res.content_length();
    let body = res.json().await.into_report().attach_printable_lazy(|| {
        format!(
            "Failed to get body as json, status: {}, content len: {:?}",
            status, content_len
        )
    })?;

    Ok(body)
}

impl QBitTorrentApi {
    //TODO: múltiplos torrents
    pub async fn add(&self, uri: &str) -> Result<String, TorrentAddError> {
        // Monta a URL da API do QBitTorrent para adicionar um novo torrent
        let api_url = format_endpoint!("torrents/add");

        // Define os parâmetros da solicitação POST
        let headers = create_defaulted_headers()
            .attach_printable("Failed creating default headers")
            .change_context(TorrentAddError)?;

        //TODO: Criar um tipo para os parâmetros
        let params_map = std::collections::HashMap::from([("urls", uri), ("savepath", ".")]);

        let req = create_request(
            &self.http_client,
            &api_url,
            Method::POST,
            headers,
            &params_map,
        );

        // Envia a solicitação POST para a API do QBitTorrent
        let res = req
            .send()
            .await
            .into_report()
            .attach_printable("Failed to send request")
            .change_context(TorrentAddError)?;

        if !res.status().is_success() {
            return Err(Report::new(TorrentAddError)
                .attach_printable(format!("Response failed with status: {}", res.status())));
        }

        // Lê a resposta da API do QBitTorrent
        let body = get_body_string(res)
            .await
            .attach_printable("Failed to read response body")
            .change_context(TorrentAddError)?;

        println!("{}", body);

        Ok(body)
    }

    pub async fn info(&self) -> Result<TorrentList, TorrentInfoError> {
        // Monta a URL da API do QBitTorrent para adicionar um novo torrent
        let api_url = format_endpoint!("torrents/info");

        let req = create_request(
            &self.http_client,
            &api_url,
            Method::GET,
            HeaderMap::default(),
            &HashMap::default(),
        );

        // Envia a solicitação POST para a API do QBitTorrent
        let res = req
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

        let body: TorrentList = get_body_json(res)
            .await
            .attach_printable("Failed to read response body")
            .change_context(TorrentInfoError)?;

        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_add_torrent_ok() {
        let api = QBitTorrentApi::new();
        let runtime = Runtime::new().unwrap();
        let uri = "magnet:?xt=urn:btih:4936206e05d1bb04084f50032d6b3704f271eff7&dn=%5BOhys-Raws%5D%20Bougyoryoku%202%20-%2006%20%28AT-X%201280x720%20x264%20AAC%29.mp4&tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce";
        let body = runtime.block_on(api.add(uri)).unwrap();
        assert_eq!(body, "Ok.");
    }

    #[test]
    fn test_add_torrent_wrong_torrent() {
        let api = QBitTorrentApi::new();
        let runtime = Runtime::new().unwrap();
        let uri = "__BROKEN__MAGNET__LINK__";
        let body = runtime.block_on(api.add(uri)).unwrap();
        assert_eq!(body, "Fails.");
    }

    #[test]
    fn test_info() {
        let api = QBitTorrentApi::new();
        let runtime = Runtime::new().unwrap();
        let body = runtime.block_on(api.info()).unwrap();
        dbg!(&body);
    }
}
