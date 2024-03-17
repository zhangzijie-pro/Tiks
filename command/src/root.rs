// build root
pub struct UserState{
    pub root: bool
}
pub struct Root {
    pub allowed_commands: Vec<String>,
}

impl UserState {
    pub fn new() -> UserState{
        let root = false;
        UserState{
            root
        }
    }

    pub fn toggle_root(&mut self) -> Self{
        self.root = !&self.root;
        Self{
            root:self.root
        }
    }

    pub fn exit_root(&mut self){
        self.root=false;
    }
}

use std::io::{self,Write};
use std::fs;
use std::process::Command;
use dirs;

pub struct User{
    pub username: String,
    pub password: String,
    has_set_password: bool,
}

impl User{
    pub fn new(username: String, password: String, has_set_password: bool) -> User {
        User {
            username,
            password,
            has_set_password,
        }
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
        self.has_set_password = true;
    }

    pub fn revise_password(&self, password: &str) -> Result<String, std::io::Error> {
        let _ = self.password == password;
        Ok("revises over!".to_string())
    }

    pub fn has_set_password(&self) -> bool {
        self.has_set_password
    }

    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let serialized_user = format!("{}:{}", self.username, self.password);
        fs::write(filename, serialized_user)
    }

    pub fn load_from_file(filename: &str) -> io::Result<User> {
        let serialized_user = fs::read_to_string(filename)?;
        let mut parts = serialized_user.split(':');
        let username = parts.next().unwrap_or("").to_string();
        let password = parts.next().unwrap_or("").to_string();
        Ok(User {
            username,
            password,
            has_set_password: true,
        })
    }
}


pub struct SessionContext {
    pub root: Root,
    pub user_state: UserState,
    pub user: User
}

impl SessionContext{
    pub fn new() -> SessionContext{
        let root = Root{
            allowed_commands: vec![
            "mkdir".to_string(),"rm".to_string(),"rn".to_string(),
            "touch".to_string(),"cat".to_string(),"mv".to_string(),
            "pd".to_string()], // add command in root
        };
        let userstate = UserState::new(); //false

        let home_dir = dirs::home_dir().expect("Failed to get home directory");
        let binding = home_dir.join(".Tiks").join("tiks");
        let user_file_path = binding.as_os_str().to_str().unwrap();

        let user = match User::load_from_file(&user_file_path){
            Ok(res) =>{
                 if !res.has_set_password() {
                    let mut username = String::new();

                    get_username(&mut username);
                    let password = get_password();
    
                    let user = User::new(username, password, false);
                    user.save_to_file(&user_file_path).expect("Failed to save user");
                    user
                }else{
                    res
                }
            },
            Err(_) =>{
                init_setup();
                let mut user = String::new();

                get_username(&mut user);
                let password = get_password();

                let user = User::new(user, password, false);
                user.save_to_file(&user_file_path).expect("Failed to save user");
                user
            }
        };
        
        SessionContext{
            root, 
            user_state: userstate,
            user
        }
    }
    pub fn get_username(&self) -> String{
        let s = self.user.username.clone().trim().to_string();
        s
    }
}

fn get_username(user: &mut String){
    print!("Enter username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(user).unwrap();
}

use rpassword;
fn get_password() -> String{
    loop {
        println!("Enter password:");
        let pd = rpassword::read_password().unwrap();
        println!("Enter password again:");
        let pd_again = rpassword::read_password().unwrap();
        if pd == pd_again{
            return pd
        }else {
            eprint!("Error: password isn't same\n");
            continue;
        }
    }
}

// for each os
fn init_setup(){
    Command::new("bash")
    .arg("setup.sh")
    .spawn()
    .expect("Error: Can't setup");
}
