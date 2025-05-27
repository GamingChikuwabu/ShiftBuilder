use std::thread;
use tiny_http::{Server, Response};
use url::Url;
use oauth2::{
    basic::BasicClient, AuthUrl, TokenUrl, ClientId, ClientSecret,
    RedirectUrl, AuthorizationCode, reqwest::async_http_client, TokenResponse,
};
use reqwest;
use serde::Deserialize;
use tauri::command;
use std::net::TcpListener;

const REDIRECT_PORT: u16 = 8080;

#[derive(Deserialize)]
struct LoginParams {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

/// ポートが使用可能か確認
fn is_port_available(port: u16) -> bool {
    TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok()
}

/// ローカルサーバーでGoogleからのリダイレクトを受け取り、認可コードを取得
fn wait_for_redirect_code() -> String {
    // ポートが使用可能か確認
    if !is_port_available(REDIRECT_PORT) {
        println!("ポート{}は既に使用されています", REDIRECT_PORT);
        return "port_in_use".to_string();
    }

    let server = match Server::http(format!("127.0.0.1:{}", REDIRECT_PORT)) {
        Ok(s) => s,
        Err(e) => {
            println!("サーバー起動エラー: {}", e);
            return "server_error".to_string();
        }
    };

    println!("ローカルサーバーが起動しました: http://localhost:{}", REDIRECT_PORT);

    let code = match server.recv() {
        Ok(request) => {
            println!("リクエストを受信しました: {}", request.url());
            let url = format!("http://localhost{}", request.url());
            let parsed = match Url::parse(&url) {
                Ok(u) => u,
                Err(e) => {
                    println!("URL解析エラー: {}", e);
                    return "url_parse_error".to_string();
                }
            };

            let code = parsed
                .query_pairs()
                .find(|(k, _)| k == "code")
                .map(|(_, v)| v.to_string())
                .unwrap_or_else(|| "missing_code".to_string());

            let html = r#"
                <html>
                    <head>
                        <title>ログイン完了</title>
                        <style>
                            body { font-family: sans-serif; text-align: center; margin-top: 50px; }
                            h1 { color: #4CAF50; }
                        </style>
                    </head>
                    <body>
                        <h1>ログインが完了しました</h1>
                        <p>このウィンドウを閉じてください。</p>
                    </body>
                </html>
            "#;
            if let Err(e) = request.respond(Response::from_string(html).with_status_code(200)) {
                println!("レスポンス送信エラー: {}", e);
            }
            code
        }
        Err(e) => {
            println!("リダイレクト受信エラー: {}", e);
            "error_code".to_string()
        }
    };

    // サーバーをシャットダウン
    drop(server);
    code
}

/// 認可コードを使ってアクセストークンを取得
async fn exchange_code_for_token(code: String, params: &LoginParams) -> String {
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
        .expect("トークン取得失敗");

    token_result.access_token().secret().to_string()
}

/// アクセストークンからユーザーのemailを取得
async fn get_user_email(access_token: &str) -> String {
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

/// Tauriコマンド：ログイン処理
#[command]
async fn start_google_login(params: LoginParams) -> Result<String, String> {
    println!("Googleログインを開始します...");
    
    // 認可コードを別スレッドで取得
    let code = tokio::task::spawn_blocking(move || wait_for_redirect_code())
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
    let token = exchange_code_for_token(code, &params).await;

    // ユーザーemail取得
    let email = get_user_email(&token).await;

    println!("ログインしたユーザー: {}", email);

    Ok(email)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_google_login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}