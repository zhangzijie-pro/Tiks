use crate::commands::command::*;
use crate::priority::{get_priority, CommandPriority};
use crate::set::set::{error_log, get_last};
use crate::process::process::ProcessManager;
use crate::process::sleep;
use crate::process::thread::ThreadControlBlock;
use crate::process::add_task::{add_command_to_thread,add_thread_to_process};
use crate::root::SessionContext;
use crate::signal::semaphore_new;

use std::io::{self, Write};
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::commands::arg::{command_match, split, Commands};

static NEXT_TID: AtomicUsize = AtomicUsize::new(1);
static NEXT_PID: AtomicUsize = AtomicUsize::new(1);


pub fn handle_command(args: Vec<String>) -> (Commands,usize,usize,CommandPriority) {
    let commands = Commands::new(args.clone());
    let command = commands.command.clone();
    let priority = get_priority(command.as_str());

    let tid = NEXT_TID.fetch_add(1, Ordering::SeqCst);

    let pid = NEXT_PID.fetch_add(1, Ordering::SeqCst);

    (commands,pid,tid,priority)
}


pub fn run(args: Vec<String>,session_context: &mut SessionContext){
    let (commands,pid,tid,priority) = handle_command(args);
    let semaphore = semaphore_new();
    let mut tcb = ThreadControlBlock::new();
    let mut pcb = ProcessManager::new();


    let (command,_option,arg) = split(commands.clone());

    add_command_to_thread(tid, command.clone(), priority, &mut tcb);
    add_thread_to_process(pid, command.clone(), tcb.clone(), semaphore, &mut pcb);
    
    let priority_tid = tcb.get_highest_priority_thread().unwrap();
    tcb.start_thread(priority_tid);
    pcb.start_process(pid);

    
    match command.as_str(){
        "ps" => pcb.ps(),
        "kill" => match arg.is_empty(){
            true => {},
            _flase => pcb.kill(arg[0].parse::<usize>().unwrap())
        },
        "sleep" => sleep(&mut tcb, commands.arg[0].parse::<usize>().unwrap()),
        _ =>{
            // start process and thread
            if session_context.user_state.root.check_permission(){
                // Execute root commands
                // Handle commands differently when user is in root mode
                if let Ok(res) = command_match(commands, session_context){
                    let status = res.0.clone();
                    let result = res.1.clone();
                    if status==0{
                        println!("[{}] Done\n{}",tid,result);
                        pcb.kill(pid);
                        tcb.stop_thread(priority_tid);
                    }else{
                        error_log(res.1.clone());
                        println!("{}",res.1);
                    }
                }
            } else if !session_context.user_state.root.check_permission() && !session_context.root.allowed_commands.contains(&commands.command) {
                // Execute normal commands
                // Handle commands normally when user is not in root mode
                if let Ok(res) = command_match(commands, session_context) {
                    let status = res.0.clone();
                    let result = res.1.clone();
                    if status==0{
                        println!("[{}] Done\n{}",tid,result);
                        pcb.kill(pid);
                        tcb.stop_thread(priority_tid);
                    }else{
                        error_log(res.1.clone());
                        println!("{}",res.1);
                    }
                }
            }else{
                eprintln!("Permission not support")
            }
        }
    }
    
}


pub fn init_shell(session_context: &mut SessionContext){
    loop {
        let mut args: Vec<String> = Vec::new();
        let mut input = String::new();
        print_prompt(session_context);
    
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
                    run(args.clone(),session_context);
                },
                None =>{
                    continue;
                }
            }
        }else{
            args.extend(command.split_whitespace().map(|s| s.to_string()));
            if args.contains(&"&&".to_string()){
                and(args.clone(), session_context)
            }else if args.contains(&"|".to_string()){
                let s = pipe(args).unwrap();
                println!("{}",s.1)
            }else if args.contains(&"&".to_string()){
                priority_run(args.clone(), session_context)
            }else{
                run(args.clone(),session_context);
            }
        }
    }
}

// root
fn print_prompt(session_context: &mut SessionContext) {
    let mut whoami = session_context.get_username();
    if session_context.user_state.root.check_permission(){
        whoami="root".to_string()
    }
    let pwd = pwd().unwrap().1;
    let input = format!("\x1B[32;1m{}\x1B[0m:\x1B[34m{}>>\x1B[0m ",whoami,pwd); // Assuming whoami() returns the current user
    print!("{}",input);
    io::stdout().flush().unwrap();
}