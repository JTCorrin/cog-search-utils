use crate::BackupCommand;

pub fn run(cmd: BackupCommand) {
    println!("Backing up index {} from {} with {} workers and a cadence of {} documents to file {}",
        cmd.index, cmd.url, cmd.workers, cmd.cadence, cmd.file);

    // TODO: Implement the actual backup process
}
