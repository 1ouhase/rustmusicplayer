use std::{io::{Result, Write}, process::ExitStatus, fs::File};

fn main() {
    console_clear();

    const INTERNET_CONNECTION: bool = true;

    if !INTERNET_CONNECTION {
        let no_internet_warn = "WARN: device has no internetconnection";
        println!("{}", no_internet_warn);
        log_action(no_internet_warn.to_string()).expect("ERROR: could not log");
    }
    
    loop {
        
    }
}

fn log_action(message: String) -> Result<()> {
    let logfile = "log.txt";
    let mut file: File = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
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
            log_action("INFO: clearing windows console".to_string()).expect("ERROR: could not log");
            std::process::Command::new("cmd").args(&["/C", "cls"]).status()
        } else {
            log_action("INFO: clearing unix console".to_string()).expect("ERROR: could not log");
            std::process::Command::new("clear").status()
        }
    );
}