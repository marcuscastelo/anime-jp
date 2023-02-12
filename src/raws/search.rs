use std::collections::LinkedList;

use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

macro_rules! create_anime_raw_query_url {
    ($anime_name: expr) => {
        format!(
            "https://nyaa.si/?f=0&c=1_4&q={}&s=seeders&o=desc",
            $anime_name
        )
    };
}

lazy_static! {
    static ref MAGNET_REGEX: Regex = match Regex::new(
        r#"(?m)href="(/view/[^"]+?)" title="([^"]+?)"(?:.|[\n\r ])+?(magnet:[^"]+)(?:.|[\n\r ])+?text-center(?:.|[\n\r ])+?text-center(?:.|[\n\r ])+?text-center">(\d+)"#,
    ) {
        Ok(regex) => regex,
        Err(error) => panic!("Failed to create regex for magnet link, error: {}", error),
    };
}

#[derive(Debug)]
pub struct AnimeRawSearchResult {
    pub anime_name: String,
    pub anime_raw_magnet: String,
}

#[derive(Error, Debug)]
pub enum ResponseParsingError {
    #[error("Regex capture count mismatch: expected {0}, actual {1}")]
    RegexCaptureCountMismatch(usize /* expected */, usize /* actual */),
}

type AnimeRawSearchResults = LinkedList<AnimeRawSearchResult>;

fn process_http_response(
    response_text: String,
) -> Result<AnimeRawSearchResults, ResponseParsingError> {
    let mut results = LinkedList::new();

    for cap in MAGNET_REGEX.captures_iter(&response_text) {
        if cap.len() != 5 {
            return Err(ResponseParsingError::RegexCaptureCountMismatch(
                5,
                cap.len(),
            ));
        }

        let anime_name = cap[2].to_string();
        let anime_raw_magnet = cap[3].to_string();
        results.push_back(AnimeRawSearchResult {
            anime_name,
            anime_raw_magnet,
        });
    }

    return Ok(results);
}

pub fn search_anime_raw(anime_name: &str) -> Result<AnimeRawSearchResults, ResponseParsingError> {
    println!("Searching for anime: {}", anime_name);

    let response_text = reqwest::blocking::get(create_anime_raw_query_url!(anime_name))
        .expect("Failed to get response from nyaa.si")
        .text()
        .expect("Failed to get text from response");

    return process_http_response(response_text);
}

#[test]
fn test_anime_name() {
    let anime_name = String::from("One Piece");
    let result = search_anime_raw(anime_name.as_str())
        .expect("Failed to search anime")
        .pop_front()
        .expect("No results found");

    assert!(result.anime_name.contains("One Piece"));
}

#[test]
fn test_anime_raw_magnet() {
    let anime_name = String::from("One Piece");
    let result = search_anime_raw(anime_name.as_str())
        .expect("Failed to search anime")
        .pop_front()
        .expect("No results found");
    assert!(result.anime_raw_magnet.starts_with("magnet:?xt=urn:btih:"));
}

#[test]
fn test_response() {
    let html = r#"
<tr class="default">
    <td>
        <a href="/?c=1_4" title="Anime - Raw">
            <img src="/static/img/icons/nyaa/1_4.png" alt="Anime - Raw" class="category-icon">
        </a>
    </td>
    <td colspan="2">
        <a href="/view/1636619" title="[Fumi-Raws] (One Piece (1051) - (フジテレビ 1920x1080).mkv">[Fumi-Raws] (One Piece (1051) - (フジテレビ 1920x1080).mkv</a>
    </td>
    <td class="text-center">
        <a href="/download/1636619.torrent"><i class="fa fa-fw fa-download"></i></a>
        <a href="magnet:?xt=urn:btih:568807a73ecd33fff3ac19f47805f0940cbdb9ac&amp;dn=%5BFumi-Raws%5D%20%28One%20Piece%20%281051%29%20-%20%28%E3%83%95%E3%82%B8%E3%83%86%E3%83%AC%E3%83%93%201920x1080%29.mkv&amp;tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&amp;tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&amp;tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&amp;tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&amp;tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce"><i class="fa fa-fw fa-magnet"></i></a>
    </td>
    <td class="text-center">1.6 GiB</td>
    <td class="text-center" data-timestamp="1676165612" title="13 minutes 30 seconds ago">2023-02-11 22:33</td>

    <td class="text-center">3</td>
    <td class="text-center">13</td>
    <td class="text-assert_matches!()center">1</td>
</tr>"#;
    let mut result =
        process_http_response(html.to_string()).expect("Failed to parse response text");

    let result = result.pop_front().expect("No results found");
    assert_eq!(
        result.anime_name,
        "[Fumi-Raws] (One Piece (1051) - (フジテレビ 1920x1080).mkv"
    );
    assert_eq!(
        result.anime_raw_magnet,
        "magnet:?xt=urn:btih:568807a73ecd33fff3ac19f47805f0940cbdb9ac&amp;dn=%5BFumi-Raws%5D%20%28One%20Piece%20%281051%29%20-%20%28%E3%83%95%E3%82%B8%E3%83%86%E3%83%AC%E3%83%93%201920x1080%29.mkv&amp;tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&amp;tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&amp;tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&amp;tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&amp;tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce"
    );
}
