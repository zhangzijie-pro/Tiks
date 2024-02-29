use async_trait::async_trait;

use crate::{cache::{initialize_command_cache, Cache, Cache_get}, command::PATH};

#[async_trait]
pub trait Command {
    async fn analysis(command: Vec<String>) -> String;
}

pub struct Commands;
// spilt
#[async_trait]
impl Command for Commands{
    async fn analysis(commands: Vec<String>) -> String{
        let len = commands.len();
        if len==1{
            let command = &commands[0];
            let cache = initialize_command_cache().await;
            let value = <Cache as Cache_get>::cache_get(cache.clone(), command.to_string()).await.unwrap();
            if !command.is_empty() {
                println!("{}",value);
            }
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
        String::new()
    }
}