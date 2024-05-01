// This is a Simple Terimal in RUST
//  Tiks
// Provided for your reference and learning. If there are any improvements or errors.
// You can git push to main. Welcome everyone to collaborate and improve together

// There is some error or other suggestions contact me : zzj01262022@163.com
// Cargo run


use command::env::init_env;
use command::run::init_shell;
use command::start_logo::start_logo;
use command::root::new_session;

#[tokio::main]
async fn main() {
        start_logo();
        
        // new user
        let mut session_context = new_session();

        // set os envirment path in Tiks
        init_env();

        // init shell
        init_shell(&mut session_context)
}