use std::borrow::Cow;

/*
    以空格拆分字符
 let input = "Hello world! Rust programming";

let words: Vec<&str> = input.split_whitespace().collect();

for word in words {
    println!("{}", word);
} */

use std::path::{Path, PathBuf};

use crate::zip_code::decompress_archive;
use crate::command::open_html;
use crate::argment::Arguments;
use crate::zip_code::zip_code;
use crate::command::command_to_run;
use crate::zip_code::compress_folder_flate;


pub fn analysis(args: &[String]){
    let args = match Arguments::new(&args){
        Ok(s) => s,
        Err(err) => return{
            println!("{}",err);
        }
    };
    
    if args.flag == "-h" || args.flag == "-help"{
        println!(
            "Usage: \n\r'-h' or '-help': here's some help for you \n\r'-f': it's a file path for you and you can fill here to zip your file example:-f file_path compress_file \n\r'-r': it's a file path for you and you can fill here to zip your file example:-r file_path command compress_file\n\r'-w' you can fast to run you code in here example: -w file_path command \n\r '-z' you can choice your compress mode,'zip' or 'tar',it's Represents a different algorithm "
            );

    }else if args.flag == "-f"{
        let file = args.file;
        let compress_file = args.zip_command;

        let result = zip_code(file, compress_file);
        match result {
            Ok((original_size, compressed_size)) => {
                println!("{} is zip to {}", original_size, compressed_size);
            },
            Err(err) => {
                println!("{}", err);
            }
        }
    }else if args.flag == "-r" {
       
        let file = args.file.clone();
        let comand = args.run_command;
        let compress_file = args.zip_command;

        let result = zip_code(file.clone(), compress_file);
        match result {
            Ok((original_size, compressed_size)) => {
                println!("{} is zip to {}", original_size, compressed_size);
                println!("{} is Minimum memory",compressed_size)
            },
            Err(err) => {
                println!("{}", err);
            }
        }

        if comand == Some("html".to_string()){
            let result = open_html(Some(file.clone()));
            match result {
                Ok(()) => (),
                Err(s) => {
                    println!("Error: {:?}",s);
                }
            }
        }else {
            command_to_run(comand, Some(file));
        }
        println!("zip is over and this code is run");

    }else if  args.flag == "-w"{
        
        let file = args.file.clone();
        let comand = args.run_command;
        if comand == Some("html".to_string()){
            let result = open_html(Some(file.clone()));
            match result {
                Ok(()) => (),
                Err(s) => {
                    println!("Error: {:?}",s);
                }
            }
        }else {
            command_to_run(comand, Some(file));
        }
    }else if args.flag == "-z" {
        let file = args.file;
        let zip_way = args.zip_way;
        let compress_file = args.zip_command;
        let file_compress = option_string_to_path(compress_file.clone());

        if zip_way == None{
            println!("it's not enough arguments");

        }else if zip_way == Some("zip".to_string()) {
            let result = zip_code(file.clone(), compress_file);
            match result {
                Ok((original_size, compressed_size)) => {
                    println!("{} is zip to {}", original_size, compressed_size);
                    println!("{} is Minimum memory",compressed_size)
                },
                Err(err) => {
                    println!("{}", err);
                }
            }
        }else if zip_way == Some("tar".to_string()) {
            let result = compress_folder_flate(&file, &file_compress.clone().unwrap());
            let result_2 = decompress_archive(&file, &file_compress.unwrap());
            match result {
                Ok(_) => {
                    println!("over!");
                }
                Err(err) => {
                    println!("{}", err);
                }
            }

            match result_2 {
                Ok(_) => {
                    println!("");
                }
                Err(err) => {
                    println!("{:?}",err);
                }
            }
        }
    }else if args.flag == "exit" {
        std::process::exit(0);
    }else if args.flag.is_empty() {
        println!("it's not enough arguments")
    }else {
        panic!("Invalid arguments");
    }
}



fn option_string_to_path(option_string: Option<String>) -> Option<Cow<'static, Path>> {
    option_string.map(|s| Cow::Owned(PathBuf::from(s)))
}