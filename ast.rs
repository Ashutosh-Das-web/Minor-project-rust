#[derive(Debug)]
pub struct Cmd {
    pub argv: Vec<String>,
    pub stdin_from: Option<String>,
    pub stdout_to: Option<String>,
}

#[derive(Debug)]
pub struct Pipeline {
    pub commands: Vec<Cmd>,
}