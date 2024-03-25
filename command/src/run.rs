use crate::process::process::ProcessManager;
use crate::process::task_command::{kill, ps, sleep};
use crate::process::thread::ThreadControlBlock;
use crate::get::priority::get_priority;
use crate::process::add_task::{add_command_to_thread,add_thread_to_process};
use crate::root::SessionContext;

use std::sync::atomic::{AtomicUsize, Ordering};

use crate::commands::arg::{command_match, Commands};

static NEXT_TID: AtomicUsize = AtomicUsize::new(1);
static NEXT_PID: AtomicUsize = AtomicUsize::new(1);

pub fn handle_command(args: Vec<String>) -> (Commands,ThreadControlBlock,ProcessManager,usize) {
    let commands = Commands::new(args.clone());
    let command = commands.command.clone();
    let priority = get_priority(command.as_str());

    let tid = NEXT_TID.fetch_add(1, Ordering::SeqCst);
    let tcb = add_command_to_thread(tid, command.clone(), priority);

    let pid = NEXT_PID.fetch_add(1, Ordering::SeqCst);
    let pcb = add_thread_to_process(pid, command.clone(),tcb.clone());

    (commands,tcb,pcb,pid)
}

pub fn run(args: Vec<String>,session_context: &mut SessionContext){
    let (commands,mut tcb,mut pcb,pid) = handle_command(args);
    let tid = tcb.get_highest_priority_thread().unwrap();

    match commands.command.as_str(){
        "ps" =>  {ps(tcb.clone());println!("\nTCB:\n{:?}\n\n",tcb);println!("PCB:\n{:?}\n\n",pcb)},
        "kill" => kill(tid, &mut tcb),
        "sleep" => sleep(&mut tcb, commands.arg[0].parse::<usize>().unwrap()),
        _ =>{
            // start process and thread
            tcb.start_thread(tid); pcb.start_process(pid);
            if session_context.user_state.root{
                // Execute root commands
                // Handle commands differently when user is in root mode
                if let Ok(res) = command_match(commands, session_context){
                    let status = res.0.clone();
                    let result = res.1.clone();
                    if status==0{
                        println!("[{}] Done\n{}",tid,result);
                        tcb.stop_thread(tid);
                        pcb.stop_process(pid);
                    }else{
                        println!("{:?}",res)
                    }
                }
            } else if !session_context.user_state.root && !session_context.root.allowed_commands.contains(&commands.command) {
                // Execute normal commands
                // Handle commands normally when user is not in root mode
                if let Ok(res) = command_match(commands, session_context) {
                    let status = res.0.clone();
                    let result = res.1.clone();
                    if status==0{
                        println!("[{}] Done\n{}",tid,result);
                        tcb.stop_thread(tid);
                        pcb.stop_process(pid);
                    }else{
                        println!("{:?}",res)
                    }
                }

    }else{
        eprintln!("Permission not support")
    }
        }
    }
    
}