use crate::get::priority::CommandPriority;

use super::thread::Thread;
use super::thread::ThreadControlBlock;
use super::process::Process;
use super::process::ProcessManager;

/*
+-------------------------------------+
|              Process 1              |
|  (PCB: Process Control Block)       |
|  Command:                            |
|  +---------------------------------+ |
|  |           Thread 1 (TCB)        | |
|  |  Command:                       | |
|  +---------------------------------+ |
|  +---------------------------------+ |
|  |           Thread 2 (TCB)        | |
|  |  Command:                       | |
|  +---------------------------------+ |
+-------------------------------------+
|              Process 2              |
|  (PCB: Process Control Block)       |
|  Command:                            |
|  +---------------------------------+ |
|  |           Thread 1 (TCB)        | |
|  |  Command:                       | |
|  +---------------------------------+ |
+-------------------------------------+
|              Process 3              |
|  (PCB: Process Control Block)       |
|  Command:                            |
|  +---------------------------------+ |
|  |           Thread 1 (TCB)        | |
|  |  Command:                       | |
|  +---------------------------------+ |
+-------------------------------------+
*/

pub fn add_command_to_thread(tid: usize, name: String,priority: CommandPriority) -> ThreadControlBlock{
    let thread = Thread::new(tid, name, priority);
    let mut tcb = ThreadControlBlock::new();
    tcb.add_thread(thread);
    tcb
}


pub fn add_thread_to_process(pid:usize,name: String,tcb:ThreadControlBlock) -> ProcessManager{
    let process = Process::new(pid, &name,tcb);
    let mut pcb = ProcessManager::new();
    pcb.add_process(process);
    
    pcb
}