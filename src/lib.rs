#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 系统“另存为”对话框
        .plugin(tauri_plugin_dialog::init())
        // 把作品、图片、笔刷写入用户选择的位置
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("AkiStash 启动失败");
}
