use crate::process::thread::ThreadControlBlock;

#[derive(Debug)]
pub struct Process {
    pid: usize,
    name: String,
    state: ProcessState,
    pub thread: ThreadControlBlock
}

#[derive(Debug)]
pub enum ProcessState {
    Running,
    Stopped,
}

impl Process {
    pub fn new(pid: usize, name: &str, tcb: ThreadControlBlock) -> Self {
        Process {
            pid,
            name: name.to_string(),
            state: ProcessState::Running,
            thread:tcb
        }
    }

    pub fn get_pid(self) -> usize{
        self.pid
    }

    pub fn stop(&mut self) {
        self.state = ProcessState::Stopped;
    }

    pub fn start(&mut self) {
        self.state = ProcessState::Running;
    }

    pub fn status(&self) -> &ProcessState {
        &self.state
    }
}


// PCB -> Process -> TCB
#[derive(Debug)]
pub struct ProcessManager {
    pub processes: Vec<Process>,
}

impl ProcessManager {
    pub fn new() -> Self {
        ProcessManager {
            processes: Vec::new(),
        }
    }

    pub fn add_process(&mut self, process: Process) {
        self.processes.push(process);
    }

    pub fn stop_process(&mut self, pid: usize) {
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.stop();
        } else {
            println!("Process with PID {} not found", pid);
        }
    }

    pub fn start_process(&mut self, pid: usize) {
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.start();
        } else {
            println!("Process with PID {} not found", pid);
        }
    }

    pub fn list_processes(&self) {
        for process in &self.processes {
            println!("PID: {}, Name: {}, State: {:?}", process.pid, process.name, process.status());
        }
    }
}