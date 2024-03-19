
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