// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn api_login_command(user_name: String, user_password: String) {
  println!("start login process...");
  println!("login name: {}",user_name);
  println!("login password: {}",user_password);
}

fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
