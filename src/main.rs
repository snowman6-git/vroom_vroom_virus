// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use cli_clipboard;

fn main() {
    tauri::Builder::default()
    // .invoke_handler(tauri::generate_handler![link])
    .invoke_handler(tauri::generate_handler![copy])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}

#[tauri::command]
fn copy(){
    let the_string = "절대 바이러쓰 안힙니다 믿어 주세요 ... 전 반려부릉입니다.";
    cli_clipboard::set_contents(the_string.to_owned()).unwrap();
    assert_eq!(cli_clipboard::get_contents().unwrap(), the_string);
}
