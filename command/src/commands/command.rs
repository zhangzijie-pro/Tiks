use std::fs::File;
use std::sync::{Mutex, RwLock};
use std::{env, fs};
use std::io::{self, Error, ErrorKind, Read};
use std::path::Path;

use lazy_static::lazy_static;

// whoami
pub fn whoami() -> String{
    whoami::username()
}

// help 
pub fn help() -> String{
    let help = format!(
    "Usage: <command> [arg] [options]
\n\x1B[34mCommands:
    pwd  View current directory
    ls   View all files in the current directory
    cd   Change directory   
    rm   delete directory or file  
    rn   rename directory or file  
    touch   create a new file
    mkdir   create a new directory
    history   View past Commands
    cat     view file only read
    mv      move file's path
    python  run code in python
    html    open html file
    apt     download file or software
    tar -zxvf : Compression    tar -xvf  : decompression
    exit    exit this process\x1B[0m\n"
    );
    help
}

// pwd
pub fn pwd() -> String{
    let path = env::current_dir().unwrap().as_path().display().to_string();
    path
}

// ls
pub fn ls() -> io::Result<String> { 
    let dir_path = Path::new("./");
    let mut result = String::new();
    if dir_path.is_dir() {
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                result.push_str(&format!("{}    ", entry.file_name().into_string().unwrap()));
            } else {
                result.push_str(&format!("\x1B[32m{}    \x1B[0m", entry.path().display()));
            }
        }
        Ok(result)
    } else {
        Err(Error::new(ErrorKind::NotFound, "Path is not a directory"))
    }
}

// history
lazy_static!{
    pub static ref HISTROY: Mutex<Vec<String>> = Mutex::new(Vec::new());
}
pub fn history_push(command: String){
    let mut history = HISTROY.lock().unwrap();
    history.push(command); 
}

pub fn history() -> Result<String,Error>{
    let s = HISTROY.lock().unwrap();
    for (i,c) in s.iter().enumerate(){
        println!("{}: {}",i,c);
    }
    Ok(String::new())
}

// cd
pub fn cd(path: &str) -> Result<String,Error>{
    let new_path = Path::new(path);
    env::set_current_dir(new_path)?;
    Ok("cd over!".to_string())
}

// new dir
lazy_static!{
    pub static ref DIR: RwLock<&'static str> = RwLock::new("");
}

pub fn turn_dir(command: String, dir: String) -> Result<String,Error>{
    let mut dir_lock = DIR.write().unwrap();
    *dir_lock = Box::leak(dir.into_boxed_str());

    match command.as_str() {
        "mkdir" => {
            mkdir(&dir_lock)
        },
        "rm" => {
            rm(&dir_lock)
        },
        "cd" =>{
            cd(&dir_lock)
        }
        _ => {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported command",
            ))
        }
    }
}

// new file 
lazy_static! {
    pub static ref FILE: RwLock<&'static str> = RwLock::new("");
}

pub fn turn_file(command: String,file: String) -> Result<String, Error> {
    let mut file_lock = FILE.write().unwrap();
    *file_lock = Box::leak(file.into_boxed_str());

    match command.as_str() {
        "touch" => {
            touch(&file_lock)
        },
        "cat" => {
            cat(&file_lock)
        }
        "rm" => {
            rm(&file_lock)
        }
        _ =>{
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported command",
            ))
        }
    }
}

//touch
pub fn touch(file: &str) -> Result<String,std::io::Error>{
    if file.is_empty(){
        return Ok("there is None".to_string());
    }
    let _ = fs::File::create_new(Path::new(file))?;
    Ok("create over!".to_string())
}

// mkdir
pub fn mkdir(dir: &str) -> Result<String,std::io::Error>{
    if dir.is_empty(){
        return Ok("there is None".to_string());
    }
    let mut builder = fs::DirBuilder::new();
    let _ = builder.recursive(true).create(Path::new(dir));
    Ok("create over!".to_string())
}

// rm
pub fn rm(file: &str) -> Result<String,std::io::Error>{
    if file.is_empty(){
        return Ok("there is None".to_string());
    }
    let filepath = Path::new(file);
    match filepath.is_dir() {
        true => {
            let _ = fs::remove_dir(filepath);
        },
        false => {
            let _ = fs::remove_file(filepath);
        }
    }
    Ok("remove over!".to_string())
}

// rn
pub fn rename(source:&str,now:&str) -> std::io::Result<String> {
    if source.is_empty(){
        return Ok("there is None".to_string());
    }
    let _ = fs::rename(source, now);
    Ok(String::new())
}

// mv
pub fn move_file(source:&str,_now:&str) -> std::io::Result<String>{
    if source.is_empty(){
       return Ok("there is None".to_string());
    }

    Ok(String::new())
}


// cat
pub fn cat(file: &str) -> Result<String,Error>{
    if file.is_empty(){
        return Ok("there is None".to_string());
    }
    let f = fs::File::open(Path::new(file));
    let mut buffer = String::new();
    let _ = f.unwrap().read_to_string(&mut buffer);
    Ok(buffer)
}

use crate::commands::download::{download_package, find_package};
// download
pub fn download(name: &str) -> Result<(), Box<dyn std::error::Error>>{
    match find_package(name) {
        Some(package) => {
            Ok(if let Err(err) = download_package(&package) {
                eprintln!("Error: {}", err);
            })
        },
        None => Ok({
            eprintln!("Package {} not found.", name);
        })
    }
}

use tar::Archive;
use flate2::read::GzDecoder;
use flate2::Compression;
use flate2::write::GzEncoder;
use super::arg::Commands;

//
pub fn tar(command: Commands, file: &str, to: &str) -> Result<(),std::io::Error>{
    if command.command.as_str() == "tar"{
        match command.option.as_str(){
            "-h" => {
                let s = format!(
                    "
                  tar -zxvf : tar file
                  tar -xvf  : untar file
                    "
                );
                println!("{}",s)
            },
            "-zxvf" => {
                let _ = zxvf(file, to);
            },
            "-xvf" => {
                let _ = xvf(to);
            },
            _ =>{
                eprintln!("Can't support this arg")
            }
        }
    }
    Ok(())
}

pub fn zxvf(file: &str, to: &str) -> Result<(),std::io::Error>{
    let tar_gz = File::create(to)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_file(file, &mut File::open(file)?)?;
    Ok(())
}

pub fn xvf(to: &str) -> Result<(),std::io::Error>{
    let tar_gz = File::open(to)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;
    Ok(())
}

// 重定向输出
