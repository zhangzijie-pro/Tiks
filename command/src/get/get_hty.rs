// root : usersname . password
// apt : download software or txt in web

use crate::commands::command::HISTROY;

pub fn get_last(index: usize) -> (usize,Option<String>){
    let len = HISTROY.lock().unwrap().len();
    if index > len{
        return (1,None);
    }
    let res = &HISTROY.lock().unwrap()[index];
    (0,Some(res.to_string()))
}