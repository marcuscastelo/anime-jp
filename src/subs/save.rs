use std::{error::Error, fs::File, io::Write};

pub type SubsInRAM = Vec<String>;


// //TODO: refactor into struct?

// // Save the subs to disk: each sub is a file
// // The file name is the anime name
// // The file extension is the sub language
// // The file content is the sub content
// pub fn save_subs(subs: &SubsInRAM, folder: &str, file_name_with_ext: &str) -> Result<(), Box<dyn Error>> {
//     // Create the tmp folder if it doesn't exist
//     log::trace!("Creating tmp folder if it doesn't exist");
//     std::fs::create_dir_all(folder)?; //TODO: check if it is a secure way to create a folder
//     log::trace!("Tmp folder created");
    
//     for (i, sub) in subs.iter().enumerate() {
//         log::debug!("Saving sub: {}", i);
//         let file_name = format!("{}/{}", folder, file_name_with_ext);
//         log::trace!("Creating file: {}", file_name);
//         let mut file = File::create(file_name)?;
//         log::trace!("Writing sub to file");
//         file.write_all(sub.as_bytes())?;
//         log::trace!("Sub {} saved", i);
//     }
//     Ok(())
// }