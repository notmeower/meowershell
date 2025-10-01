use std::io;
use std::process::{Command, Stdio};
use meowershell_commands::commands::*;

pub fn execute_line(input: &str) -> io::Result<()> {
    let pipeline: Vec<&str> = input.split('|').map(|s| s.trim()).collect();
    let mut previous_stdout: Option<std::process::ChildStdout> = None;

    for (i, cmd) in pipeline.iter().enumerate() {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() { continue; }

        let command = parts[0];
        let args = &parts[1..];

        let stdin = previous_stdout
            .take()
            .map_or(Stdio::inherit(), |out| Stdio::from(out));
        let stdout = if i == pipeline.len() - 1 { Stdio::inherit() } else { Stdio::piped() };

        match command {
            "echo" => echo::echo(args)?,
            "export" => export::export(args)?,
            "shutdown" => shutdown::shutdown(args)?,
            _ => {
                let mut child = Command::new(command)
                    .args(args)
                    .stdin(stdin)
                    .stdout(stdout)
                    .stderr(Stdio::inherit())
                    .spawn()?;

                if i != pipeline.len() - 1 {
                    previous_stdout = child.stdout.take();
                } else {
                    let status = child.wait()?;
                    if !status.success() {
                        eprintln!("Command '{}' failed with code {:?}", command, status.code());
                    }
                }
            }
        }
    }

    Ok(())
}
