use reqwest::Client;

const DRIVE_URL: &str = "https://www.googleapis.com/drive/v3/files";



/// ファイル名に沿ったファイルを探してIDを返す
/// 
/// # Arguments
/// 
/// * `access_token` - アクセストークン
/// * `name` - ファイル名
/// 
/// # Returns
/// 
/// * `String` - ファイルID
/// * `String` - エラー
pub async fn get_file_id(access_token: &str,name: &str) -> Result<String, String> {
    let client = Client::new();
    // 1. ファイルを探す
    let query = format!("name = '{}' and 'appDataFolder' in parents", name);
    let list_url = format!(
        "{}?q={}&spaces=appDataFolder&fields=files(id,name)",
        DRIVE_URL,
        urlencoding::encode(&query)
    );

    let res = client
        .get(&list_url)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| format!("Drive検索失敗: {}", e))?;

    let list_json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    let file_id = list_json["files"]
        .as_array()
        .and_then(|arr| arr.first())
        .and_then(|f| f["id"].as_str())
        .ok_or("ファイルが見つかりません")?;

    Ok(file_id.to_string())
}

/// ファイルIDに沿ったファイルの内容を取得する
/// 
/// # Arguments
/// 
/// * `access_token` - アクセストークン
/// * `file_id` - ファイルID
pub  async fn get_file_content(access_token: &str,file_id: &str) -> Result<String, String> {
    let client = Client::new();
    let content_url = format!(
        "https://www.googleapis.com/drive/v3/files/{}?alt=media",
        file_id
    );

    let res = client
        .get(&content_url)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| format!("ファイル取得失敗: {}", e))?;

    let content = res.text().await.map_err(|e| e.to_string())?;

    Ok(content)
}


pub async fn load_from_sheet_config(content: &str) -> Result<String, String> {
    let config: serde_json::Value = serde_json::from_str(content).map_err(|e| e.to_string())?;
    let spreadsheet_id = config["spreadsheet_id"]
        .as_str()
        .ok_or("spreadsheet_idが見つかりません")?
        .to_string();

    Ok(spreadsheet_id)
}