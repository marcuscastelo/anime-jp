use std::{error::Error, fs::File, io::Write};

pub type SubsInRAM = Vec<String>;

// Save the subs to disk: each sub is a file
// The file name is the anime name
// The file extension is the sub language
// The file content is the sub content
pub fn save_subs(anime_name: &str, subs: &SubsInRAM) -> Result<(), Box<dyn Error>> {
    // Create the tmp folder if it doesn't exist
    log::trace!("Creating tmp folder if it doesn't exist");
    std::fs::create_dir_all("tmp")?;
    log::trace!("Tmp folder created");
    
    for (i, sub) in subs.iter().enumerate() {
        log::debug!("Saving sub: {}", i);
        let file_name = format!("tmp/{}-{}.srt", anime_name, i);
        log::trace!("Creating file: {}", file_name);
        let mut file = File::create(file_name)?;
        log::trace!("Writing sub to file");
        file.write_all(sub.as_bytes())?;
        log::trace!("Sub {} saved", i);
    }
    Ok(())
}