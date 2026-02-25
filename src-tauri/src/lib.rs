use serde::{Deserialize, Serialize};
use std::fs::{self, remove_dir_all};
use std::path::Path;
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackupConfig {
    pub note: String,
    pub save_time: String,
    pub source_path: String,
    pub size: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackupItem {
    pub id: String,
    pub note: String,
    pub save_time: String,
    pub source_path: String,
    pub size: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_app_data_dir(app: tauri::AppHandle) -> Result<String, String> {
    app.path()
        .app_data_dir()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn backup_save(
    source_path: String,
    note: String,
    _game_name: String,
) -> Result<String, String> {
    let timestamp = chrono_timestamp();
    
    let exe_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .ok_or("Failed to get executable directory")?
        .to_path_buf();
    
    let backup_dir = exe_dir.join("backupFiles").join(&timestamp);
    let file_dir = backup_dir.join("file");
    
    fs::create_dir_all(&file_dir).map_err(|e| e.to_string())?;
    
    let source = Path::new(&source_path);
    let size = if source.exists() {
        copy_dir_all(source, &file_dir).map_err(|e| e.to_string())?;
        calculate_dir_size(&file_dir)
    } else {
        String::new()
    };
    
    let config = BackupConfig {
        note,
        save_time: timestamp.clone(),
        source_path,
        size,
    };
    
    let config_path = backup_dir.join("config.json");
    let config_json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(&config_path, config_json).map_err(|e| e.to_string())?;
    
    Ok(timestamp)
}

#[tauri::command]
fn get_backup_list() -> Result<Vec<BackupItem>, String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .ok_or("Failed to get executable directory")?
        .to_path_buf();
    
    let backup_base = exe_dir.join("backupFiles");
    
    if !backup_base.exists() {
        return Ok(vec![]);
    }
    
    let mut backups = Vec::new();
    
    for entry in fs::read_dir(&backup_base).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        
        if !path.is_dir() {
            continue;
        }
        
        let id = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        
        let config_path = path.join("config.json");
        let config_content = if config_path.exists() {
            fs::read_to_string(&config_path).ok()
        } else {
            None
        };
        
        let (note, save_time, source_path, size) = if let Some(content) = config_content {
            if let Ok(config) = serde_json::from_str::<BackupConfig>(&content) {
                let size = if config.size.is_empty() {
                    calculate_dir_size(&path.join("file"))
                } else {
                    config.size
                };
                (config.note, config.save_time, config.source_path, size)
            } else {
                (String::new(), id.clone(), String::new(), calculate_dir_size(&path.join("file")))
            }
        } else {
            (String::new(), id.clone(), String::new(), calculate_dir_size(&path.join("file")))
        };
        
        backups.push(BackupItem {
            id,
            note,
            save_time,
            source_path,
            size,
        });
    }
    
    backups.sort_by(|a, b| b.save_time.cmp(&a.save_time));
    
    Ok(backups)
}

#[tauri::command]
fn update_backup_note(backup_id: String, note: String) -> Result<(), String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .ok_or("Failed to get executable directory")?
        .to_path_buf();
    
    let backup_dir = exe_dir.join("backupFiles").join(&backup_id);
    let config_path = backup_dir.join("config.json");
    
    if !config_path.exists() {
        return Err("备份文件不存在".to_string());
    }
    
    let config_content = fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
    let mut config: BackupConfig = serde_json::from_str(&config_content).map_err(|e| e.to_string())?;
    
    config.note = note;
    
    let config_json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write(&config_path, config_json).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
fn delete_backup(backup_id: String) -> Result<(), String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .ok_or("Failed to get executable directory")?
        .to_path_buf();
    
    let backup_dir = exe_dir.join("backupFiles").join(&backup_id);
    
    if !backup_dir.exists() {
        return Err("备份文件不存在".to_string());
    }
    
    fs::remove_dir_all(&backup_dir).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
fn delete_backups(backup_ids: Vec<String>) -> Result<(), String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .ok_or("Failed to get executable directory")?
        .to_path_buf();
    
    let backup_base = exe_dir.join("backupFiles");
    
    for backup_id in backup_ids {
        let backup_dir = backup_base.join(&backup_id);
        if backup_dir.exists() {
            fs::remove_dir_all(&backup_dir).map_err(|e| e.to_string())?;
        }
    }
    
    Ok(())
}

fn calculate_dir_size(path: &Path) -> String {
    if !path.exists() {
        return "0 B".to_string();
    }
    
    let total_size = calculate_size_recursive(path);
    
    if total_size < 1024 {
        format!("{} B", total_size)
    } else if total_size < 1024 * 1024 {
        format!("{:.1} KB", total_size as f64 / 1024.0)
    } else if total_size < 1024 * 1024 * 1024 {
        format!("{:.1} MB", total_size as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", total_size as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn calculate_size_recursive(path: &Path) -> u64 {
    if !path.exists() {
        return 0;
    }
    
    if path.is_file() {
        return path.metadata().map(|m| m.len()).unwrap_or(0);
    }
    
    let mut total = 0u64;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            total += calculate_size_recursive(&entry.path());
        }
    }
    total
}

#[tauri::command]
fn restore_save(backup_id: String) -> Result<String, String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .ok_or("Failed to get executable directory")?
        .to_path_buf();
    
    let backup_dir = exe_dir.join("backupFiles").join(&backup_id);
    let file_dir = backup_dir.join("file");
    let config_path = backup_dir.join("config.json");
    
    if !backup_dir.exists() {
        return Err("备份目录不存在".to_string());
    }
    
    if !file_dir.exists() {
        return Err("备份文件目录不存在".to_string());
    }
    
    let source_path = if config_path.exists() {
        let config_content = fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
        let config: BackupConfig = serde_json::from_str(&config_content).map_err(|e| e.to_string())?;
        config.source_path
    } else {
        return Err("备份配置不存在".to_string());
    };
    
    let dest_path = Path::new(&source_path);
    
    if dest_path.exists() {
        if dest_path.is_dir() {
            remove_dir_all(dest_path).map_err(|e| e.to_string())?;
        } else {
            fs::remove_file(dest_path).map_err(|e| e.to_string())?;
        }
    }
    
    copy_dir_all(&file_dir, dest_path).map_err(|e| e.to_string())?;
    
    Ok(source_path)
}

fn chrono_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());
        
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dest_path)?;
        } else {
            fs::copy(entry.path(), dest_path)?;
        }
    }
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![greet, get_app_data_dir, backup_save, get_backup_list, restore_save, update_backup_note, delete_backup, delete_backups])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
