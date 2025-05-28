use tauri::{AppHandle, Manager};
use crate::auth;
use crate::sheet;

pub async fn init_app(app: &AppHandle) -> Result<(), String> {
    //初回起動かの判定
    let is_logged_in = app.path().app_config_dir().unwrap().join("token.json").exists();
    if is_logged_in {
        //アクセストークンの取得
        let access_token = init_auth_access_token(app).await?;
        let email = auth::get_user_email(&access_token).await;
        println!("ログインしたユーザー: {}", email);
        //スプレッドシートの初期化
        let spreadsheet_id = init_check_spreadsheet(&access_token).await?;
        println!("スプレッドシートのID: {}", spreadsheet_id);
    }
    else {
        println!("初回起動なのでログイン画面を表示します");
    }
    Ok(())
}

async fn init_auth_access_token(app: &AppHandle) -> Result<String, String> {
    let access_token = auth::login_with_saved_token(app).await?;
    Ok(access_token)
}

async fn init_check_spreadsheet(access_token: &String) -> Result<String, String> {
    let spreadsheet_id = sheet::find_or_create_book(&access_token).await?;
    Ok(spreadsheet_id)
}