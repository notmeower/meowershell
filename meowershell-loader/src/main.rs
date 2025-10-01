use std::io::{self, Write};
use meowershell_interpreter::execute_line;
use meowershell_config::Config;

fn main() -> io::Result<()> {
    let config = Config::load()?;

    for cmd in &config.init_commands {
        if let Err(err) = execute_line(cmd) {
            eprintln!("Init script error: {}", err);
        }
    }

    loop {
        print!("{}", config.get_ps1());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let trimmed = input.trim();

        if trimmed == "exit" {
            break Ok(());
        }

        if let Err(err) = execute_line(trimmed) {
            eprintln!("error: {}", err);
        }
    }
}
