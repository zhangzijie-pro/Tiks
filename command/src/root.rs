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

pub struct SessionContext {
    pub root: Root,
    pub user_state: UserState,
}

impl SessionContext{
    pub fn new() -> SessionContext{
        let root = Root{
            allowed_commands: vec!["mkdir".to_string(),"rm".to_string(),"rn".to_string(),"touch".to_string(),"cat".to_string()],
        };
        let userstate = UserState::new(); //false

        SessionContext{
            root, 
            user_state: userstate
        }
    }
}