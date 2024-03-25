pub const STATUE_CODE:usize=0;


pub fn missing_pattern() -> (usize, String) {
    (104, "Error: Missing parameters".to_string())
}

pub fn exit_code() -> (usize, String) {
    (0, "Exit".to_string())
}

pub fn run_code_er() -> (usize, String) {
    (103, "Error: code error".to_string())
}

pub fn run_code() -> (usize, String) {
    (0, "Running...".to_string())
}

pub fn empty_file() -> (usize, String) {
    (105, "Error: File is not exist".to_string())
}

pub fn empty_dir() -> (usize, String) {
    (105, "Error: Dir is not exist".to_string())
}

pub fn pipe_err() -> (usize,String){
    (110,"Error: Meaningless".to_string())
}