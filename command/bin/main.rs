use command::{arg::Command, cache::{initialize_command_cache, Cache, Cache_get}, command::history, start_logo};
use std::io::Write;
use command::arg::Commands;
use command::command::history_push;
#[tokio::main]
async fn main() {
        start_logo::strat_logo();
        let mut commands: Vec<Vec<String>> = Vec::new();
        let mut command = Vec::new();
        command.push("ls".to_string());
        commands.push(command.clone());
        let mut command1 = Vec::new();
        command1.push("pwd".to_string());
        commands.push(command1.clone());
        let mut command2 = Vec::new();
        command2.push("history".to_string());
        commands.push(command2.clone());
        
        for (i,c) in commands.iter().enumerate(){
            println!("{}",i+1);
            Commands::analysis(c.clone()).await;
        }
        /*command.push("pwd".to_string());
                let cache = initialize_command_cache().await;
                let value = <Cache as Cache_get>::cache_get(cache.clone(), command[1].to_string()).await.unwrap();
        println!("{}",value);*/

/*        command.push("history".to_string());
                let cache = initialize_command_cache().await;
                let value = <Cache as Cache_get>::cache_get(cache.clone(), command[2].to_string()).await.unwrap();
        println!("{}",value);
*/
        /*
        history_push(command[1].to_string());
        history_push(command[2].to_string());*/

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