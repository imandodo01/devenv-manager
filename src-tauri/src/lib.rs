mod environment;

#[tauri::command]
fn get_environment_inventory() -> environment::InventorySnapshot {
    environment::collect_inventory()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_environment_inventory])
        .run(tauri::generate_context!())
        .expect("failed to run DevEnv Manager");
}

#[cfg(test)]
mod tests {
    #[test]
    fn application_crate_has_expected_identity() {
        assert_eq!(env!("CARGO_PKG_NAME"), "devenv-manager");
    }
}
