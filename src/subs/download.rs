use std::error::Error;

use chrono::Utc;
use rand::Rng;

use crate::core::download::downloader::{Destination, FileDownloader, Uri};
use crate::core::download::prelude::*;
use crate::core::indexer::Indexer;
use crate::subs;

const DEFAULT_FOLDER: &str = "subs";
const DEFAULT_EXTENSION: &str = "srt";

pub struct SubsDownloader {
    inner_downloader: ReqwestDownloader,
    default_folder: String,
}

impl SubsDownloader {
    pub fn new() -> Self {
        SubsDownloader {
            inner_downloader: ReqwestDownloader::new(),
            default_folder: DEFAULT_FOLDER.to_string(),
        }
    }

    pub fn with_default_folder(custom_default_folder: String) -> Self {
        SubsDownloader {
            inner_downloader: ReqwestDownloader::new(),
            default_folder: custom_default_folder,
        }
    }

    fn save(
        &self,
        content: &String,
        destination: &Destination,
        file_basename_hint: Option<&String>,
    ) {
        let folder_name = match destination {
            Destination::GivenFolderGivenFileBasename(folder, _) => folder,
            Destination::GivenFolderGuessFileBasename(folder) => folder,
            Destination::DefaultFolderGivenFileBasename(_) => &self.default_folder,
            Destination::Default => &self.default_folder,
        };

        let file_basename = match destination {
            Destination::GivenFolderGivenFileBasename(_, ref file_basename) => Some(file_basename),
            Destination::GivenFolderGuessFileBasename(_) => file_basename_hint,
            Destination::DefaultFolderGivenFileBasename(ref file_basename) => Some(file_basename),
            Destination::Default => file_basename_hint,
        };

        let file_basename = match file_basename {
            Some(file_basename) => file_basename.to_owned(),
            None => generate_random_file_basename(),
        };

        let file_name = format!("{}.{}", file_basename, DEFAULT_EXTENSION);
        let file_path = format!("{}/{}", folder_name, file_name);

        // Create the folder if it doesn't exist
        std::fs::create_dir_all(folder_name).unwrap(); // TODO: move to new module
        std::fs::write(file_path, content).unwrap(); // TODO: move to new module
    }
}

impl RawDownloader for SubsDownloader {
    fn download_uri(&self, uri: &Uri) -> Result<String, Box<dyn Error>> {
        self.inner_downloader.download_uri(uri)
    }

    fn download_indexer(&self, indexer: &Indexer) -> Result<String, Box<dyn Error>> {
        self.inner_downloader.download_indexer(indexer)
    }

    fn download_indexers(&self, indexers: &[Indexer]) -> Result<Vec<String>, Box<dyn Error>> {
        self.inner_downloader.download_indexers(indexers)
    }
}

// Generate a random file name in the following format: "20230213-123123.ext"
fn generate_random_file_basename() -> String {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen();
    let random_number_string = random_number.to_string();
    let now = Utc::now();
    let now_string = now.format("%Y%m%d-%H%M%S").to_string();
    let file_name = format!("{}-{}", now_string, random_number_string);
    file_name
}

impl FileDownloader for SubsDownloader {
    // TODO: handle .zip and other formats
    fn download_uri_to_file(&self, uri: &Uri, dest: &Destination) -> Result<(), Box<dyn Error>> {
        let content = self.download_uri(uri)?;
        self.save(&content, &dest, None);
        return Ok(());
    }

    fn download_indexer_to_file(
        &self,
        indexer: &Indexer,
        dest: &Destination,
    ) -> Result<(), Box<dyn Error>> {
        let content = self.download_indexer(indexer)?;
        self.save(&content, &dest, Some(indexer.name()));
        return Ok(());
    }

    fn download_indexers_to_file(
        &self,
        indexers: &[Indexer],
        dest: &Destination,
    ) -> Result<(), Box<dyn Error>> {
        let content = self.download_indexers(indexers)?;
        // TODO: async code (both download and save)
        for (indexer, content) in indexers.iter().zip(content.iter()) {
            self.save(&content, &dest, Some(indexer.name()));
        }
        return Ok(());
    }
}
