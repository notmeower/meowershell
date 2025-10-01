use std::io;
use libc;

pub fn shutdown(args: &[&str]) -> io::Result<()> {
    if !args.contains(&"--force") {
        eprintln!("shutdown: refusing to execute without --force");
        return Ok(());
    }

    let action = if args.contains(&"-r") {
        libc::RB_AUTOBOOT
    } else {
        libc::RB_POWER_OFF
    };

    unsafe {
        if libc::reboot(action) != 0 {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(())
}