use crate::ast::Pipeline;
use crate::builtin;

pub fn execute(pipeline: Pipeline) {

    // For now we only support one command.
    if pipeline.commands.is_empty() {
        return;
    }

    let cmd = &pipeline.commands[0];

    // Check for built-in commands
    if builtin::execute_builtin(cmd) {
        return;
    }

    println!("External command: {:?}", cmd.argv);
}