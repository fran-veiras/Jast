use std::fs::OpenOptions;
use chrono::Utc;
use std::io::Write;

pub fn error_log(err_message : &str) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .write(true)
        .create(true)
        .open("jast_err_logs.txt");


    let utc_time = Utc::now();
    
    let full_log : String = format!("Error: {}; time: {:?}", err_message, utc_time);

    writeln!(file?, "{};", full_log)?;

    Ok(())
}

