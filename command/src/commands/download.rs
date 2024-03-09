// as linux "apt" to download some file or software

// download file in web such as : http:*********
use std::{fs::File, process::Command};
use std::io::copy;
use reqwest::Client;

#[allow(dead_code)]
pub struct Package{
    name: String,
    version: String,
    download_link: String,
}

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
    let output = Command::new("")
        .arg("")
        .arg(format!("{}.tar.gz", package.name))
        .output()?;
    
    if output.status.success() {
        println!("Package {} installed successfully.", package.name);
    } else {
        eprintln!("Failed to install package {}.", package.name);
    }

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