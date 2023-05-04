use std::fs::OpenOptions;
use chrono::Utc;
use std::io::Write;

/// Saves error messages to a file for traceability purposes.
///
/// # Arguments
///
/// * `err_message` - The error message to be saved to the file
///
/// # Returns
///
/// ok(()) if the error message was successfully saved to the file.

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

