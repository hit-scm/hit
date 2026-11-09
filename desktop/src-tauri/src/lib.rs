pub mod commands;

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::open_repository,
            commands::get_status,
            commands::get_file_diff,
            commands::stage_file,
            commands::stage_all,
            commands::unstage_file,
            commands::unstage_all,
            commands::commit,
            commands::sync_repo,
            commands::push_repo,
            commands::pull_repo,
            commands::fetch_repo,
            commands::get_history,
            commands::get_branches,
            commands::switch_branch,
            commands::get_submodules,
            commands::update_submodules,
            commands::get_auth_state,
            commands::login_account,
            commands::logout_account,
            commands::run_doctor,
            commands::open_browser,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Hit Desktop application");
}
