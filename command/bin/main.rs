use command::cache::{Cache,Cache_get,initialize_command_cache};
use command::command::{history, HISTROY, PATH};
use command::start_logo;
use std::env;

#[tokio::main]
async fn main() {
        start_logo::strat_logo();
        let args:Vec<String> =env::args().collect();
        unsafe{
            let path = &args[2];
            let boxed_str: Box<str> = <String as Clone>::clone(&path).into_boxed_str();
            // 将 Box<str> 转换为 'static 引用并泄漏（leak）其所有权
            let static_str: &'static str = Box::leak(boxed_str);
            PATH.unwrap().clone_from(&static_str);
        }
        let command = &args[1];
        let cache2 = initialize_command_cache().await;
        let value = <Cache as Cache_get>::cache_get(cache2.clone(), command.to_string()).await.unwrap();
        if !value.is_empty(){
            println!("{}",value);
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
        cache2.clear();
}

#[cfg(test)]
mod test{
    use super::{Cache,Cache_get};

    #[test]
    fn test_cache(){
        panic!("!")
    }
}