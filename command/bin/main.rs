// This is a Simple Terimal in RUST
//  Tiks
// there is some error or other suggestions contact me : zzj01262022@163.com
// Cargo run

use command::{cache::initialize_command_cache, commands::{arg::{Command, Commands}, command::{history_push, pwd,whoami}}, root::SessionContext};
use command::start_logo;
use std::io::{self, Write};
use command::get::get_hty::get_last;

#[tokio::main]
async fn main() {
        start_logo::strat_logo();
        let cache = initialize_command_cache().await;
        let mut session_context = SessionContext::new();
        loop {
            let mut args: Vec<String> = Vec::new();
            let mut input = String::new();
            print_prompt(&session_context);
        
            if let Err(err) = io::stdin().read_line(&mut input) {
                eprintln!("Failed to read input: {}", err);
                continue;
            }
        
            let command = input.trim();
            history_push(command.to_string());

            if command.is_empty() {
                continue; // Ignore empty commands
            }
            match command{
                "root" => {
                    session_context.user_state.toggle_root();
                    println!("Switched to root mode: {}", session_context.user_state.root);
                },
                "exit" => {
                    if session_context.user_state.root ==true{
                        session_context.user_state.exit_root();
                        println!("Switched to root mode: {}", session_context.user_state.root);
                    }else {
                        cache.clear();
                        std::process::exit(0);
                    }
                },
                _ =>{
                    if command.parse::<usize>().is_ok(){
                        let (_i,res) = get_last(command.parse::<usize>().unwrap());
                        match res{
                            Some(command) => {
                                args.extend(command.split_whitespace().map(|s| s.to_string()));
                                Commands::handle_command(cache.clone(),args.clone(),&session_context).await;
                            },
                            None =>{
                                continue;
                            }
                        }

                    }else{
                        args.extend(command.split_whitespace().map(|s| s.to_string()));
                        Commands::handle_command(cache.clone(),args.clone(),&session_context).await;
                    }
                }
            }
        }
}

// root
fn print_prompt(session_context: &SessionContext) {
    let mut  whoami = whoami();
    if session_context.user_state.root {
        whoami = "root".to_string();
    }
    
    print!("\x1B[32;1m{}\x1B[0m:\x1B[34m{}>>\x1B[0m ",whoami,pwd()); // Assuming whoami() returns the current user
    io::stdout().flush().unwrap();
}

#[cfg(test)]
mod test{
    #[test]
    fn test_cache(){
        panic!("!")
    }
}