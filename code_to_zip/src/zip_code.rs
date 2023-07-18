use std::io::{Read, Write};
use std::path::PathBuf;
use std::path::Path;
use std::fs::{File, self};
use tar::Header;
use zip::result::ZipResult;
//use std::env;
use zip::write::FileOptions;
use zip::write::ZipWriter;
use flate2::write::GzEncoder;
use flate2::Compression;
use flate2::read::GzDecoder;


// zip,     .zip
pub fn zip_code(code: PathBuf, zip_command: Option<String>) ->std::io::Result<(u64,u64)> {
    let file = File::create(zip_command.unwrap())?;
    let mut zip = ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let mut original_size = 0;
    let mut compressed_size = 0;


    let result = compress_folder(&code, &mut zip, &options, &mut original_size, &mut compressed_size);

    match result {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Error is :{:?}",err)
        }
    }

    zip.finish()?;
    Ok((original_size,compressed_size))
}

#[allow(unused_must_use)]
fn compress_folder(
    folder: &PathBuf,
    zip: &mut ZipWriter<File>,
    options: &FileOptions,
    original_size: &mut u64,
    compressed_size: &mut u64,
) -> ZipResult<()>{
    if folder.is_dir(){
        let files = fs::read_dir(&folder)?;

        for file in files{
            let file = file?;
            let path = file.path();

            if path.is_file() {
                let file_name = path
                                        .file_name()
                                        .unwrap_or_default()
                                        .to_string_lossy()
                                        .into_owned();
                
                zip.start_file(file_name, options.clone());
                
                let mut source = File::open(path.clone())?;
                let mut buffer = Vec::new();

                source.read_to_end(&mut buffer)?;
                
                *original_size += buffer.len() as u64;
                *compressed_size += buffer.len() as u64;

                zip.write_all(&buffer)?;

                // 将压缩后的文件放回原来的文件夹
                //fs::write(path, &buffer)?;
                let parent_folder = path.parent().expect("Failed to get parent folder");
                let new_file_path = parent_folder.join(file_name);

                fs::write(new_file_path, &buffer)?;
            }else if path.is_dir() {
                compress_folder(&path, zip, options, original_size, compressed_size)?;
            }
        }
    }else {
        let file_name = folder
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .into_owned();
        zip.start_file(file_name, options.clone());
                
                let mut source = File::open(folder.clone())?;
                let mut buffer = Vec::new();

                source.read_to_end(&mut buffer)?;
                
                *original_size += buffer.len() as u64;
                *compressed_size += buffer.len() as u64;

                zip.write_all(&buffer)?;

                fs::write(folder, &buffer)?;
    }
    Ok(())
}





// flate2,  tar.gz
pub fn compress_folder_flate(source_folder: &Path, dest_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let dest_file = File::create(dest_file)?;
    let encoder = GzEncoder::new(dest_file, Compression::default());

    let mut archive = tar::Builder::new(encoder);

    add_folder_to_archive(source_folder, &mut archive, source_folder)?;

    Ok(())
}

fn add_folder_to_archive(
    source_folder: &Path,
    archive: &mut tar::Builder<GzEncoder<File>>,
    current_folder: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(current_folder)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            add_folder_to_archive(source_folder, archive, &path)?;
        } else if path.is_file() {
            let rel_path = path.strip_prefix(source_folder)?;
            let mut header = header_for_file(source_folder, &path)?;
            let file = File::open(&path)?;
            archive.append_data(&mut header,rel_path,&file)?;
        }
    }

    Ok(())
}
fn header_for_file(source_folder: &Path, file_path: &Path) -> Result<Header, Box<dyn std::error::Error>> {
    let metadata = fs::metadata(file_path)?;
    let rel_path = file_path.strip_prefix(source_folder)?;

    let mut header = Header::new_gnu();
    header.set_path(rel_path)?;
    header.set_metadata(&metadata);
    header.set_entry_type(tar::EntryType::file());

    Ok(header)
}

pub fn decompress_archive(source_file: &Path, dest_folder: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let source_file = File::open(source_file)?;
    let decoder = GzDecoder::new(source_file);
    let mut archive = tar::Archive::new(decoder);

    archive.unpack(dest_folder)?;

    Ok(())
}
