use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // TODO: Is u32 the appropriate type?
    /// PID
    #[arg(short, long)]
    pid: Option<i32>,
    /// Command to execute
    #[arg()]
    command: Option<String>,
    /// Arguments
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    arguments: Vec<String>,
}

fn main() {
    let cli = Args::parse();

    if let Some(pid) = cli.pid {
        let process = procfs::process::Process::new(pid).unwrap();

        while let Ok(status) = process.stat() {
            // https://docs.rs/procfs/latest/procfs/process/enum.ProcState.html
            let state = status.state().unwrap();
            let executable = status.comm;

            println!("{executable}: {:?}", state);

            thread::sleep(Duration::from_millis(500));
        }

        println!("Process finished!");
    }

    if let Some(command) = cli.command {
        println!("Command: {} {}\n", command, cli.arguments.join(" "));

        let mut child = Command::new(command)
            .args(&cli.arguments)
            .stdin(Stdio::null())
            .spawn()
            .expect("Failed to execute command");
        //.id()

        let process = procfs::process::Process::new(child.id() as i32).unwrap();

        // Old
        /*while let Ok(None) = child.try_wait() {
            println!("Still running...");
            let vmrss = process.status().unwrap().vmrss.unwrap() as f64;
            println!("VMRSS: {} MiB", vmrss / 1024.0);
            println!("Peak: {}", process.status().unwrap().vmpeak.unwrap());
            thread::sleep(Duration::from_millis(500));
        }*/

        while let Ok(status) = process.status() {
            println!("Still running...");
            let vmrss = status.vmrss.unwrap() as f64;
            println!("VMRSS: {} MiB", vmrss / 1024.0);
            println!("Peak: {}", status.vmpeak.unwrap());
            thread::sleep(Duration::from_millis(500));
        }
    }
}
