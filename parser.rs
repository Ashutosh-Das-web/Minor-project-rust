use crate::ast::{Cmd, Pipeline};

pub fn parse(_tokens: Vec<String>) -> Pipeline {
    Pipeline {
        commands: vec![
            Cmd {
                argv: vec!["cat".to_string(), "file.txt".to_string()],
                stdin_from: None,
                stdout_to: None,
            },
            Cmd {
                argv: vec!["grep".to_string(), "rs".to_string()],
                stdin_from: None,
                stdout_to: Some("out.txt".to_string()),
            },
        ],
    }
}