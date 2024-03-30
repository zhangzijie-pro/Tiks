use self::thread::ThreadControlBlock;

pub mod process;
pub mod add_task;
pub mod thread;


pub fn sleep(tcb:&mut ThreadControlBlock,time: usize){
    tcb.sleep_threads(time)
}