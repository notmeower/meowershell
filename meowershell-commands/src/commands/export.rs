use std::env;
use std::io;

pub fn export(args: &[&str]) -> io::Result<()> {
    for arg in args {
        if let Some((key, value)) = arg.split_once('=') {
            unsafe {
                env::set_var(key, value);
            }
        } else {
            eprintln!("export: invalid format, expected KEY=VALUE");
        }
    }
    Ok(())
}