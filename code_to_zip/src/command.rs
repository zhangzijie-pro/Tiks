use std::process::{Command,exit};
use std::path::PathBuf;
use webbrowser;


pub fn command_to_run(command: Option<String>,source_file: Option<PathBuf>){
    if let Some(cmd) = command {
        let status = match cmd.as_str() {
            "python"|"py" => execute_command("python",source_file),
            "java" => execute_command("java",source_file),
            "go" => execute_command("go",source_file),
            "ruby"|"rb" => execute_command("ruby",source_file),
            "c++" => execute_command("c++",source_file),
            "c" => execute_command("c",source_file),
            "node"|"js" => execute_command("node.js",source_file),
            _ => {
                eprintln!("Error: command not support or your computer is not support this language");
                exit(1);
            }
        };

        if let Err(err) = status {
            eprintln!("Command execution failed: {:?}", err);
            exit(1);
        } else if !status.unwrap().success() {
            eprintln!("Command execution failed with non-zero exit code");
            exit(1);
        }
    }else {
        eprintln!("No command support");
        exit(1);
    }
}

fn execute_command(command: &str,source_file: Option<PathBuf>) -> Result<std::process::ExitStatus, std::io::Error> {
    let mut cmd = Command::new(command);
    if let Some(file) = source_file {
        cmd.arg(file);
    }
    cmd.spawn()?.wait()
}

pub fn open_html(source_file: Option<PathBuf>)-> Result<(),std::io::Error>{
    match source_file {
        Some(p) => {
            if let Some(p_str) = p.to_str(){
                let result = webbrowser::open(p_str);

                if let Err(err) = result {
                    eprintln!("Error: {:?}",err);
                }
            }else {
                eprintln!("Error: Path is None. Please provide a valid file path.");
            }
        }
        None => {
            println!("Path is None. Please provide a valid file path.");
        }
    }

    Ok(())
}