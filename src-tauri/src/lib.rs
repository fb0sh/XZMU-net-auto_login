mod xzmu;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            xzmu::init_app,
            xzmu::save_account,
            xzmu::login,
            xzmu::test_xzmu_connection,
            xzmu::test_internet_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
