use crate::process::thread::ThreadControlBlock;
use crate::signal::Semaphore;

#[derive(Debug,Clone)]
pub struct Process {
    pub pid: usize,
    pub name: String,
    pub state: ProcessState,
    pub thread: ThreadControlBlock,
    pub semaphore: Semaphore,
}

#[derive(Debug,Clone)]
pub enum ProcessState {
    Running,
    Stopped,
}

impl Process {
    pub fn new(pid: usize, name: &str, tcb: ThreadControlBlock,semaphore: Semaphore) -> Self {
        Process {
            pid,
            name: name.to_string(),
            state: ProcessState::Running,
            thread:tcb,
            semaphore
        }
    }

    pub fn get_pid(self) -> usize{
        self.pid
    }

    pub fn stop(&mut self) {
        self.state = ProcessState::Stopped;
        self.semaphore.release()
    }

    pub fn start(&mut self) {
        self.semaphore.acquire();
        self.state = ProcessState::Running;
    }

    pub fn status(&self) -> &ProcessState {
        &self.state
    }
}


// PCB -> Process -> TCB
#[derive(Debug,Clone)]
pub struct ProcessManager {
    pub processes: Vec<Process>,
}

#[allow(dropping_references)]
impl Drop for ProcessManager{
    fn drop(&mut self) {
        for i in self.processes.iter_mut(){
            drop(&i);
            i.stop()
        }
    }
}

impl ProcessManager {
    pub fn new() -> Self {
        ProcessManager {
            processes: Vec::new(),
        }
    }

    pub fn ps(&self) {
        for process in &self.processes {
            println!("PID: {}, Name: {}, State: {:?}", process.pid, process.name, process.status());
        }
    }

    pub fn kill(&mut self, pid: usize) {
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid == pid) {
            process.stop();
        } else {
            println!("Process with PID {} not found", pid);
        }
    }

    pub fn add_process(&mut self, process: Process) {
        self.processes.push(process);
    }

    pub fn start_process(&mut self,pid: usize){
        if let Some(process) = self.processes.iter_mut().find(|p| p.pid==pid){
            process.start();
        }
    }
}