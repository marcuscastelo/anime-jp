use std::error::Error;

pub trait Downloader {
    fn new() -> Self;
    fn download(&self, uri: &str) -> Result<String, Box<dyn Error>>;
}