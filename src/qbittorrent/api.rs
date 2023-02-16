use reqwest::{Client, header::{HeaderMap, CONTENT_TYPE}};
use tokio::runtime::Runtime;

pub async fn add_torrent(uri: &str) -> Result<String, Box<dyn std::error::Error>>  {
    let client = Client::new();

    // Monta a URL da API do QBitTorrent para adicionar um novo torrent
    let api_url = format!("http://127.0.0.1:8080/api/v2/torrents/add");

    // Define os parâmetros da solicitação POST
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
    let mut params = std::collections::HashMap::new();
    params.insert("urls", uri);
    params.insert("savepath", ".");

    // Envia a solicitação POST para a API do QBitTorrent
    let res = client.post(&api_url)
        .headers(headers)
        .form(&params)
        .basic_auth("username", Some("password"))
        .send()
        .await?;

    // Lê a resposta da API do QBitTorrent
    let body = res.text().await?;
    println!("{}", body);

    Ok(body)
}


pub async fn is_torrent_completed(hash: &str) -> Result<bool, Box<dyn std::error::Error>> {
    todo!("Implementar")
}

#[test]
fn test_add_torrent() {
    let runtime = Runtime::new().unwrap();
    let uri = "magnet:?xt=urn:btih:4936206e05d1bb04084f50032d6b3704f271eff7&dn=%5BOhys-Raws%5D%20Bougyoryoku%202%20-%2006%20%28AT-X%201280x720%20x264%20AAC%29.mp4&tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce";
    let body = runtime.block_on(add_torrent(uri)).unwrap();
    assert_eq!(body, "Ok.");
}