use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug,StructOpt)]
pub struct Arguments{
    pub flag: String,
    pub zip_way: Option<String>, 
    #[structopt(short = "f",long = "file")]
    pub file: PathBuf,                  // -> zip_code -> file_local

    #[structopt(short = "w",long = "way")]
    pub run_command: Option<String>,    // -> command

    #[structopt(short = "z",long = "zip")]
    pub zip_command: Option<String>,  // -> zip_code-> local  
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str>{   // -h 帮助, -f 仅进行文件压缩 ,-w 进行文件压缩且进行运行文件 ,-r,只进行文件的运行，文件保存地址,"-z" 可选择压缩方式为（.zip,.tar）
        if args.len() == 0{
            return Err("it's not enough arguments");
        }

        let flag = args[0].clone();
        if flag == "-h" || flag == "-help"{
            // help
            Ok(
                Arguments{
                flag: String::from("-h"),
                zip_way:None,
                file:PathBuf::from(""),
                run_command:None,
                zip_command:None,
                }
            )
        }else if flag == "-f" && args.len() > 1 {
          // flag , file , zip_localtion
            let file = match PathBuf::from_str(&args[1]) {
                Ok(s) => s,
                Err(_) => return Err("not a vaild file path")
            };
            let compress_file = match args[2].parse::<String>() {
                Ok(s) => Some(s),
                Err(_) => return Err("please uploading your zip way")
            };
            return Ok(
                Arguments{
                    flag: String::from("-f"),
                    zip_way:None,
                    file:PathBuf::from(file),
                    run_command:None,
                    zip_command:compress_file,
                }
            );
        }else if flag == "-r" && args.len() > 1 {  
            // flag, file, command, zip_localtion
            let file = match PathBuf::from_str(&args[1]) {
                Ok(s) => s,
                Err(_) => return Err("not a vaild file path")
            };
            let command = match args[2].parse::<String>() {
                Ok(s) => Some(s),
                Err(_) => return Err("failed to parse command,maybe your computer is not support this language")
            };
            let compress_file = match args[3].parse::<String>() {
                Ok(s) => Some(s),
                Err(_) => return Err("please uploading your zip way")
            };
            return Ok(
                Arguments{
                    flag: String::from("-r"),
                    zip_way:None,
                    file:PathBuf::from(file),
                    run_command:command,
                    zip_command: compress_file,
                }
            );
        }else if flag == "-w" && args.len() > 1{  
            // flag, file, command
            if args[1].len() == 0{
                return Err("it's not enough arguments");
            }
            let file = match PathBuf::from_str(&args[1]) {
                Ok(s) => s,
                Err(_) => return Err("not a vaild file path")
            };
            let command = match args[2].parse::<String>() {
                Ok(s) => Some(s),
                Err(_) => return Err("failed to parse command,maybe your computer is not support this language")
            };
            return Ok(
                Arguments{
                    flag: String::from("-w"),
                    zip_way:None,
                    file:PathBuf::from(file),
                    run_command:command,
                    zip_command:None,
                }
            );
        }else if flag == "-z" && args.len() > 1{
            // choice compress mode
            let file = match PathBuf::from_str(&args[2]) {
                Ok(s) => s,
                Err(_) => return Err("not a vaild file path")
            };
            let zip_way = match args[1].parse::<String>() {
                Ok(s) => Some(s),
                Err(_) => return Err("maybe our system can't support this zip way"),
            };
            let compress_file = match args[3].parse::<String>() {
                Ok(s) => Some(s),
                Err(_) => return Err("please uploading your zip way")
            };

            return Ok(
                Arguments{
                    flag:String::from("-z"),
                    zip_way,
                    file:PathBuf::from(file),
                    run_command:None,
                    zip_command:compress_file,
                }
            );
        }else if flag == "exit" {
            return Ok(
                Arguments{
                    flag: String::from("exit"),
                    zip_way:None,
                    file:PathBuf::from(""),
                    run_command:None,
                    zip_command:None,
                }
            );
        }else {
            return Err("These parameters are meaningless");
        }    
    }
}