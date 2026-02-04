use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use chrono::Local;

/// Generate the log filename for today
pub fn get_today_filename(project_name: &str) -> String {
    let date_str = Local::now().format("%Y%m%d").to_string();
    format!("{}{}.txt", project_name, date_str)
}

/// Save content to a log file (append mode)
/// Adds two blank lines before the content (like Java version)
pub fn save_log(log_path: &Path, project_name: &str, content: &str) -> io::Result<()> {
    // Don't save empty content
    if content.trim().is_empty() {
        return Ok(());
    }

    let filename = get_today_filename(project_name);
    let path = log_path.join(&filename);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    // Add blank lines before content (like Java version)
    writeln!(file)?;
    writeln!(file)?;
    write!(file, "{}", content)?;

    Ok(())
}

/// Get current time formatted as HH:MM
pub fn get_current_time() -> String {
    Local::now().format("%H:%M").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_time_format() {
        let time = get_current_time();
        // Should be in HH:MM format (5 characters)
        assert_eq!(time.len(), 5);
        assert_eq!(&time[2..3], ":");
    }

    #[test]
    fn test_get_today_filename() {
        let filename = get_today_filename("WorkLog");
        // Should start with project name
        assert!(filename.starts_with("WorkLog"));
        // Should end with .txt
        assert!(filename.ends_with(".txt"));
        // Should be ProjectName + 8 digits + .txt
        assert_eq!(filename.len(), "WorkLog".len() + 8 + 4);
    }
}
