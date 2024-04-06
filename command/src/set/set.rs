use crate::commands::command::{get_time, HISTROY};
use chrono::{DateTime, Datelike, Local, Timelike};

use std::{fs::OpenOptions, io::Write, path::Path, time::Duration};

pub fn get_last(index: usize) -> (usize,Option<String>){
    let len = HISTROY.lock().unwrap().len();
    if index > len{
        return (1,None);
    }
    let res = &HISTROY.lock().unwrap()[index];
    (0,Some(res.to_string()))
}

fn turn_time(du: Duration) -> String{
    let created: DateTime<Local> = Local::now() - du;

    let month_abbrev = match created.month() {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "Invalid",
    };

    let day = created.day();
    let hour = created.hour();
    let minute = created.minute();

    let output = format!("{} {:02} {:02}:{:02}", month_abbrev, day, hour, minute);
    output
}

pub fn file_create_time(path: &str) -> String{
    let mut time = String::new();
    if let Ok(metadata) = std::fs::metadata(&path) {
        if let Ok(created) = metadata.created() {
            let du = created.elapsed().unwrap();
            time = turn_time(du);
        }
    }
    time
}


// get similar command
pub fn get_similar(arg: &str) -> Vec<String>{
    let commands = vec!["ls","pwd","pd","history","whoami","help","ll","cd","mv","cp","rn","tar","rm","mkdir","touch","python","html","web","cat","exit","sudo","apt","ps","sleep","kill"];
    let mut output = Vec::new();
    let threshold = 1;
    for command in commands {
        if levenshtein_distance(arg, command) <= threshold{
            output.push(command.to_string())
        }
    }
    output
}

fn levenshtein_distance(arg:&str,command:&str) -> usize{
    let len1 = arg.chars().count();
    let len2 = command.chars().count();

    let mut dp = vec![vec![0;len2+1];len1+1];

    for i in 0..=len1 {
        dp[i][0] = i;
    }

    for j in 0..=len2 {
        dp[0][j] = j;
    }

    for (i,c) in arg.chars().enumerate(){
        for (j,c2) in command.chars().enumerate(){
            let cost = if c==c2{0}else{1};
            dp[i + 1][j + 1] = *[
                dp[i][j + 1] + 1,
                dp[i + 1][j] + 1,
                dp[i][j] + cost,
            ]
            .iter()
            .min()
            .unwrap();
        }
    }
    dp[len1][len2]

}


// write error.log
fn get_home_err() -> String{
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let binding = home_dir.join(".Tiks").join("error.log");
    let res = binding.as_os_str().to_str().unwrap();
    res.to_string()
}

pub fn error_log(err: String){
    let time = get_time().unwrap().1;
    let home_path = get_home_err();
    let path = Path::new(&home_path);

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!("Failed to create directory: {}", e);
                return;
            }
        }
    }

    let mut file = match OpenOptions::new().append(true).create(true).open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return;
        }
    };

    let error = format!("[{}]: {}\n", time, err);
    let write_err = error.as_bytes();

    if let Err(e) = file.write_all(write_err) {
        eprintln!("Failed to write to file: {}", e);
        return;
    }
    // error.log -> $HOME/.Tiks/error.log -> [time]: [Error]
}

pub const MB:f64 = 1024.0*1024.0;

use std::fs::metadata;
use std::io;

fn get_file_size(file:&str) -> f64{
    let mut size = 0;
    if let Ok(s) = metadata(file){
        size = s.len();
    }

    size as f64
}

type MB = f64;

fn byte_to_mb(size: f64) -> MB{
    let new_size = size/MB;
    new_size
}

pub fn max_size_file(path: &str) -> Result<(),&'static str>{
    let size = get_file_size(path);
    let mb = byte_to_mb(size); // MB

    if mb > 128_f64{
        loop {
            let mut choice = String::new();
            println!("This file is more than 256MB, Do you want to read? [y/n]");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut choice).unwrap();
            if choice.as_str() == "y"{
                break;
            }else{
                return Err("Canel read");
            }
        }
    }

    Ok(())
}