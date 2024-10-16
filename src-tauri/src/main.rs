#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager, State, Window, Emitter};
use std::sync::{Arc, Mutex};
use std::process::Command;
use tauri_plugin_dialog::FileDialogBuilder;

// Shared state to track progress
struct ProgressState {
  transcode_progress: Arc<Mutex<u32>>,
  upload_progress: Arc<Mutex<u32>>,
}

// Command to select a folder using Tauri's dialog system
#[tauri::command]
fn select_folder() -> String {
  let mut folder_path = String::new();

  FileDialogBuilder::new(tauri_plugin_dialog::Dialog::default())
  .pick_folder(|folder| {
      if let Some(path) = folder {
          folder_path = path.to_string().unwrap_or("").to_string();
      }
  });

  folder_path
}

// Command to start the transcoding process, simulate progress, and then simulate the upload
#[tauri::command]
async fn start_transcoding(
  raw_folder: String,
  destination_folder: String,
  gpu: String,
  state: State<'_, ProgressState>,
  app_handle: AppHandle
) -> Result<(), String> {
  // Reset progress
  *state.transcode_progress.lock().unwrap() = 0;
  *state.upload_progress.lock().unwrap() = 0;

  // Launch Deno script to handle transcoding
  let status = Command::new("deno")
      .args(&[
          "run", 
          "--allow-read", 
          "--allow-write", 
          "--allow-run", 
          "transcoder.ts", 
          &raw_folder, 
          &destination_folder, 
          &gpu
      ])
      .status()
      .expect("Failed to spawn Deno subprocess");

  if status.success() {
    // Simulate transcoding progress
    let app_handle_clone = app_handle.clone();
    std::thread::spawn(move || {
      for i in 0..=100 {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let progress = i;
        app_handle_clone.emit("transcode-progress", Some(progress)).unwrap();
        if progress == 100 { break; }
      }
    });

    // Simulate upload progress after transcoding
    let app_handle_clone = app_handle.clone();
    std::thread::spawn(move || {
      for i in 0..=100 {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let upload_progress = i;
        app_handle_clone.emit("upload-progress", Some(upload_progress)).unwrap();
        if upload_progress == 100 { break; }
      }
    });

    Ok(())
  } else {
    Err("Transcoding failed".into())
  }
}

fn main() {
  tauri::Builder::default()
    .manage(ProgressState {
      transcode_progress: Arc::new(Mutex::new(0)),
      upload_progress: Arc::new(Mutex::new(0)),
    })
    .invoke_handler(tauri::generate_handler![select_folder, start_transcoding])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
