use async_trait::async_trait;

use crate::{cache::{Cache, CacheMap, Cache_get}, command::{history_push, rename, turn_dir, turn_file}};

#[async_trait]
pub trait Command {
    async fn analysis(cache:CacheMap,command: Vec<String>);
}

pub struct Commands;
#[async_trait]
impl Command for Commands{
    async fn analysis(cache:CacheMap,commands: Vec<String>){
        for i in &commands{
            history_push(i.to_string());
        }
        let len = commands.len();    
        match len{
            1=> {
                let command = &commands[0];
                if *command=="exit".to_string(){
                    std::process::exit(0)
                }
                let value = <Cache as Cache_get>::cache_get(cache.clone(), command.to_string()).await;
                match value {
                    Some(ref s) => println!("{}",s),
                    None => {
                        eprintln!("Error: Can't found this {}",command);
                    }
                }
            },
            2 => {
                let command = &commands[0];
                let path = &commands[1];
                if let Ok(res) = turn_file(command.clone(), path.clone()) {
                    println!("{}",res);
                }else if let Ok(res) = turn_dir(command.clone(), path.clone()) {
                    println!("{}",res);
                }else{
                    eprintln!("\x1B[32mError: Can't found this: {}\x1B[0m", command);
                }
            },
            3 => {
                let command = &commands[0];
                let source = &commands[1]; 
                let now = &commands[2];
                if *command == "rename".to_string(){
                    let s = rename(source, now).unwrap();
                    println!("{}",s)
                }
            },
            _ =>{
                println!("None")
            }
        }
    }
}