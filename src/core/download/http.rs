use std::error::Error;

use super::downloader::Downloader;

pub struct ReqwestDownloader;
impl Downloader for ReqwestDownloader {
    fn new() -> Self {
        ReqwestDownloader
    }

    fn download(&self, uri: &str) -> Result<String, Box<dyn Error>> {
        let response = reqwest::blocking::get(uri)?;
        let response_text = response.text()?;
        return Ok(response_text);
    }
}