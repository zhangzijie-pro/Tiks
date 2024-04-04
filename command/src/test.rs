#[cfg(test)]
mod tests {
    use crate::{commands::command::echo_print, env::set_env};

    #[test]
    fn detection_func() {
        let input = "$HOME";
        let (ouput_n,output_s) = echo_print(input.to_string());
        assert_eq!(0,ouput_n);
        assert_eq!("/home/zzj",output_s)
    }

    #[test]
    fn env_set() {
        let (n,s) = set_env();
        assert_eq!(n,0);
        assert_eq!("Env set over".to_string(),s)
    }

    #[test]
    fn your_function(){

    }
}
