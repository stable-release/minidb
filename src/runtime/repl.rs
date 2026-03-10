use std::io::{Write, stdin, stdout};

use anyhow::Result;

pub fn REPL () -> Result<()> {
    let stdin = stdin();
    let mut stdout = stdout();    

    loop {
        stdout.flush()?;
        
        let mut line = String::new();
        if stdin.read_line(&mut line)? == 0 {
            break;
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with('.') {
            if let Err(e) = handle_dot_commands(&mut db, line) {
                eprintln!("Error: {}", e);
            }
            continue;
        }
    }

    Ok(())
}