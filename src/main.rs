// 发布版在 Windows 上不弹出黑色命令行窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    akistash_lib::run()
}
