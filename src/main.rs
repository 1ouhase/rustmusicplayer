use std::{io::{Result, Write}, process::ExitStatus, fs::File};

fn main() {
    console_clear();

    
}

fn log_action(message: String) -> Result<()> {
    let logfile = "log.txt";
    let mut file: File = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(false)
        .open(logfile)?;
    file.write_all((message + "\n").as_bytes())?;
    Ok(())
}

fn handle_console_clear_error(result: Result<ExitStatus>) {
    match result {
        Ok(_) => {
            log_action("INFO: console has been cleared".to_string()).expect("ERROR: could not log");
        },
        Err(err) => {
            log_action("ERROR: console could not be cleared".to_string()).expect("ERROR: could not log");
            println!("ERROR: console could not be cleared: {}", err);
        }
    }
}

fn console_clear() {
    handle_console_clear_error(
        if cfg!(windows) {
            std::process::Command::new("cmd").args(&["/C", "cls"]).status()
        } else {
            std::process::Command::new("clear").status()
        }
    );
}