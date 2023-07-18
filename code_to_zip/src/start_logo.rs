pub fn strat_logo(){
    println!(
        "\x1B[31m{}\x1B[0m",
        r#"
  _______ _ _      
 |__   __(_) |     
    | |   _| |_ ___
    | |  | | __/ __|
    | |  | | |_\__ \
    |_|  |_|\__|___/
                    
"#
    );
println!("author: zijie Zhang");
println!("version: 1.0.1");
println!("If you have any questions, please contact me: zzj01262022@163.com");
println!("Here you can compress your file items for easy transfer and use");
println!(
        "Usage: \n\r'-h' or '-help': you can see more help for you \n\r'-f': it's a file path for you and you can fill here to zip your file \n\r'-w': you can run your code \n\r'-z': you can choice your compress mode "
    );
}
