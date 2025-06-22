use std::{fs::File, io::Write};

pub fn create_and_write_to_file(file_name: &str, content: &str) -> std::io::Result<()> {
    log::info!("Creating {file_name}");
    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
