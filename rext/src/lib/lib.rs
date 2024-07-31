use std::fs;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

pub fn open_pdf_with_browser(folder_path: &str, browser_command: &str) {
    let mut pdf_files: Vec<_> = WalkDir::new(folder_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("pdf"))
        .collect();

    pdf_files.sort_by_key(|e| fs::metadata(e.path()).and_then(|meta| meta.modified()).ok());
    pdf_files.reverse();

    for entry in pdf_files {
        let path = entry.path().canonicalize().unwrap();
        Command::new(browser_command)
            .arg("--new-tab")
            .arg(path)
            .spawn()
            .expect("Failed to open PDF in browser");
    }
}

pub fn open_pdf_with_firefox(folder_path: &str) {
    let browser_command = get_browser_command("firefox");
    open_pdf_with_browser(folder_path, &browser_command);
}

pub fn open_pdf_with_chrome(folder_path: &str) {
    let browser_command = get_browser_command("chrome");
    open_pdf_with_browser(folder_path, &browser_command);
}

fn get_browser_command(browser: &str) -> String {
    if cfg!(target_os = "linux") {
        match browser {
            "chrome" => "google-chrome".to_string(),
            "firefox" => "firefox".to_string(),
            _ => panic!("Unsupported browser"),
        }
    } else if cfg!(target_os = "windows") {
        format!("start {}", browser)
    } else {
        panic!("Unsupported operating system");
    }
}
