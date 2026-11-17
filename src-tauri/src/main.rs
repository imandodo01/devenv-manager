// Prevents an additional console window from opening on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    devenv_manager_lib::run();
}
