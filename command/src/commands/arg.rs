use crate::commands::command::stdout_file;
use crate::get::get_hty::get_similar;
use crate::root::{decryption, SessionContext};

use super::code::{html, python};
use super::command::{apt, cp, get_time, grep, history, ll, ls, pipe, rename, sudo, turn_dir, turn_file, update_new, whoami, xvf, zxvf};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Commands{
    pub command: String,
    pub option: String,
    pub arg: Vec<String>
}

#[allow(unused_assignments)]
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
                if commands.contains(&"|".to_string()){
                    arg=commands;
                }else{
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
        }
        Commands{
            command,
            option,
            arg
        }
    }
}


pub async fn handle_command(args: Vec<String>, session_context: &mut SessionContext) {
    let commands = Commands::new(args.clone());
    let command = commands.command.clone();

    if session_context.user_state.root{
        // Execute root commands
        // Handle commands differently when user is in root mode
        if let Ok(res) = command_match(commands.clone(),session_context){
            println!("{}",res)
        }
    } else if !session_context.user_state.root && !session_context.root.allowed_commands.contains(&command) {
        // Execute normal commands
        // Handle commands normally when user is not in root mode
        if let Ok(res) = command_match(commands.clone(),session_context){
            println!("{}",res)
        }
    }else{
        eprintln!("Permission not support")
    }
}

pub fn split(commands: Commands) -> (String,String,Vec<String>){
    let command = commands.command.clone();
    let option = commands.option.clone();
    let arg = commands.arg.clone();
    (command,option,arg)
}

pub fn command_match(commands: Commands,session_context: &mut SessionContext) -> Result<String,std::io::Error>{
    let (command,option,arg) = split(commands.clone());
    match command.is_empty() && option.is_empty(){
        true=>pipe(arg),
        false=>match option.as_str() {
            ">" => stdout_file(commands, session_context),
            _ => execute_command(&command, &option, &arg, session_context),
        }
    }
 
}

#[allow(unused_assignments)]
pub fn execute_command(command: &str, option: &str, arg: &Vec<String>, session_context: &mut SessionContext) -> Result<String, std::io::Error> {
    match command {
        "root" => {
            let output = sudo(session_context);
            output
        },
        "exit" => {
            match option{
                "-all" => {
                    std::process::exit(0);
                },
                _=>{
                    if session_context.user_state.root {
                        session_context.user_state.exit_root();
                        println!("Switched to root mode: {}", session_context.user_state.root);
                    } else {
                        std::process::exit(0);
                    }
                }
            }
            Ok("Exit".to_string())
        },
        "apt"=>match option{ // arg
            "-i"|"-install"=>match arg.is_empty(){
                true=>Ok("Error: Missing parameters".to_string()),
                false=>apt(&arg[0].clone())
            }
            "-u"|"-update"=>match arg.is_empty(){
                true=>Ok("Error: Missing parameters".to_string()),
                false=>update_new(&arg[0].clone())
            }
            _=>Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Error: can't found apt {}", option)))
        },
        "whoami" => Ok(whoami(session_context)),
        "pd" => match option{  // match arg empty
            "-f"|"-fix" => match arg.is_empty(){
                true=>Ok("Error: Missing parameters".to_string()),
                false=>session_context.user.revise_password(&arg[0].clone())
            }
            "-c"|"-check"=>Ok({
                let pd = session_context.user.password.clone();
                let password = decryption(pd.clone());
                password
            }),
            _=>Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Error: can't found pd {}", option))),
        },
        "ll" => {
            let va = ll(&session_context).unwrap();
            Ok(va)
        },
        _ => execute_other_command(command, option, arg),
    }
}

// match has arg's function
pub fn execute_other_command(command: &str, option: &str, arg: &[String]) -> Result<String, std::io::Error> {
    match command {
        "time" => get_time(),
        "history" => history(),
        "ls" | "l" => ls(),
        "grep" => match arg.is_empty(){
            true=>Ok("Error: Missing parameters".to_string()),
            false=>grep(&arg[0], &arg[1])
        }
        "cd" | "rm" | "mkdir" | "touch" | "python" | "html" | "web" | "cat" => match arg.is_empty(){
            true=>Ok("Error: Missing parameters".to_string()),
            false=>turn_file_or_dir(command, &arg[0])
        }
        "tar" => match option {
            "-zxvf" => match arg.is_empty(){
                true=>Ok("Error: Missing parameters".to_string()),
                false=>zxvf(&arg[0], &arg[1]),
            } 
            "-xvf" => match arg.is_empty(){
                true=>Ok("Error: Missing parameters".to_string()),
                false=> xvf(&arg[0]),
            }
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Error: can't found tar {}", option))),
        },
        "rn"|"mv" =>match arg.is_empty(){
            true=>Ok("Error: Missing parameters".to_string()),
            false=>  rename(&arg[0], &arg[1]),
        }
        "cp"=> match arg.is_empty(){
            true=>Ok("Error: Missing parameters".to_string()),
            false=>cp(&arg[0], &arg[1]),
        }
        _ =>{
            let similar = get_similar(&command).join("    ");
            let output = format!("
Error: Can't found this \x1B[31m{}\x1B[0m
    Did you mean?
{}", command,similar
            );
            Ok(output)
        }
    }
}

fn turn_file_or_dir(command: &str, arg: &str) -> Result<String, std::io::Error> {
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