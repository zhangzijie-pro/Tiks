use std::io::Write;
use code_to_zip::start_logo::strat_logo;
use code_to_zip::flag::analysis;
//use code_to_zip::build_logo::logo_to_exe;

// 应该将匹配命令的内容另写为一个函数，然后调用，并使用测试模块来测试每个部分是否正常

fn main(){

    //logo_to_exe();
    strat_logo();

    loop {   
        let mut args = Vec::new();
        let mut input = String::new();
        print!("\x1B[32;1m@$>>\x1B[0m ");
        std::io::stdout().flush().unwrap();  // 立即刷新输出缓冲区

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read your line");

        for item in input.trim().split_whitespace() {
            args.push(item.to_string());
        }  
        
         let args = args;

        analysis(&args);      
    } 
}

// test part of code
#[cfg(test)]

mod tests{
    use super::*;
    use std::path::PathBuf;
    use code_to_zip::zip_code::zip_code;
    use code_to_zip::command::*;
    
    //#[ignore = "test"]
    #[test]
    fn test(){
        let test_function = || {
        //let args = Arguments::new(&["-h".to_string()]).unwrap();
        let file = PathBuf::from(r"C:\Users\lenovo\Desktop\rust\zip_code\explain.py");
        let zip_local = "python.zip".to_string();
        
        loop {
            let result = zip_code(file.to_path_buf(), Some(zip_local.to_string()));
            match result {
                Ok(_) => {
                    println!("压缩成功！");
                    break;
                }
                Err(_) => {
                    println!("压缩失败！再次尝试...");
                    continue;
                }
            }
        }
    };
    test_function();
    }

    //#[ignore = "command_test"]
    #[test]
    fn command_test(){
        let test_function = || {
            let command = Some("python".to_string());
            let file = PathBuf::from(r"C:\Users\lenovo\Desktop\rust\zip_code\explain.py");

            command_to_run(command,Some(file));
        };
        test_function();
    }

    //#[ignore = "flag"]
    #[test]
    fn flag(){
        let tets_function = || {
            let args = vec!["-h".to_string()];
            analysis(&args);
        };
        tets_function();
    }

    //#[ignore = "flag_command"]
    #[test]
    fn flag_command(){
        let tets_function = || {
            let args = vec!["-r".to_string(),r"C:\Users\lenovo\Desktop\rust\zip_code\explain.py".to_string(),"python".to_string(),"python.zip".to_string()];
            analysis(&args);
        };
        tets_function();
    }

    #[test]
    fn zip_way(){
        let tets_function = || {
            let args = vec!["-z".to_string(),"zip".to_string(),r"C:\Users\lenovo\Desktop\rust\zip_code\explain.py".to_string(),"python.zip".to_string()];
            analysis(&args);
        };
        tets_function();
    }

    #[test]
    fn zip_way_tar(){
        let tets_function = || {
            let args = vec!["-z".to_string(),"tar".to_string(),r"C:\Users\lenovo\Desktop\rust\ip_sniffer".to_string(),"port.tar.gz".to_string()];
            analysis(&args);
        };
        tets_function();
    }
    #[ignore = "run_html"]
    #[test]
    fn run_html(){
        let test_function = || {
            let files = Some(PathBuf::from(r"C:\Users\lenovo\Desktop\rust\zip_code\hello.html"));
            let result = open_html(files);
            match result {
                Ok(()) => (),
                Err(s) => {
                    println!("Error: {:?}",s);
                }
            }
        };
        test_function();
    }

    #[test]
    fn flag_command_test_html(){
        let tets_function = || {
            let args = vec!["-r".to_string(),r"C:\Users\lenovo\Desktop\rust\zip_code\hello.html".to_string(),"html".to_string(),"html.zip".to_string()];
            analysis(&args);
        };
        tets_function();
    }

}