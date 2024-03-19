use std::fs::File;
use std::os::unix::fs::MetadataExt;
use std::process::Command;
use std::sync::{Mutex, RwLock};
use std::{env, fs};
use std::io::{self, Error, ErrorKind, Read, Write};
use std::path::Path;

use lazy_static::lazy_static;


// whoami
pub fn whoami(session_context: &mut SessionContext) -> String{
    let mut res = session_context.get_username();
    if session_context.user_state.root{
        res = "root".to_string()
    }

    res
}


// help 
pub fn help() -> String{
    let help = format!(
    "Usage: <command> [options] [arg]
\0\x1B[32mCommands:
    pwd     View current directory                         apt -i ..   Install package
    ls      View all files in the current directory        history     View past Commands
    cd      Change directory                               whoami  ||  apt -update version
    rm      Delete directory or file                       rn          Rename directory or file  
    touch   Create a new file                              mkdir       Create a new directory
    cat     View file only read                            mv          Move file's path
    python  Run code in python                             tar -zxvf:  Compression  
    html    Open html file                                 tar -xvf:   Decompression
    exit    Exit this process\0\x1B[0m\n"
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


// ll
pub fn ll(context: &SessionContext) -> io::Result<String>{
    let dir_path = Path::new("./");
    let mut result = String::new();
    let dirs = fs::read_dir(dir_path)?;
    for dir in dirs{
        let dir = dir?;
        let matadata = dir.metadata()?;
        let file_type = dir.file_type()?;

        // file type
        let file_type_str = if file_type.is_dir(){
            "d"
        }else if file_type.is_file(){
            "-"
        }else if file_type.is_symlink(){
            "l"
        }else{
            "?"
        };

        // file name
        let file_name = if file_type.is_dir(){
            format!("\x1B[32m{}    \x1B[0m", dir.path().display())
        }else if file_type.is_file(){
            dir.file_name().into_string().unwrap()
        }else{
            format!("\x1B[32m{}    \x1B[0m", dir.path().display())
        };


        // permission
        let uid = matadata.uid();
        let gid = matadata.gid();

        let output_o = match uid{
            1000=>context.get_username(),
            0=>"root".to_string(),
            _=>"-".to_string()
        };

        let output_p = match gid{
            1000=>context.get_username(),
            0=>"root".to_string(),
            _=>"-".to_string()
        };
        

        let size = matadata.len();
        

        // created time
        let path = dir.path();
        let s = path.as_os_str().to_str().unwrap();
        let time = file_create_time(s);

        // output
        result.push_str(&format!(
            "{} {}  {:>8}   {:>6} {}  {}\n",
            file_type_str,
            output_p,
            output_o,
            size,
            time,
            file_name
        ));
    }
    Ok(result)
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

    let res = String::new().trim().to_owned();
    Ok(res)
}


// cd
pub fn cd(path: &str) -> Result<String,Error>{
    let new_path = Path::new(path);
    env::set_current_dir(new_path)?;

    let res = format!("Successfully changed directory to {}.",path);
    Ok(res)
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

    let res = format!("Successfully created {}",file);
    Ok(res)
}


// mkdir
pub fn mkdir(dir: &str) -> Result<String,std::io::Error>{
    if dir.is_empty(){
        return Ok("there is None".to_string());
    }
    let mut builder = fs::DirBuilder::new();
    let _ = builder.recursive(true).create(Path::new(dir));

    let res = format!("Successfully created {}",dir);
    Ok(res)
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

    let res = String::new().trim().to_owned();
    Ok(res)
}


// rn mv
pub fn rename(source:&str,now:&str) -> std::io::Result<String> {
    if source.is_empty(){
        return Ok("there is None".to_string());
    }
    let _ = fs::rename(source, now);
    let res = String::new().trim().to_owned();
    Ok(res)
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


use crate::cache::CacheMap;
use crate::commands::download::{download_package, find_package};
use crate::get::get_hty::file_create_time;
use super::download::update;
use crate::root::SessionContext;


// apt -install  xxx
pub fn apt(name: &str) -> io::Result<String>{
    if name.is_empty(){
        return Ok("Error: Missing parameters".to_string());
    }
    match find_package(name) {
        Some(package) => {
            if let Err(err) = download_package(&package) {
                eprintln!("Error: {}", err);
            }
        },
        None => {
            eprintln!("Package {} not found.", name);
        }
    }

    let res = format!("Successfully download Package {}",name);
    Ok(res)
}


// apt -update xxx
pub fn update_new(version: &str) -> io::Result<String>{
    if version.is_empty(){
        return Ok("Error: Missing parameters".to_string());
    }
    match update(&version) {
        Ok(_) => {
            let script_path = dirs::home_dir().unwrap().join(".Tiks").join("update_script.sh");
            let output = Command::new("bash")
                .arg(script_path.clone())
                .output()
                .expect("Error: network error....");
            if output.status.success() {
                let _ = std::fs::remove_file(script_path);
            }
        }
        Err(_) => {
            let err = format!("The current version is the latest one");
            return Ok(err);
        },
    }

    let res = format!("Successfully Update version {}",version);
    Ok(res)
}


use tar::Archive;
use flate2::read::GzDecoder;
use flate2::Compression;
use flate2::write::GzEncoder;
use super::arg::{execute_command, Commands};


pub fn zxvf(file: &str, to: &str) -> Result<String,std::io::Error>{
    if file.is_empty() || to.is_empty(){
        return Ok("Error: Missing parameters".to_string());
    }
    let tar_gz = File::create(to)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_file(file, &mut File::open(file)?)?;
    Ok("Successfully Compression".to_string())
}


pub fn xvf(to: &str) -> Result<String,std::io::Error>{
    if to.is_empty(){
        return Ok("Error: Missing parameters".to_string());
    }
    let tar_gz = File::open(to)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;
    Ok("Successfully Decompression".to_string())
}


// 重定向输出   > 
pub async fn stdout_file(commands: Commands,cache: CacheMap,session_context: &mut SessionContext) -> Result<String, std::io::Error>{
    let command = commands.command.clone();
    let arg = commands.arg.clone();
    let result = execute_command(&command, "", &arg, session_context,cache.clone()).await?;
    let mut file = File::create(arg[arg.len()-1].clone())?;
    file.write_all(result.as_bytes())?;
    Ok("write over!".to_string())
}


// cp
#[allow(unused_assignments)]
pub fn cp(source:&str, to: &str) -> io::Result<String>{
    if source.is_empty() || to.is_empty(){
        return Ok("Error: Missing parameters".to_string());
    }

    let file = fs::read(source)?;

    let result = fs::write(to, file);
    let mut output = String::new();
    match result.is_ok(){
        true => {
            output = format!("Successfully to copy {}",to);
        },
        false =>{
            output = format!("Error: copy {} to {} failed",source,to);
        }
    }

    Ok(output)
}


// sudo
#[allow(unused_assignments)]
pub fn sudo(session_context: &mut SessionContext)->io::Result<String>{
    loop{
        let mut output = String::new();
        let user = session_context.get_username();
        println!("[sudo] password for {}:",user);
        let pd = rpassword::read_password().unwrap();
        let res = session_context.toggle_root(pd);
        if res.is_ok() {
            output = format!("Sucessfully to change root");
            return Ok(output);
        } else {
            println!("Sorry, try again");
            continue;
        }
    }
}