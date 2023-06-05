// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;
use std::path::Path;
use fs_extra::dir::{CopyOptions, copy};

// use tauri::http::ResponseBuilder;
// use tauri::http::Request;
// use tauri::http::Response;
// use std::fs::canonicalize;
// use std::error::Error;
// use tauri::AppHandle;
// use std::fs::read;


#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
  if let Some(splashscreen) = window.get_window("splashscreen") {
    splashscreen.close().unwrap();
  }
  window.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
async fn copy_folder(src: String, dest: String) -> Result<(), String> {
  let source_folder = Path::new(&src);
  let dest_folder = Path::new(&dest);

  if !source_folder.is_dir() {
    return Err("源目录不存在或不是目录".to_string());
  }

  if !dest_folder.is_dir() {
    return Err("目标目录不存在或不是目录".to_string());
  }

  let options = CopyOptions::new();
  match copy(src, dest, &options) {
    Ok(_) => Ok(()),
    Err(err) => Err(format!("复制文件过程中发生错误：{}", err)),
  }
}

// fn handle_register_uri_scheme_protocol(app: &AppHandle, request: &Request) -> Result<Response, Box<dyn Error>> {

// }

// fn handle_register_uri_scheme_protocol(
//   _app: &AppHandle,
//   request: &Request,
// ) -> Result<Response, Box<dyn Error>> {
//   println!("path1: {}", request.uri());
//   let path = request.uri().replace("https://test.localhost/", "");
//   println!("path2: {}", path);
//   let content = read(canonicalize(&path)?)?;
//   let (data, meta) = if path.ends_with(".html") {
//     (content, "text/html")
//   } else if path.ends_with(".js") {
//     (content, "text/javascript")
//   } else if path.ends_with(".png") {
//     (content, "image/png")
//   } else {
//     unimplemented!();
//   };
//   ResponseBuilder::new().header("Access-Control-Allow-Origin", "https://test.localhost").mimetype(meta).body(data)
// }



fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .invoke_handler(tauri::generate_handler![close_splashscreen,copy_folder])
        // .register_uri_scheme_protocol("test", handle_register_uri_scheme_protocol)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}