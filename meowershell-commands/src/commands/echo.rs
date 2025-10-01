use std::io::{self, Write};
use std::env;

pub fn echo(args: &[&str]) -> io::Result<()> {
    let expanded: Vec<String> = args.iter().map(|arg| {
        if arg.starts_with('$') {
            let key = &arg[1..];
            env::var(key).unwrap_or_default()
        } else {
            arg.to_string()
        }
    }).collect();

    println!("{}", expanded.join(" "));
    io::stdout().flush()
}
