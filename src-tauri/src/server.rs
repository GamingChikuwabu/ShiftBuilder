use tiny_http::{Server, Response};
use url::Url;
use std::net::TcpListener;

const REDIRECT_PORT: u16 = 8080;

/// ポートが使用可能か確認
fn is_port_available(port: u16) -> bool {
    TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok()
}

/// ローカルサーバーでGoogleからのリダイレクトを受け取り、認可コードを取得
pub fn wait_for_redirect_code() -> String {
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