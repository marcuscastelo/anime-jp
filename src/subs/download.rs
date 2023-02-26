use std::path::{Path, PathBuf};

use chrono::Utc;
use rand::Rng;

use crate::core::download::downloader::{
    Destination, FileDownloader, FileDownloaderError, StringDownloaderError, Uri,
};
use crate::core::download::prelude::*;
use crate::core::indexer::Indexer;

use error_stack::{IntoReport, Result, ResultExt};

const DEFAULT_FOLDER: &str = "subs";
const DEFAULT_EXTENSION: &str = "srt";
pub struct AnimeSubsDownloader {
    inner_downloader: Box<dyn StringDownloader>,
    default_folder: String,
}

impl AnimeSubsDownloader {
    pub fn new() -> Self {
        AnimeSubsDownloader {
            inner_downloader: Box::new(ReqwestDownloader::new()),
            default_folder: DEFAULT_FOLDER.to_string(),
        }
    }

    fn create_file_path(
        &self,
        destination: &Destination,
        file_basename_hint: Option<&str>,
    ) -> PathBuf {
        let folder_name = match destination {
            Destination::GivenFolderGivenFileBasename(folder, _) => folder,
            Destination::GivenFolderGuessFileBasename(folder) => folder,
            Destination::DefaultFolderGivenFileBasename(_) => &self.default_folder,
            Destination::Default => &self.default_folder,
        };

        let file_basename = match destination {
            Destination::GivenFolderGivenFileBasename(_, ref file_basename) => {
                Some(file_basename.as_str())
            }
            Destination::GivenFolderGuessFileBasename(_) => file_basename_hint,
            Destination::DefaultFolderGivenFileBasename(ref file_basename) => {
                Some(file_basename.as_str())
            }
            Destination::Default => file_basename_hint,
        };

        let file_basename = match file_basename {
            Some(file_basename) => file_basename.to_owned(),
            None => generate_random_file_basename(),
        };

        let file_name = format!("{}.{}", file_basename, DEFAULT_EXTENSION);
        let file_path = format!("{}/{}", folder_name, file_name);
        PathBuf::from(file_path)
    }

    fn save(&self, content: &String, path: &Path) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            // Create the folder if it doesn't exist
            std::fs::create_dir_all(parent)
                .into_report()
                .attach_printable_lazy(|| {
                    format!(
                        "Failed to create folders for path: '{}', tried to create: '{}'",
                        path.to_str().unwrap_or("invalid path"),
                        parent.to_str().unwrap_or("invalid path")
                    )
                })?;
        }

        std::fs::write(path, content)
            .into_report()
            .attach_printable_lazy(|| {
                format!(
                    "Failed to save file to path: '{}'",
                    path.to_str().unwrap_or("invalid path")
                )
            }) // TODO: move to new module
    }
}

impl StringDownloader for AnimeSubsDownloader {
    fn download_uri(&self, uri: &Uri) -> Result<String, StringDownloaderError> {
        self.inner_downloader.download_uri(uri)
    }

    fn download_indexer(&self, indexer: &Indexer) -> Result<String, StringDownloaderError> {
        self.inner_downloader.download_indexer(indexer)
    }

    fn download_indexers(
        &self,
        indexers: &[Indexer],
    ) -> Result<Vec<String>, StringDownloaderError> {
        log::debug!("Downloading from indexers: {:?}", indexers);
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

impl FileDownloader for AnimeSubsDownloader {
    // TODO: handle .zip and other formats
    fn download_uri_to_file(
        &self,
        uri: &Uri,
        dest: &Destination,
    ) -> Result<(), FileDownloaderError> {
        let file_path = self.create_file_path(&dest, None);

        if file_path.exists() {
            log::debug!(
                "File already exists, skipping download: '{}'",
                file_path.to_str().unwrap_or("invalid path")
            );
            return Ok(());
        }

        let content = self
            .download_uri(uri)
            .attach_printable("Failed to download String from URI")
            .change_context(FileDownloaderError)?;

        self.save(&content, &file_path)
            .attach_printable("Failed to save file to path")
            .change_context(FileDownloaderError)
    }

    fn download_indexer_to_file(
        &self,
        indexer: &Indexer,
        dest: &Destination,
    ) -> Result<(), FileDownloaderError> {
        let file_path = self.create_file_path(&dest, Some(indexer.name()));

        if file_path.exists() {
            log::debug!(
                "File already exists, skipping download: '{}'",
                file_path.to_str().unwrap_or("invalid path")
            );
            return Ok(());
        }

        let content = self
            .download_indexer(indexer)
            .attach_printable_lazy(|| {
                format!("Failed to download String from Indexer: {:?}", indexer)
            })
            .change_context(FileDownloaderError)?;

        self.save(&content, &file_path)
            .attach_printable("Failed to save file to path")
            .change_context(FileDownloaderError)
    }

    fn download_indexers_to_file(
        &self,
        indexers: &[Indexer],
        dest: &Destination,
    ) -> Result<(), FileDownloaderError> {
        log::debug!("Downloading subtitles from indexers: {:?}", indexers);
        let content = self
            .download_indexers(indexers)
            .attach_printable_lazy(|| {
                format!("Failed to download Strings from Indexers: {:?}", indexers)
            })
            .change_context(FileDownloaderError)?;

        log::debug!("Saving downloaded subtitles to files");
        // TODO: async code (both download and save)
        for (indexer, content) in indexers.iter().zip(content.iter()) {
            let file_path = self.create_file_path(&dest, Some(indexer.name()));

            if file_path.exists() {
                log::debug!(
                    "File already exists, skipping save: '{}'", //TODO: also skip download (if possible)
                    file_path.to_str().unwrap_or("invalid path")
                );
                continue;
            }

            self.save(&content, &file_path)
                .attach_printable("Failed to save file to path")
                .change_context(FileDownloaderError)?;
        }

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_file_basename() {
        let file_name = generate_random_file_basename();
        assert_eq!(file_name.len(), 26);
    }

    #[test]
    fn test_save() {
        let content = "Hello world".to_string();
        let dest = Destination::Default;
        let file_basename_hint = Some("hello");
        let subs_downloader = AnimeSubsDownloader::new();
        let file_path = subs_downloader.create_file_path(&dest, file_basename_hint);
        subs_downloader.save(&content, &file_path);
        //TODO: assert file exists (or use a mock)
    }

    #[test]
    fn test_save_with_given_folder() {
        let content = "Hello world".to_string();
        let dest =
            Destination::GivenFolderGivenFileBasename("test".to_string(), "hello".to_string());
        let file_basename_hint = Some("hello");
        let subs_downloader = AnimeSubsDownloader::new();
        let file_path = subs_downloader.create_file_path(&dest, file_basename_hint);
        subs_downloader.save(&content, &file_path);
        //TODO: assert file exists (or use a mock)
    }

    #[test]
    fn test_save_with_given_folder_and_no_file_basename_hint() {
        let content = "Hello world".to_string();
        let dest = Destination::GivenFolderGuessFileBasename("test".to_string());
        let file_basename_hint = None;
        let subs_downloader = AnimeSubsDownloader::new();
        let file_path = subs_downloader.create_file_path(&dest, file_basename_hint);
        subs_downloader.save(&content, &file_path);
        //TODO: assert file exists (or use a mock)
    }
}
