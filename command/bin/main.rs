use command::{arg::Command, cache::initialize_command_cache, command::{history_push, pwd, whoami}, start_logo};
use std::io::{self, Write};
use command::arg::Commands;

#[tokio::main]
async fn main() {
        start_logo::strat_logo();
        let cache = initialize_command_cache().await;
        loop {
            let mut args: Vec<String> = Vec::new();
            let mut input = String::new();
            print!("\x1B[32;1m{}\x1B[0m:\x1B[34m{}>>\x1B[0m ",whoami(),pwd());
            io::stdout().flush().unwrap();
        
            if let Err(err) = io::stdin().read_line(&mut input) {
                eprintln!("Failed to read input: {}", err);
                continue;
            }
        
            let command = input.trim();
            history_push(command.to_string());
            if command.is_empty() {
                continue; // Ignore empty commands
            }
            args.extend(command.split_whitespace().map(|s| s.to_string()));
            Commands::analysis(cache.clone(),args.clone()).await;
        }

        // 命令行内容
        /*let command = String::from("pwd");
        let command2 = String::from("ls");
        let command3 = String::from("cd");
        // 初始化缓存内容
        let cache2 = initialize_command_cache().await;
        let value_cd = <Cache as Cache_get>::cache_get(cache2.clone(), command3).await.unwrap();
        let value = <Cache as Cache_get>::cache_get(cache2.clone(), command).await.unwrap();
        let value_ls = <Cache as Cache_get>::cache_get(cache2.clone(), command2).await.unwrap();
        println!("cd is successed :{}",value_cd);
        println!("pwd : {:?}",value);
        println!("ls: {:?}",value_ls);*/
        // 清理缓存数据
}

#[cfg(test)]
mod test{
    #[test]
    fn test_cache(){
        panic!("!")
    }
}