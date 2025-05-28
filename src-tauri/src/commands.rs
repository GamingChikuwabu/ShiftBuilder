use tauri::{command, Manager};
use crate::auth;
use crate::server;
use crate::starter;

/// Tauriコマンド：ログイン処理
#[command]
pub async fn start_google_login(params: auth::LoginParams, app: tauri::AppHandle) -> Result<(), String> {
    println!("Googleログインを開始します...");
    
    // 認可コードを別スレッドで取得
    let code = tokio::task::spawn_blocking(move || server::wait_for_redirect_code())
        .await
        .unwrap();

    match code.as_str() {
        "port_in_use" => return Err("ポート8080が既に使用されています。他のアプリケーションを終了してください。".into()),
        "server_error" => return Err("サーバーの起動に失敗しました。".into()),
        "url_parse_error" => return Err("URLの解析に失敗しました。".into()),
        "missing_code" | "error_code" => return Err("認証コードの取得に失敗しました。".into()),
        _ => {}
    }

    // アクセストークン取得
    let access_token = auth::exchange_code_for_token(&app, code, &params).await?;

    println!("アクセストークン: {}", access_token);

    // アプリケーションを初期化
    starter::init_app(&app).await?;

    Ok(())
}

#[command]
pub fn is_logged_in(app: tauri::AppHandle) -> bool {
    let token_path = app.path().app_config_dir().unwrap().join("token.json");
    token_path.exists()
}

#[command]
pub fn logout(app: tauri::AppHandle) -> Result<(), String> {
    let token_path = app.path().app_config_dir()
        .map_err(|e| format!("設定ディレクトリの取得に失敗: {}", e))?
        .join("token.json");
    
    if token_path.exists() {
        std::fs::remove_file(token_path)
            .map_err(|e| format!("トークンファイルの削除に失敗: {}", e))?;
    }
    Ok(())
}

#[command]
pub fn get_current_user(app: tauri::AppHandle) -> Result<String, String> {
    let token_path = app.path().app_config_dir()
        .map_err(|e| format!("設定ディレクトリの取得に失敗: {}", e))?
        .join("token.json");
    
    if !token_path.exists() {
        return Err("ログインしていません".to_string());
    }

    let token_json = std::fs::read_to_string(token_path)
        .map_err(|e| format!("トークンの読み込みに失敗: {}", e))?;

    let token: auth::SavedToken = serde_json::from_str(&token_json)
        .map_err(|e| format!("トークンのデシリアライズに失敗: {}", e))?;

    // トークンの有効期限をチェック
    if chrono::Utc::now() > token.expires_at {
        return Err("トークンの有効期限が切れています".to_string());
    }

    Ok(token.email)
}