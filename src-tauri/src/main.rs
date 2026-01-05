// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{webview::WebviewWindowBuilder, utils::config::WebviewUrl};


/// # 为前端提供用户登录所需的窗口
/// 作者: 陈慢慢
#[tauri::command]
async fn navigate_to_login(app: tauri::AppHandle) {
            WebviewWindowBuilder::new(&app, "core", WebviewUrl::App("https://17roco.qq.com/login.html".into()))
            .title("请先登录")
            .on_navigation(|url| {
                println!("导航到: {}", url);
                true
            })
            .build()
            .unwrap();
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![navigate_to_login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
  
}
