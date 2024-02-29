use std::{env, fs};
use std::io::{self, Error, ErrorKind, Read};
use std::path::{Component, Path};

// help 
pub fn help() -> String{
    let help = format!(
    "Commands:
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
    "
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
                result.push_str(&format!("  {}  ", entry.file_name().into_string().unwrap()));
            } else {
                result.push_str(&format!("  {}  ", entry.path().display()));
            }
        }
        Ok(result)
    } else {
        Err(Error::new(ErrorKind::NotFound, "Path is not a directory"))
    }
}

// cd
pub static mut PATH:Option<&str> = Some("./");

pub fn cd(path: Option<&str>) -> Result<String,Box<dyn std::error::Error>>{
    let mut new_dir = env::current_dir().unwrap();
    let new_path = Path::new(path.unwrap());
    for component in new_path.components(){
        match component {
            Component::Normal(p) => new_dir.push(p),
            Component::ParentDir => {new_dir.pop();},
            _ => {}
        }
    }

    env::set_current_dir(new_path)?;
    Ok(String::new())
}

// touch
pub fn touch(file: &str) -> Result<String,std::io::Error>{
    let _ = fs::File::create_new(Path::new(file));
    Ok(String::new())
}
// mkdir
pub fn mkdir(dir: &str) -> Result<String,std::io::Error>{
    let mut builder = fs::DirBuilder::new();
    let _ = builder.recursive(true).create(Path::new(dir));
    Ok(String::new())
}
// rm
pub fn rm(file: &str) -> Result<String,std::io::Error>{
    let filepath = Path::new(file);
    match filepath.is_dir() {
        true => {
            let _ = fs::remove_dir(filepath);
        },
        false => {
            let _ = fs::remove_file(filepath);
        }
    }
    Ok(String::new())
}


// rn
pub fn rename(source:&str,now:&str) -> std::io::Result<String> {
    let _ = fs::rename(source, now);
    Ok(String::new())
}

// mv


// cat
pub fn cat(file: &str) -> Result<String,Error>{
    let f = fs::File::open(Path::new(file));
    let mut buffer = String::new();
    let _ = f.unwrap().read_to_string(&mut buffer);
    Ok(buffer)
}

// history
pub static mut HISTROY:Vec<String>=Vec::new();
pub fn history_push(command: String){
    unsafe{
        HISTROY.push(command);
    }
}

pub fn history() -> String{
    unsafe{
        for (i,item) in HISTROY.iter().enumerate(){
            println!("{}: {}",i+1,item);
        }
    }
    String::new()
}