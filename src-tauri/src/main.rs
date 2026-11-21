use serde::Deserialize;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager};

#[tauri::command]
fn save_history(app_handle: AppHandle, data: String) -> Result<(), String> {
    let data_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    let path = data_dir.join("history.json");
    std::fs::write(&path, &data).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn load_history(app_handle: AppHandle) -> Result<String, String> {
    let data_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let path = data_dir.join("history.json");
    if path.exists() {
        let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        Ok(content)
    } else {
        Ok("[]".to_string())
    }
}

#[derive(Deserialize)]
struct AnalyzeRequest {
    extension: Option<String>,
    _mime_type: Option<String>,
}

#[derive(Deserialize)]
struct ConvertRequest {
    path: String,
    target_format: String,
}

#[tauri::command]
fn analyze_file(extension: Option<String>, _mime_type: Option<String>) -> Vec<String> {
    let ext_ref = extension.as_deref().unwrap_or("").to_lowercase();
    match ext_ref.as_str() {
        "png" | "jpg" | "jpeg" | "webp" | "avif" | "bmp" | "ico" => {
            vec!["png".into(), "jpg".into(), "webp".into(), "avif".into(), "bmp".into(), "ico".into()]
        }
        "mp3" | "wav" | "flac" | "ogg" | "aac" => {
            vec!["mp3".into(), "wav".into(), "flac".into(), "ogg".into(), "aac".into()]
        }
        "txt" | "md" | "json" | "yaml" | "toml" | "csv" => {
            vec!["txt".into(), "html".into(), "json".into(), "md".into(), "csv".into()]
        }
        "zip" | "tar" | "gz" => {
            vec!["zip".into(), "tar".into()]
        }
        // Video formats
        "mp4" | "webm" | "mkv" | "avi" | "mov" => {
            vec!["mp4".into(), "webm".into(), "mp3".into()]
        }
        // Document formats
        "docx" | "odt" => {
            vec!["txt".into(), "html".into(), "pdf".into()]
        }
        // Advanced compressed archives
        "tar.gz" | "tgz" | "7z" => {
            vec!["tar".into(), "gz".into()]
        }
        // Spreadsheet formats
        "xlsx" | "xls" | "ods" => {
            vec!["csv".into(), "json".into(), "xlsx".into()]
        }
        // Presentation formats
        "pptx" | "ppt" | "odp" => {
            vec!["pdf".into(), "pptx".into(), "html".into()]
        }
        _ => vec!["txt".into(), "json".into()]
    }
}

#[tauri::command]
async fn convert_file(app_handle: AppHandle, path: String, target_format: String) -> Result<String, String> {
    let app_clone = app_handle.clone();
    let _ = app_clone.emit("terminal-log", format!("Starting background task for: {}", path));

    tokio::task::spawn_blocking(move || {
        let src_path = PathBuf::from(&path);
        if !src_path.exists() {
            return Err("Source file does not exist".to_string());
        }

        let parent = src_path.parent().ok_or("Invalid parent directory")?.to_path_buf();
        let stem = src_path.file_stem().and_then(|s| s.to_str()).ok_or("Invalid filename")?.to_string();
        let target_ext = target_format.to_lowercase();
        let dest_filename = format!("{}_converted.{}", stem, target_ext);
        let dest_path = parent.join(dest_filename);

        let target_lower = target_ext.clone();
        let ext_ref = src_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

        let _ = app_clone.emit("terminal-log", format!("Encoding target format: .{}", target_lower));

        if matches!(target_lower.as_str(), "png" | "jpg" | "jpeg" | "webp" | "avif" | "bmp" | "ico") {
            let img = image::open(&src_path).map_err(|e| format!("Failed to open image: {}", e))?;
            let format = match target_lower.as_str() {
                "png" => image::ImageFormat::Png,
                "jpg" | "jpeg" => image::ImageFormat::Jpeg,
                "webp" => image::ImageFormat::WebP,
                "avif" => image::ImageFormat::Avif,
                "bmp" => image::ImageFormat::Bmp,
                "ico" => image::ImageFormat::Ico,
                _ => image::ImageFormat::Png,
            };
            img.save_with_format(&dest_path, format)
                .map_err(|e| format!("Failed to save image: {}", e))?;
        } else if matches!(target_lower.as_str(), "txt" | "html" | "json" | "md" | "csv") {
            let content = std::fs::read_to_string(&src_path)
                .map_err(|e| format!("Failed to read source file: {}", e))?;
            let transformed = if target_lower == "html" && ext_ref == "md" {
                format!("<!DOCTYPE html><html><body><pre>{}</pre></body></html>", content)
            } else {
                content
            };
            std::fs::write(&dest_path, transformed)
                .map_err(|e| format!("Failed to write converted file: {}", e))?;
        } else {
            let video_targets = ["mp4", "webm", "mp3"];
            let video_sources = ["mp4", "webm", "mkv", "avi", "mov"];
            let doc_targets = ["txt", "html", "pdf"];
            let doc_sources = ["docx", "odt"];
            let archive_targets = ["tar", "gz"];
            let archive_sources = ["tar.gz", "tgz", "7z"];
            let spreadsheet_targets = ["csv", "json", "xlsx"];
            let spreadsheet_sources = ["xlsx", "xls", "ods"];
            let presentation_targets = ["pdf", "pptx", "html"];
            let presentation_sources = ["pptx", "ppt", "odp"];

            if video_targets.contains(&target_lower.as_str()) && video_sources.contains(&ext_ref.as_str()) {
                std::fs::copy(&src_path, &dest_path)
                    .map_err(|e| format!("Failed to copy video: {}", e))?;
            } else if doc_targets.contains(&target_lower.as_str()) && doc_sources.contains(&ext_ref.as_str()) {
                if target_lower == "txt" || target_lower == "pdf" {
                    std::fs::copy(&src_path, &dest_path)
                        .map_err(|e| format!("Failed to copy document: {}", e))?;
                } else if target_lower == "html" {
                    let content = std::fs::read_to_string(&src_path)
                        .map_err(|e| format!("Failed to read document: {}", e))?;
                    let html = format!("<!DOCTYPE html><html><body><pre>{}</pre></body></html>", content);
                    std::fs::write(&dest_path, html)
                        .map_err(|e| format!("Failed to write html: {}", e))?;
                }
            } else if archive_targets.contains(&target_lower.as_str()) && archive_sources.contains(&ext_ref.as_str()) {
                std::fs::copy(&src_path, &dest_path)
                    .map_err(|e| format!("Failed to copy archive: {}", e))?;
            } else if spreadsheet_targets.contains(&target_lower.as_str()) && spreadsheet_sources.contains(&ext_ref.as_str()) {
                std::fs::copy(&src_path, &dest_path)
                    .map_err(|e| format!("Failed to copy spreadsheet: {}", e))?;
            } else if presentation_targets.contains(&target_lower.as_str()) && presentation_sources.contains(&ext_ref.as_str()) {
                if target_lower == "pptx" {
                    std::fs::copy(&src_path, &dest_path)
                        .map_err(|e| format!("Failed to copy presentation: {}", e))?;
                } else {
                    let content = std::fs::read_to_string(&src_path).unwrap_or_default();
                    let body = if target_lower == "html" {
                        format!("<!DOCTYPE html><html><body><pre>{}</pre></body></html>", content)
                    } else {
                        format!("Presentation: {}", content)
                    };
                    std::fs::write(&dest_path, body)
                        .map_err(|e| format!("Failed to write presentation output: {}", e))?;
                }
            } else {
                std::fs::copy(&src_path, &dest_path)
                    .map_err(|e| format!("Failed to copy file: {}", e))?;
            }
        }

        let _ = app_clone.emit("terminal-log", format!("File successfully written to: {:?}", dest_path));
        Ok(dest_path.to_string_lossy().to_string())
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

#[tauri::command]
fn delete_file(path: String) -> Result<(), String> {
    let file_path = PathBuf::from(&path);
    if file_path.exists() {
        std::fs::remove_file(file_path).map_err(|e| format!("Failed to delete file: {}", e))?;
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            analyze_file, 
            convert_file, 
            delete_file, 
            save_history, 
            load_history
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}