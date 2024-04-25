// as linux "apt" to download some file or software

// download file in web such as : http:*********
// maybe there are some error 
use std::fs::File;
use std::io::{copy, Write};
use reqwest::Client;

use super::command::xvf;

#[allow(dead_code)]
pub struct Package{
    name: String,
    version: String,
    download_link: String,
}

// apt install 


// upload soon
impl Package{
    pub  fn new(name: String, version: String, download_link: String) -> Package{
        Package{
            name,
            version,
            download_link
        }
    }
}

pub fn find_package(name: &str) -> Option<Package>{
    match name {
        "tree" => Some(Package::new("tree".to_owned(), "1.85.0".to_owned(), "http://mama.indstate.edu/users/ice/tree/src/tree-1.8.0.tgz".to_owned())),
        _ => None
    }
}

pub fn download_package(package: &Package) -> Result<(),Box<dyn std::error::Error>>{
    let _ = download(&package.download_link, &format!("{}.tar.gz",package.name));
    // 解压缩并安装
    let file = format!("{}.tar.gz", package.name);
    let _ = xvf(&file).expect("Failed to install package.");

    Ok(())
}

async fn download(link: &str, filename: &str) -> Result<(),Box<dyn std::error::Error>> {
    let cilent = Client::new();

    let response = cilent.get(link).send().await.unwrap();
    println!("{:?}",response);
    if !response.status().is_success(){
        eprint!("Fail to get Response")
    }
    let mut file = File::create(filename).unwrap();

    copy(&mut response.bytes().await.unwrap().as_ref(), &mut file)?;
    Ok(())
}


// apt update new

const _GITHUB_RELEASE_LINUX: &str = "https://github.com/zhangzijie-pro/Tiks/releases/download/1.0.0/tiks";
const _GITHUB_RELEASE_WINDOW: &str = "https://github.com/zhangzijie-pro/Tiks/releases/download/1.0.0/tiks.exe";

// upload soon
pub fn update(version: &str) -> std::io::Result<()>{
    let mut version = version;
    let home = dirs::home_dir().unwrap();
    let app_dir = home.join(".Tiks");
    let app = app_dir.join("update_script.sh");

    let mut file = std::fs::File::create(&app).unwrap();
    if version=="1.0.0"{
        version="main";
    }
    let update = format!("
#!/bin/bash
cd \"{}\"
git pull origin {}

cargo clean
cargo build
",app_dir.display(),version);

    let u = update.as_bytes();
    let _ = file.write(u);

    Ok(())
}

fn _update_last(){
    
}