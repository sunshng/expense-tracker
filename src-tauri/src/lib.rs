mod database;

use database::{
    AddTransactionRequest, Database, FilterParams, UpdateTransactionRequest,
};
use tauri::{Manager, State};
use std::sync::Mutex;

/// 应用状态：持有数据库实例
struct AppState {
    db: Database,
}

// ==================== Tauri 命令处理器 ====================

/// 获取分类列表
#[tauri::command]
fn get_categories(state: State<'_, Mutex<AppState>>) -> Result<Vec<database::Category>, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    app_state.db.get_categories()
}

/// 获取账单列表
#[tauri::command]
fn get_transactions(
    state: State<'_, Mutex<AppState>>,
    filters: FilterParams,
) -> Result<Vec<database::Transaction>, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    app_state.db.get_transactions(filters)
}

/// 添加账单
#[tauri::command]
fn add_transaction(
    state: State<'_, Mutex<AppState>>,
    transaction: AddTransactionRequest,
) -> Result<database::Transaction, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    app_state.db.add_transaction(transaction)
}

/// 删除账单
#[tauri::command]
fn delete_transaction(state: State<'_, Mutex<AppState>>, id: i64) -> Result<bool, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    app_state.db.delete_transaction(id)
}

/// 更新账单
#[tauri::command]
fn update_transaction(
    state: State<'_, Mutex<AppState>>,
    id: i64,
    transaction: UpdateTransactionRequest,
) -> Result<bool, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    app_state.db.update_transaction(id, transaction)
}

/// 获取统计数据
#[tauri::command]
fn get_statistics(
    state: State<'_, Mutex<AppState>>,
    start_date: Option<String>,
    end_date: Option<String>,
) -> Result<database::Statistics, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    app_state.db.get_statistics(start_date, end_date)
}

// ==================== 应用入口 ====================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // 获取应用数据目录作为数据库存储路径
            let app_data_dir = app.path().app_data_dir().expect("无法获取数据目录");
            let db_path = app_data_dir.join("expense-tracker.db");

            // 初始化数据库
            let db = Database::new(db_path)
                .expect("数据库初始化失败");

            // 将数据库实例注册为应用状态
            app.manage(Mutex::new(AppState { db }));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_categories,
            get_transactions,
            add_transaction,
            delete_transaction,
            update_transaction,
            get_statistics,
        ])
        .run(tauri::generate_context!())
        .expect("启动应用失败");
}
