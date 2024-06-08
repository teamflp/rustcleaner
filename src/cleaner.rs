use walkdir::{WalkDir, Error as WalkDirError};
use regex::Regex;
use std::fs;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use prettytable::{Table, Row, Cell, row};
use indicatif::{ProgressBar, ProgressStyle};
use std::io::ErrorKind;
use rayon::prelude::*;
use chrono::{DateTime, Utc, Duration};
use std::time::SystemTime;
use std::time::Duration as StdDuration;
use std::thread;
use colored::*;

fn is_permission_denied(err: &WalkDirError) -> bool {
    if let Some(inner_err) = err.io_error() {
        return inner_err.kind() == ErrorKind::PermissionDenied;
    }
    false
}

fn has_permission(path: &Path) -> bool {
    fs::read_dir(path).is_ok()
}

fn get_trash_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        None
    }
    #[cfg(target_os = "macos")]
    {
        Some(PathBuf::from("~/Library/.Trash"))
    }
    #[cfg(target_os = "linux")]
    {
        Some(PathBuf::from("~/.local/share/Trash/files"))
    }
}

pub fn scan_files<P: AsRef<Path>>(dirs_to_scan: &[P], _exclude_dirs: &HashSet<String>, exclude_types: &HashSet<String>) -> Vec<PathBuf> {
    let re = Regex::new(r".*\.(tmp|log|old|bak)$").unwrap();
    let mut table = Table::new();
    let mut files_to_clean = Vec::new();

    table.add_row(row!["File Path".bold().blue(), "Status".bold().blue()]);

    let total_entries: Vec<_> = dirs_to_scan.iter()
        .flat_map(|dir| {
            if !has_permission(dir.as_ref()) && !dir.as_ref().to_str().unwrap_or_default().contains("Downloads") {
                eprintln!("{}", format!("Skipping directory due to lack of permission: {}", dir.as_ref().display()).yellow());
                vec![]
            } else {
                WalkDir::new(dir).into_iter().collect::<Vec<_>>()
            }
        })
        .collect();

    let pb = ProgressBar::new(total_entries.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} {msg}\n{wide_bar:.green} {pos}/{len} ({percent}%)")
        .expect("Invalid template")
        .progress_chars("█  "));
    pb.set_message("Scanning in progress...");

    for entry in total_entries {
        pb.inc(1);
        thread::sleep(StdDuration::from_millis(1)); // Sleep to prevent progress bar from flickering
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    let path = entry.path();
                    let ext = path.extension().unwrap_or_default().to_str().unwrap_or_default();
                    if re.is_match(path.to_str().unwrap()) && !exclude_types.contains(ext) {
                        files_to_clean.push(path.to_path_buf());
                        table.add_row(Row::new(vec![
                            Cell::new(&path.to_str().unwrap().blue().to_string()),
                            Cell::new(&"To clean".yellow().to_string()),
                        ]));
                    }
                }
            }
            Err(e) => {
                if is_permission_denied(&e) {
                    //eprintln!("{}", format!("Permission denied: {}", e).red());
                } else {
                    eprintln!("{}", format!("Failed to access entry: {}", e).red());
                }
            }
        }
    }
    pb.finish_with_message("Scan complete");

    table.printstd();
    files_to_clean
}

pub fn clean_files(files_to_clean: Vec<PathBuf>, secure: bool) {
    let mut table = Table::new();

    table.add_row(row!["File Path".bold().blue(), "Status".bold().blue()]);
    let pb = ProgressBar::new(files_to_clean.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} {msg}\n{wide_bar:.green} {pos}/{len} ({percent}%)")
        .expect("Invalid template")
        .progress_chars("█  "));
    pb.set_message("Cleaning in progress...");

    let results: Vec<(String, String)> = files_to_clean.par_iter().map(|path| {
        pb.inc(1);
        thread::sleep(StdDuration::from_millis(10));
        let status = if secure {
            match secure_delete(path) {
                Ok(_) => "Deleted".to_string(),
                Err(e) => {
                    if e.kind() == ErrorKind::PermissionDenied {
                        eprintln!("{}", format!("Permission denied: {}", path.display()).red());
                        "Permission denied".to_string()
                    } else {
                        eprintln!("{}", format!("Failed to delete {}: {}", path.display(), e).red());
                        "Failed to delete".to_string()
                    }
                }
            }
        } else {
            match fs::remove_file(path) {
                Ok(_) => "Deleted".to_string(),
                Err(e) => {
                    if e.kind() == ErrorKind::PermissionDenied {
                        eprintln!("{}", format!("Permission denied: {}", path.display()).red());
                        "Permission denied".to_string()
                    } else {
                        eprintln!("{}", format!("Failed to delete {}: {}", path.display(), e).red());
                        "Failed to delete".to_string()
                    }
                }
            }
        };
        (path.to_str().unwrap().to_string(), status)
    }).collect();

    pb.finish_with_message("Clean complete");

    for (path, status) in results {
        table.add_row(Row::new(vec![
            Cell::new(&path.blue().to_string()),
            Cell::new(&status.green().to_string()),
        ]));
    }

    table.printstd();
}

pub fn clear_directory<P: AsRef<Path>>(dir: P) {
    let mut table = Table::new();
    table.add_row(row!["File Path".bold().blue(), "Status".bold().blue()]);

    if has_permission(dir.as_ref()) {
        let entries: Vec<_> = WalkDir::new(&dir).into_iter().collect();
        let pb = ProgressBar::new(entries.len() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} {msg}\n{wide_bar:.green} {pos}/{len} ({percent}%)")
            .expect("Invalid template")
            .progress_chars("█  "));
        pb.set_message("Clearing directory...");

        let results: Vec<(String, String)> = entries.par_iter().map(|entry| {
            pb.inc(1);
            thread::sleep(StdDuration::from_millis(10));
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_file() {
                        let path = entry.path();
                        let status = match fs::remove_file(path) {
                            Ok(_) => "Deleted".to_string(),
                            Err(e) => {
                                if e.kind() == ErrorKind::PermissionDenied {
                                    eprintln!("{}", format!("Permission denied: {}", path.display()).red());
                                    "Permission denied".to_string()
                                } else {
                                    eprintln!("{}", format!("Failed to delete {}: {}", path.display(), e).red());
                                    "Failed to delete".to_string()
                                }
                            }
                        };
                        (path.to_str().unwrap().to_string(), status)
                    } else {
                        (entry.path().to_str().unwrap().to_string(), "Skipped".to_string())
                    }
                }
                Err(e) => {
                    if is_permission_denied(&e) {
                        eprintln!("{}", format!("Permission denied: {}", e).red());
                    } else {
                        eprintln!("{}", format!("Failed to access entry: {}", e).red());
                    }
                    (e.path().map_or("".to_string(), |p| p.to_str().unwrap().to_string()), "Error".to_string())
                }
            }
        }).collect();

        pb.finish_with_message("Clear complete");

        for (path, status) in results {
            table.add_row(Row::new(vec![
                Cell::new(&path.blue().to_string()),
                Cell::new(&status.green().to_string()),
            ]));
        }
    } else {
        eprintln!("{}", format!("Skipping directory due to lack of permission: {}", dir.as_ref().display()).yellow());
    }

    table.printstd();
}

pub fn clear_trash() {
    if let Some(trash_dir) = get_trash_dir() {
        clear_directory(trash_dir);
    } else {
        println!("{}", "Cannot access trash directory on this platform.".yellow());
    }
}

pub fn report_clean(files_to_clean: Vec<PathBuf>, start_time: SystemTime) {
    let end_time = SystemTime::now();
    let duration = end_time.duration_since(start_time).expect("Time went backwards");
    let total_size: u64 = files_to_clean.par_iter().map(|path| {
        fs::metadata(path).map(|meta| meta.len()).unwrap_or(0)
    }).sum();

    println!("{}", "Report:".bold());
    println!("{}", format!("Total files cleaned: {}", files_to_clean.len()).green());
    println!("{}", format!("Total size cleaned: {} bytes", total_size).green());
    println!("{}", format!("Time taken: {:.2?}", duration).green());
}

pub fn secure_delete(path: &Path) -> std::io::Result<()> {
    use rand::Rng;
    let metadata = fs::metadata(path)?;
    let len = metadata.len();
    let mut rng = rand::thread_rng();
    let mut data = vec![0u8; len as usize];
    rng.fill(&mut data[..]);
    fs::write(path, &data)?;
    fs::remove_file(path)
}

pub fn scan_files_for_age<P: AsRef<Path>>(dirs_to_scan: &[P], exclude_dirs: &HashSet<String>, age_in_days: i64) -> Vec<PathBuf> {
    let now = Utc::now();
    let mut files_to_clean = Vec::new();

    for dir in dirs_to_scan {
        let dir_str = dir.as_ref().to_str().unwrap();
        if !exclude_dirs.contains(dir_str) && has_permission(dir.as_ref()) {
            for entry in WalkDir::new(dir) {
                match entry {
                    Ok(entry) => {
                        if entry.file_type().is_file() {
                            let metadata = fs::metadata(entry.path()).unwrap();
                            if let Ok(modified) = metadata.modified() {
                                let modified: DateTime<Utc> = modified.into();
                                if now.signed_duration_since(modified) > Duration::days(age_in_days) {
                                    files_to_clean.push(entry.path().to_path_buf());
                                }
                            }
                        }
                    }
                    Err(e) => {
                        if is_permission_denied(&e) {
                            eprintln!("{}", format!("Permission denied: {}", e).red());
                        } else {
                            eprintln!("{}", format!("Failed to access entry: {}", e).red());
                        }
                    }
                }
            }
        } else {
            eprintln!("{}", format!("Skipping directory due to lack of permission or exclusion: {}", dir_str).yellow());
        }
    }

    files_to_clean
}

pub fn find_duplicate_files(dirs_to_scan: &[PathBuf], exclude_dirs: &HashSet<String>, exclude_types: &HashSet<String>) -> Vec<PathBuf> {
    let mut files_to_clean = Vec::new();
    let mut file_hashes: HashSet<u64> = HashSet::new();

    for dir in dirs_to_scan {
        if !exclude_dirs.contains(dir.to_str().unwrap()) && has_permission(dir.as_ref()) {
            for entry in WalkDir::new(dir) {
                match entry {
                    Ok(entry) => {
                        if entry.file_type().is_file() {
                            let path = entry.path();
                            let ext = path.extension().unwrap_or_default().to_str().unwrap_or_default();
                            if !exclude_types.contains(ext) {
                                let metadata = fs::metadata(path).unwrap();
                                let hash = metadata.len();
                                if file_hashes.contains(&hash) {
                                    files_to_clean.push(path.to_path_buf());
                                } else {
                                    file_hashes.insert(hash);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        if is_permission_denied(&e) {
                            eprintln!("{}", format!("Permission denied: {}", e).red());
                        } else {
                            eprintln!("{}", format!("Failed to access entry: {}", e).red());
                        }
                    }
                }
            }
        }
    }

    files_to_clean
}

pub fn print_duplicates_report(duplicate_files: &[PathBuf]) {
    let mut table = Table::new();
    table.add_row(row!["File Path".bold().blue(), "Status".bold().blue()]);

    for file in duplicate_files {
        table.add_row(Row::new(vec![
            Cell::new(&file.to_str().unwrap().blue().to_string()),
            Cell::new(&"Duplicate".yellow().to_string()),
        ]));
    }

    table.printstd();
}

pub fn clean_browser_cache() {
    let cache_dirs = vec![
        dirs::cache_dir().unwrap().join("mozilla"),
        dirs::cache_dir().unwrap().join("google-chrome"),
        dirs::cache_dir().unwrap().join("firefox"),
    ];

    for cache_dir in cache_dirs {
        clear_directory(cache_dir);
    }
}

pub fn restore_files() {
    // Logic to restore files from a backup location can be implemented here
    println!("Restoring files...");
}

pub fn secure_clean_files(files_to_clean: Vec<PathBuf>) {
    for file in files_to_clean {
        match secure_delete(&file) {
            Ok(_) => println!("Securely deleted: {}", file.display()),
            Err(e) => println!("Failed to securely delete {}: {}", file.display(), e),
        }
    }
}

// Tests Unitaires
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File};
    use tempfile::tempdir;

    #[test]
    fn test_scan_files() {
        let dir = tempdir().unwrap();
        let file1 = dir.path().join("test1.tmp");
        let file2 = dir.path().join("test2.log");
        let file3 = dir.path().join("test3.txt");

        File::create(&file1).unwrap();
        File::create(&file2).unwrap();
        File::create(&file3).unwrap();

        let exclude_dirs: HashSet<String> = HashSet::new();
        let exclude_types: HashSet<String> = HashSet::new();
        let dirs_to_scan = vec![dir.path().to_path_buf()];

        let files_to_clean = scan_files(&dirs_to_scan, &exclude_dirs, &exclude_types);

        assert!(files_to_clean.contains(&file1));
        assert!(files_to_clean.contains(&file2));
        assert!(!files_to_clean.contains(&file3));
    }

    #[test]
    fn test_clean_files() {
        let dir = tempdir().unwrap();
        let file1 = dir.path().join("test1.tmp");
        let file2 = dir.path().join("test2.log");
        let file3 = dir.path().join("test3.txt");

        File::create(&file1).unwrap();
        File::create(&file2).unwrap();
        File::create(&file3).unwrap();

        let files_to_clean = vec![file1.clone(), file2.clone()];

        clean_files(files_to_clean.clone(), false);

        assert!(!file1.exists());
        assert!(!file2.exists());
        assert!(file3.exists());

        for file in files_to_clean {
            assert!(!file.exists());
        }
    }

    #[test]
    fn test_clear_directory() {
        let dir = tempdir().unwrap();
        let file1 = dir.path().join("test1.tmp");
        let file2 = dir.path().join("test2.log");
        let file3 = dir.path().join("test3.txt");

        File::create(&file1).unwrap();
        File::create(&file2).unwrap();
        File::create(&file3).unwrap();

        clear_directory(dir.path());

        assert!(!file1.exists());
        assert!(!file2.exists());
        assert!(!file3.exists());
    }

    #[test]
    fn test_scan_files_for_age() {
        let dir = tempdir().unwrap();
        let file1 = dir.path().join("test1.tmp");
        let file2 = dir.path().join("test2.log");
        let file3 = dir.path().join("test3.txt");

        File::create(&file1).unwrap();
        File::create(&file2).unwrap();
        File::create(&file3).unwrap();

        let exclude_dirs: HashSet<String> = HashSet::new();
        let dirs_to_scan = vec![dir.path().to_path_buf()];

        // Simulate files modified 31 days ago
        let age_in_days = 31;
        let modified_time = SystemTime::now() - std::time::Duration::from_secs(age_in_days * 24 * 60 * 60);
        filetime::set_file_mtime(&file1, filetime::FileTime::from_system_time(modified_time)).unwrap();
        filetime::set_file_mtime(&file2, filetime::FileTime::from_system_time(modified_time)).unwrap();

        let files_to_clean = scan_files_for_age(&dirs_to_scan, &exclude_dirs, age_in_days as i64);

        assert!(files_to_clean.contains(&file1));
        assert!(files_to_clean.contains(&file2));
        assert!(!files_to_clean.contains(&file3));
    }

    #[test]
    fn test_find_duplicate_files() {
        let dir = tempdir().unwrap();
        let file1 = dir.path().join("test1.txt");
        let file2 = dir.path().join("test2.txt");
        let file3 = dir.path().join("test3.txt");

        File::create(&file1).unwrap();
        File::create(&file2).unwrap();
        File::create(&file3).unwrap();

        let exclude_dirs: HashSet<String> = HashSet::new();
        let exclude_types: HashSet<String> = HashSet::new();
        let dirs_to_scan = vec![dir.path().to_path_buf()];

        // Create duplicate content
        fs::write(&file1, "duplicate content").unwrap();
        fs::write(&file2, "duplicate content").unwrap();

        let duplicate_files = find_duplicate_files(&dirs_to_scan, &exclude_dirs, &exclude_types);

        assert!(duplicate_files.contains(&file1));
        assert!(duplicate_files.contains(&file2));
        assert!(!duplicate_files.contains(&file3));
    }
}
