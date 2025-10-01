use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::Command;
use hostname;

pub struct Config {
    pub ps1_template: String,
    pub init_commands: Vec<String>,
}

impl Config {
    pub fn load() -> io::Result<Self> {
        let mut ps1_template = String::new();
        let mut init_commands = Vec::new();

        let home = env::var("HOME").unwrap_or_else(|_| ".".into());
        let config_path = PathBuf::from(&home).join(".meowerrc");

        if !config_path.exists() {
            let mut file = fs::File::create(&config_path)?;
            writeln!(file, "# MeowerShell configuration file")?;
            writeln!(file, "PS1=\"{{user}}@{{hostname}} ~# {{pwd}}: \"")?;
        }

        let file = fs::File::open(&config_path)?;
        for line in io::BufReader::new(file).lines() {
            let line = line?;
            let line_trim = line.trim();
            if line_trim.is_empty() || line_trim.starts_with('#') {
                continue;
            }

            if line_trim.starts_with("PS1=") {
                ps1_template = line_trim[4..].trim_matches('"').to_string();
            } else {
                init_commands.push(line_trim.to_string());
            }
        }

        if ps1_template.is_empty() {
            ps1_template = "{user}@{hostname} ~# {pwd}: ".to_string();
        }

        Ok(Config { ps1_template, init_commands })
    }

    pub fn get_ps1(&self) -> String {
        let user = env::var("USER").unwrap_or_else(|_| "user".into());
        let hostname = hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| "host".into());
        let cwd = env::current_dir()
            .ok()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "~".into());

        self.ps1_template
            .replace("{user}", &user)
            .replace("{hostname}", &hostname)
            .replace("{pwd}", &cwd)
    }
}
