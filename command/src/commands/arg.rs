use std::io::Error;

use async_trait::async_trait;

use crate::cache::{Cache, CacheMap, Cache_get};
use crate::commands::command::rename;
use crate::root::SessionContext;

use super::code::{html, python};
use super::command::{help, history, ls, turn_dir, turn_file};

#[async_trait]
pub trait Command {
    async fn handle_command(cache: CacheMap, args: Vec<String>, session_context: &SessionContext);
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Commands{
    pub command: String,
    pub option: String,
    pub arg: Vec<String>
}

impl Commands {
    pub fn new(commands: Vec<String>) -> Commands{
        let len = commands.len();
        let command = commands[0].clone();
        let mut option = String::new();
        let mut arg: Vec<String> = Vec::new();
        match len{
            2 =>{
                match commands[1].starts_with("-"){
                    true => {
                        option=commands[1].clone()
                    },
                    false =>{
                        arg.push(commands[1].clone())
                    }
                }
            },
            _ =>{
                option = commands[1].clone();
                arg.append(&mut commands[2..=len-1].to_vec())
            }
        }
        Commands{
            command,
            option,
            arg
        }
    }

    pub fn find_help(command: Commands) -> String{
        let mut res = String::new();
        if command.option=="-h"{
            let binding = help();
            let s = binding.lines();
            for i in s{
                if command.option == "-h" && i.contains(&command.command){
                    res = i.to_string()
                }
            }
            res = "Can't found this command".to_string()
        }
        res
    }
}


#[async_trait]
impl Command for Commands{
    // tar and find_help
    async fn handle_command(cache: CacheMap, args: Vec<String>, session_context: &SessionContext) {
        let len = args.len();
        let command = &args[0];
        if session_context.user_state.root && session_context.root.allowed_commands.contains(command) {
            // Execute root commands
            // Handle commands differently when user is in root mode
            match len {
                1 => {               
                    let res = normal_command(cache.clone(), command).await.unwrap();
                    println!("{}",res)
                },
                2 => {
                    let path = &args[1];
                    twice_option(command, path)
                },
                3 => {
                    let source = &args[1]; 
                    let now = &args[2];
                    if *command == "rename".to_string(){
                        let s = rename(source, now).unwrap();
                        println!("{}",s);
                    }
                },
                _ => {
                    eprint!("404: Not Found")
                }
            }
        } else if !session_context.user_state.root && !session_context.root.allowed_commands.contains(command) {
            // Execute normal commands
            // Handle commands normally when user is not in root mode
            match len{
                1 => {               
                    let res = normal_command(cache.clone(), command).await.unwrap();
                    println!("{}",res)
                },
                2 => {
                    let path = &args[1];
                    twice_option(command, path)
                },
                _ => {
                    eprint!("404: Not Found")
                }
            }
        }else{
            eprintln!("Permission not support")
        }
    }
}

async fn normal_command(cache:CacheMap,command: &String) -> Result<String,Error>{
    match command.as_str() {
        "history" => {
            history()
        },
        "ls"|"l" => {
            ls()
        },
        _ => Ok({
            let value = <Cache as Cache_get>::cache_get(cache.clone(), command.to_string()).await;
            match value {
                Some(ref s) => s.to_string(),
                None => {
                    eprintln!("Error: Can't found this \x1B[31m{}\x1B[0m",command);
                    String::new()
                }
            }
        })
    }

}

fn twice_option(command: &String,path: &String){
    if let Ok(res) = turn_file(command.clone(), path.clone()) {
        println!("{}",res);
    }else if let Ok(res) = turn_dir(command.clone(), path.clone()) {
        println!("{}",res);
    }else if let Ok(res) = run_code(command,Some(&path)){
        println!("{}",res);
    }else{
        eprintln!("Error: Can't found this: \x1B[33m{}\x1B[0m", command);
    }
}

fn run_code(command: &String,file: Option<&str>) -> Result<String,std::io::Error>{
    match command.as_str() {
        "html" | "web" => {
            html(file)
        },
        "python" | "py" => {
            python(file)
        },
        _ => Ok({
            let apt = format!("      
Command '{}' not found, did you mean:
    apt install {}
        ",command,command);
            apt
        }) 
    }
}

async fn root_command(len: usize,cache: CacheMap,command: Commands){
    match len {
        1 => {               
            let res = normal_command(cache.clone(), &command.command).await.unwrap();
            println!("{}",res)
        },
        2 => {
            twice_option(&command.command, &command.arg[0])
        },
        3 => {
            let source = &command.arg[0]; 
            let now = &command.arg[1];
            if command.command == "rename".to_string(){
                let s = rename(source, now).unwrap();
                println!("{}",s);
            }
        },
        _ => {
            eprint!("404: Not Found")
        }
    }
}

async fn user_command(len: usize,cache: CacheMap,command: Commands){
    match len {
        1 => {               
            let res = normal_command(cache.clone(), &command.command).await.unwrap();
            println!("{}",res)
        },
        2 => {
            twice_option(&command.command, &command.arg[0])
        },
        _ => {
            eprint!("404: Not Found")
        }
    }
}