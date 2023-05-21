use crate::RestoreCommand;

pub fn run(cmd: RestoreCommand) {
    println!("Restoring index {} from {} using backup files from {}",
        cmd.index, cmd.url, cmd.file);

    // TODO: Implement the actual restore process
}
