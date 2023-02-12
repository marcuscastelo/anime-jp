use crate::core::download::prelude::*;


pub struct SubsDownloader {
    inner_downloader: ReqwestDownloader,
}
impl Downloader for SubsDownloader {
    fn new() -> Self {
        SubsDownloader {
            inner_downloader: ReqwestDownloader::new(),
        }
    }

    fn download(&self, uri: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.inner_downloader.download(uri)
    }
}
