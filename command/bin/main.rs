// This is a Simple Terimal in RUST
//  Tiks
// Provided for your reference and learning. If there are any improvements or errors.
// You can git push to main. Welcome everyone to collaborate and improve together

// There is some error or other suggestions contact me : zzj01262022@163.com
// Cargo run

use command::get::get_hty::get_last;
use command::root::SessionContext;
use command::commands::arg::handle_command;
use command::commands::command::{history_push,pwd};
use command::start_logo;
use std::io::{self, Write};


#[tokio::main]
async fn main() {
        start_logo::start_logo();
        let mut session_context = SessionContext::new();
        loop {
            let mut args: Vec<String> = Vec::new();
            let mut input = String::new();
            print_prompt(&mut session_context);
        
            if let Err(err) = io::stdin().read_line(&mut input) {
                eprintln!("Failed to read input: {}", err);
                continue;
            }
        
            let command = input.trim();
            history_push(command.to_string());

            if command.is_empty() {
                continue; // Ignore empty commands
            }else if command.parse::<usize>().is_ok(){
                let (_i,res) = get_last(command.parse::<usize>().unwrap());
                match res{
                    Some(command) => {
                        args.extend(command.split_whitespace().map(|s| s.to_string()));
                        handle_command(args.clone(),&mut session_context).await;
                    },
                    None =>{
                        continue;
                    }
                }

            }else{
                args.extend(command.split_whitespace().map(|s| s.to_string()));
                handle_command(args.clone(),&mut session_context).await;
            }
        }
}

// root
fn print_prompt(session_context: &mut SessionContext) {
    let mut whoami = session_context.get_username();
    if session_context.user_state.root{
        whoami="root".to_string()
    }
    let input = format!("\x1B[32;1m{}\x1B[0m:\x1B[34m{}>>\x1B[0m ",whoami,pwd()); // Assuming whoami() returns the current user
    print!("{}",input);
    io::stdout().flush().unwrap();
}