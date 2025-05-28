use oauth2::{
    basic::BasicClient, AuthUrl, TokenUrl, ClientId, ClientSecret,
    RedirectUrl, AuthorizationCode, reqwest::async_http_client, TokenResponse,
};
use reqwest;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc, Duration};
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginParams {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub email: String,
}

/// トークンを保存
pub async fn save_token(app: &AppHandle, access_token: &str, refresh_token: &str, expires_in: Duration, email: &str) -> Result<(), String> {
    let token = SavedToken {
        access_token: access_token.to_string(),
        refresh_token: refresh_token.to_string(),
        expires_at: Utc::now() + expires_in,
        email: email.to_string(),
    };

    let token_dir = get_token_dir(app)?;
    fs::create_dir_all(&token_dir)
        .map_err(|e| format!("トークンディレクトリの作成に失敗: {}", e))?;

    let token_path = token_dir.join("token.json");
    let token_json = serde_json::to_string_pretty(&token)
        .map_err(|e| format!("トークンのシリアライズに失敗: {}", e))?;

    fs::write(&token_path, token_json)
        .map_err(|e| format!("トークンの保存に失敗: {}", e))?;

    println!("トークンを保存しました: {}", token_path.to_string_lossy());
    Ok(())
}

/// 保存されたトークンを読み込む
async fn load_token(app: &AppHandle) -> Result<SavedToken, String> {
    let token_path = get_token_dir(app)?.join("token.json");
    
    if !token_path.exists() {
        return Err("保存されたトークンがありません".to_string());
    }

    let token_json = fs::read_to_string(token_path)
        .map_err(|e| format!("トークンの読み込みに失敗: {}", e))?;

    let token: SavedToken = serde_json::from_str(&token_json)
        .map_err(|e| format!("トークンのデシリアライズに失敗: {}", e))?;

    // トークンの有効期限をチェック
    if Utc::now() > token.expires_at {
        return Err("トークンの有効期限が切れています".to_string());
    }

    Ok(token)
}

/// トークンディレクトリのパスを取得
fn get_token_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_config_dir()
        .map_err(|e| format!("データディレクトリが見つかりません: {}", e))
}
/// 認可コードを使ってアクセストークンを取得
pub async fn exchange_code_for_token(app: &AppHandle, code: String, params: &LoginParams) -> Result<String, String> {
    let client = BasicClient::new(
        ClientId::new(params.client_id.clone()),
        Some(ClientSecret::new(params.client_secret.clone())),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
        Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(params.redirect_uri.clone()).unwrap());

    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await
        .map_err(|e| format!("トークン取得失敗: {}", e))?;

    let access_token = token_result.access_token().secret().to_string();
    let email = get_user_email(&access_token).await;

    // トークンを保存
    if let Some(refresh_token) = token_result.refresh_token() {
        save_token(
            app,
            &access_token,
            refresh_token.secret(),
            Duration::seconds(token_result.expires_in().unwrap_or(std::time::Duration::from_secs(3600)).as_secs() as i64),
            &email,
        ).await?;
    }

    Ok(access_token)
}

/// 保存されたトークンを使用してログイン
pub async fn login_with_saved_token(app: &AppHandle) -> Result<String, String> {
    let token = load_token(app).await?;
    Ok(token.access_token)
}

/// アクセストークンからユーザーのemailを取得
pub async fn get_user_email(access_token: &str) -> String {
    let client = reqwest::Client::new();
    let res = client
        .get("https://www.googleapis.com/oauth2/v1/userinfo?alt=json")
        .bearer_auth(access_token)
        .send()
        .await
        .expect("ユーザー情報取得失敗");

    let json: serde_json::Value = res.json().await.unwrap();
    json["email"]
        .as_str()
        .unwrap_or("unknown@example.com")
        .to_string()
}