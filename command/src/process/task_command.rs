use super::thread::ThreadControlBlock;


pub fn ps(tcb: ThreadControlBlock){
    tcb.list_threads()
}

pub fn kill(tid:usize,tcb:&mut ThreadControlBlock){
    tcb.stop_thread(tid)
}

pub fn sleep(tcb:&mut ThreadControlBlock,time: usize){
    tcb.sleep_threads(time)
}
