use std::io::Write;

use async_trait::async_trait;

use crate::{cache::{initialize_command_cache, Cache, Cache_get}, command::{history_push, PATH}};

#[async_trait]
pub trait Command {
    async fn analysis(command: Vec<String>);
}

pub struct Commands;

impl Commands {
    pub fn new() -> Vec<String>{
        let mut args: Vec<String> = Vec::new();
        let mut input = String::new();
        print!("\x1B[32;1m>>\x1B[0m ");
        std::io::stdout().flush().unwrap();  // 立即刷新输出缓冲区

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read your line");

        for item in input.trim().split_whitespace() {
            args.push(item.to_string());
        }
        args
    }
}
// spilt
#[async_trait]
impl Command for Commands{
    async fn analysis(commands: Vec<String>){
        let len = commands.len();
        if len==1{
            let command = &commands[0];
            let cache = initialize_command_cache().await;
            let value = <Cache as Cache_get>::cache_get(cache.clone(), command.to_string()).await.unwrap();
/*            if !command.is_empty() {
                println!("{}",value);
            }*/
            println!("{}",value);
        }else{
            let command = &commands[0];
            unsafe{
                let path = &commands[1];
                let boxed_str: Box<str> = <String as Clone>::clone(&path).into_boxed_str();
                // 将 Box<str> 转换为 'static 引用并泄漏（leak）其所有权
                let static_str: &'static str = Box::leak(boxed_str);
                PATH.unwrap().clone_from(&static_str);
            }
            let cache = initialize_command_cache().await;
            let value = <Cache as Cache_get>::cache_get(cache.clone(), command.to_string()).await.unwrap();
            if !command.is_empty() {
                println!("{}",value);
            }
        }
        for (i,command) in commands.iter().enumerate(){
            history_push(command.to_string());
        }
    }
}