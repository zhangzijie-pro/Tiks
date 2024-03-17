use crate::cache::{Cache, CacheMap, Cache_get};
use crate::commands::command::stdout_file;
use crate::root::SessionContext;

use super::code::{html, python};
use super::command::{history, ll, ls, rename, turn_dir, turn_file, whoami, xvf, zxvf};

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
        let mut command = String::new();
        let mut option = String::new();
        let mut arg: Vec<String> = Vec::new();
        match len{
            1=>{
                command = commands[0].clone();
            }
            2 =>{
                command = commands[0].clone();
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
                command = commands[0].clone();
                match commands[1].starts_with("-")|| commands[1]==">"{
                    true => {
                        option=commands[1].clone()
                    },
                    false =>{
                        arg.push(commands[1].clone())
                    }
                }
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
}

pub async fn handle_command(cache: CacheMap, args: Vec<String>, session_context: &mut SessionContext) {
    let commands = Commands::new(args.clone());
    let command = commands.command.clone();

    if session_context.user_state.root{
        // Execute root commands
        // Handle commands differently when user is in root mode
        if let Ok(res) = command_match(commands.clone(), cache.clone(),session_context).await{
            println!("{}",res)
        }
    } else if !session_context.user_state.root && !session_context.root.allowed_commands.contains(&command) {
        // Execute normal commands
        // Handle commands normally when user is not in root mode
        if let Ok(res) = command_match(commands.clone(), cache.clone(),session_context).await{
            println!("{}",res)
        }
    }else{
        eprintln!("Permission not support")
    }
}


pub async fn command_match(commands: Commands,cache: CacheMap,session_context: &mut SessionContext) -> Result<String,std::io::Error>{
    let command = commands.command.clone();
    let option = commands.option.clone();
    let arg = commands.arg.clone();
    match option.as_str() {
        ">" => stdout_file(commands,cache.clone(), session_context).await,
        _ => execute_command(&command, &option, &arg, session_context, cache).await,
    }
}


pub async fn execute_command(command: &str, option: &str, arg: &Vec<String>, session_context: &mut SessionContext, cache: CacheMap) -> Result<String, std::io::Error> {
    match command {
        "root" => {
            session_context.user_state.toggle_root();
            let s = format!("Switched to root mode: {}", session_context.user_state.root);
            Ok(s)
        },
        "exit" => {
            match option{
                "-all" => {
                    cache.clear();
                    std::process::exit(0);
                },
                _=>{
                    if session_context.user_state.root {
                        session_context.user_state.exit_root();
                        println!("Switched to root mode: {}", session_context.user_state.root);
                    } else {
                        cache.clear();
                        std::process::exit(0);
                    }
                }
            }
            Ok("Exit".to_string())
        },
        "whoami" => Ok(whoami(session_context)),
        "pd" => match option{
            "-f"|"-fix" => session_context.user.revise_password(&arg[0].clone()),
            "-c"|"-check"=>Ok(session_context.user.password.clone()),
            _=>Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Error: can't found tar {}", option))),
        },
        "ll" => {
            let va = ll(&session_context).unwrap();
            Ok(va)
        },
        _ => execute_other_command(command, option, arg, cache).await,
    }
}

pub async fn execute_other_command(command: &str, option: &str, arg: &[String], cache: CacheMap) -> Result<String, std::io::Error> {
    match command {
        "history" => history(),
        "ls" | "l" => ls(),
        "cd" | "rm" | "mkdir" | "touch" | "python" | "html" | "web" | "cat" => {
            turn_file_or_dir(command, &arg[0]).await
        }
        "tar" => match option {
            "-zxvf" => zxvf(&arg[0], &arg[1]),
            "-xvf" => xvf(&arg[0]),
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Error: can't found tar {}", option))),
        },
        "rn" => rename(&arg[0], &arg[1]),
        _ => match_cache_or_error(cache.clone(), command.to_string()).await,
    }
}

async fn turn_file_or_dir(command: &str, arg: &str) -> Result<String, std::io::Error> {
    if let Ok(res) = turn_file(command.to_string(), arg.to_string()) {
        Ok(res)
    } else if let Ok(res) = turn_dir(command.to_string(), arg.to_string()) {
        Ok(res)
    } else if let Ok(res) = run_code(&command.to_string(), Some(arg)) {
        Ok(res)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error: Can't found this: \x1B[33m{}\x1B[0m", command),
        ))
    }
}

async fn match_cache_or_error(cache: CacheMap, command: String) -> Result<String, std::io::Error> {
    match <Cache as Cache_get>::cache_get(cache.clone(), command.to_string()).await {
        Some(s) => Ok(s.to_string()),
        None => {
            eprintln!("Error: Can't found this \x1B[31m{}\x1B[0m", command);
            Ok(String::new())
        }
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