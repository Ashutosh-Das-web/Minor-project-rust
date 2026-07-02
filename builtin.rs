use crate::ast::Cmd;
use std::env;
pub fn execute_builtin(cmd: &Cmd) -> bool {
    if cmd.argv.is_empty() {
        return false;
    }

    match cmd.argv[0].as_str() {
        "exit" => {
            println!("thanks for using minish");
            std::process::exit(0);
        }

        "pwd" => {
            match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("pwd: {}", e),
            }
            true
        }

        "cd" => {
            // cd with no arguments -> HOME
            let target = if cmd.argv.len() > 1 {
                cmd.argv[1].clone()
            } else {
                env::var("HOME").unwrap_or_else(|_| ".".to_string())
            };

            if let Err(e) = env::set_current_dir(&target) {
                eprintln!("cd: {}", e);
            }

            true
        }

        _ => false,
    }
}