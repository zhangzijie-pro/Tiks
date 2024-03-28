use super::{process::ProcessManager, thread::ThreadControlBlock};


pub fn ps(tcb: ThreadControlBlock,pcb: ProcessManager){
    pcb.list_running_processes();
    tcb.list_running_threads();
}

pub fn ps_t(tcb: ThreadControlBlock){
    tcb.list_running_threads();
}

pub fn ps_p(pcb: ProcessManager){
    pcb.list_running_processes();
}

pub fn kill_t(tid:usize,tcb:&mut ThreadControlBlock){
    tcb.stop_thread(tid)
}

pub fn kill_p(pid:usize,pcb:&mut ProcessManager){
    pcb.stop_process(pid);
}

pub fn sleep(tcb:&mut ThreadControlBlock,time: usize){
    tcb.sleep_threads(time)
}
