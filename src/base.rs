use std::io::Write;
use std::fs::OpenOptions;
use chrono::Local;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn write_logfile(message: &str) {
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let date = now.format("%Y-%m-%d").to_string();
    
    let log_entry = format!("[{}] {}\n", timestamp, message);
    
    let log_dir = PathBuf::from("/opt/builder/log");
    let log_file = log_dir.join(format!("{}.log", date));
    
    match OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
    {
        Ok(mut file) => {
            match file.write_all(log_entry.as_bytes()) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("ERROR: could not write to log file: {:?}: {}", log_file, e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("ERROR: could not open log file: {:?}: {}", log_file, e);
            std::process::exit(1);
        }
    }
}

pub fn log(message: &str) {
    println!("{}", message);
    write_logfile(message);
}

pub fn halt(message: &str) -> ! {
    let error_message = &format!("ERROR: {}", message);
    eprintln!("{}", error_message);
    write_logfile(error_message);
    std::process::exit(1)
}

pub fn perform( description: &str, check: Option<Command>, mut operation: Command ){
    if let Some(mut check_cmd) = check {
        match check_cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    log(&format!("{} was already done, skipping.", description));
                    return
                } 
            },
            Err(e) => {
                halt(&format!("Check command '{}' failed: {}", check_cmd.cmdline(), e));
            }
        }
    }

    match operation.output() {
        Ok(output) => {
            if output.status.success() {
                log(&format!("{} succeeded.", description));
            } else {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                halt(&format!("Command '{}' failed:\nSTDOUT: {}\nSTDERR: {}", operation.cmdline(), stdout, stderr));
            }
        },
        Err(e) => {
            halt(&format!("Command '{}' failed: {}", operation.cmdline(), e));
        }
    }

}


trait CommandExt {
    fn cmdline(&self) -> String;
}

impl CommandExt for Command {
    fn cmdline(&self) -> String {
        format!("{} {}",
            self.get_program().to_str().unwrap_or(""),
            self.get_args().map(|s| s.to_str().unwrap_or("")).collect::<Vec<_>>().join(" ")
        )
    }
}


pub fn wait(mut command: Command, sleep: u64) {
    loop {
        match command.output() {
            Ok(output) => {
                if output.status.success() {
                    return;
                } else {
                    thread::sleep(Duration::from_secs(sleep));
                }
            },
            Err(e) => {
                halt(&format!("Command '{}' failed: {}", command.cmdline(), e));
            }
        }
    }
}