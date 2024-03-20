
use crate::commands::command::HISTROY;
use chrono::{DateTime, Datelike, Local, Timelike};

use std::time::Duration;

pub fn get_last(index: usize) -> (usize,Option<String>){
    let len = HISTROY.lock().unwrap().len();
    if index > len{
        return (1,None);
    }
    let res = &HISTROY.lock().unwrap()[index];
    (0,Some(res.to_string()))
}

fn turn_time(du: Duration) -> String{
    // 将时间间隔转换为 DateTime 对象
    let created: DateTime<Local> = Local::now() - du;

    // 将月份转换为对应的英文缩写//Feb 08 20:23
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

    // 获取日、小时、分钟
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
    let commands = vec!["ls","pwd","pd","history","whoami","help","ll","cd","mv","cp","rn","tar","rm","mkdir","touch","python","html","web","cat","exit","root","apt"];
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