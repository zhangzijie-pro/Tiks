use std::process::Command;

// run code use python ...
pub fn python(file: Option<&str>) -> Result<String, std::io::Error> {
    if file.is_none(){
        eprintln!("Error: please provide a valid file")
    }
    let mut binding = Command::new("python3");
    let cmd = binding
    .arg(file.unwrap());

    let s = cmd.spawn()?.wait();
    if s.is_err(){
        let help = format!("      
Command 'python' not found, did you mean:
    apt install python
        ");
        return Ok(help);
    }
    Ok("code run over!".to_string())
}


// open web in html
pub fn html(file: Option<&str>) -> Result<String,std::io::Error>{
    match file {
        Some(html) => {
            let s = webbrowser::open(html);
            if s.is_err(){
                return Ok("Error: Can't open".to_string());
            }
        },
        None => {
            eprintln!("Error: Path is None. Please provide a valid file path or web.");
        }
    }
    Ok("open over!".to_string())
}

// others