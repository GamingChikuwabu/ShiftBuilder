use crate::drive;
use serde::{Serialize, Deserialize};
use reqwest::Client;
use serde_json::json;
use reqwest::multipart;
use std::collections::HashMap;

#[derive(Serialize)]
struct SpreadsheetProperties {
    title: String,
}

#[derive(Serialize)]
struct SpreadsheetCreateRequest {
    properties: SpreadsheetProperties,
}

#[derive(Deserialize, Debug)]
struct SpreadsheetCreateResponse {
    #[serde(rename = "spreadsheetId")]
    spreadsheet_id: String,
    #[serde(rename = "spreadsheetUrl")]
    spreadsheet_url: String,
}

/// シート情報の型
#[derive(Debug, Deserialize)]
struct SheetProperties {
    sheetId: i64,
    title: String,
}

#[derive(Debug, Deserialize)]
struct Sheet {
    properties: SheetProperties,
}

#[derive(Debug, Deserialize)]
struct SheetListResponse {
    sheets: Vec<Sheet>,
}

const CONFIG_FILE_NAME: &str = "shiftbuilder_config.json";
const BOOK_NAME: &str = "shiftbuilder_data";
//const MEMBER_LIST_SHEET_NAME: &str = "member_list";
const SHEET_URL: &str = "https://sheets.googleapis.com/v4/spreadsheets";

/// スプレッドシートを探すか作成する関数
/// 
/// # Arguments
/// 
/// * `access_token` - アクセストークン
/// 
/// # Returns
/// 成功したら `String` を返す
/// 失敗したら `String` を返す
pub async fn find_or_create_book(access_token: &String) -> Result<String, String> {
    let file_id = drive::get_file_id(access_token, CONFIG_FILE_NAME).await;
    if file_id.is_err() {
        println!("スプレッドシートが見つからないので作成します");
        let spreadsheet_id = create_book(access_token).await?;
        return Ok(spreadsheet_id);
    }
    else {
        println!("スプレッドシートが見つかったので読み込みます");
        let content = drive::get_file_content(access_token, &file_id.unwrap()).await?;
        let spreadsheet_id = drive::load_from_sheet_config(&content).await?;
        return Ok(spreadsheet_id);
    }
}

/// スプレッドシートを作成する関数
/// 
/// # Arguments
/// 
/// * `access_token` - アクセストークン
/// 
/// # Returns
/// 成功したら `String` を返す
/// 失敗したら `String` を返す
async fn create_book(access_token: &String) -> Result<String, String> {

    let client = Client::new();

    let body = SpreadsheetCreateRequest {
        properties: SpreadsheetProperties {
            title: BOOK_NAME.to_string(),
        },
    };

    let res = client
        .post(SHEET_URL)
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("リクエストに失敗: {}", e))?;

    if res.status().is_success() {
        let response_text = res.text().await
            .map_err(|e| format!("レスポンスの読み取りに失敗: {}", e))?;
        
        let created: SpreadsheetCreateResponse = serde_json::from_str(&response_text)
            .map_err(|e| format!("JSONの解析に失敗: {}\nレスポンス: {}", e, response_text))?;
        println!("📄 Created spreadsheet ID: {}", created.spreadsheet_id);
        println!("🔗 URL: {}", created.spreadsheet_url);
        upload_config_to_drive(access_token, &created.spreadsheet_id.to_string()).await?;
        Ok(created.spreadsheet_id)
    } else {
        let text = res.text().await
            .map_err(|e| format!("レスポンスの読み込みに失敗: {}", e))?;
        eprintln!("❌ Failed to create spreadsheet: {}", text);
        Err(format!("スプレッドシートの作成に失敗: {}", text))
    }


}

/// スプレッドシートの設定をDriveにアップロードする関数
/// 
/// # Arguments
/// 
/// * `access_token` - アクセストークン
/// * `spreadsheet_id` - スプレッドシートID
///
/// # Returns
/// 成功したら `()` を返す
/// 失敗したら `String` を返す
async fn upload_config_to_drive(access_token: &str, spreadsheet_id: &str) -> Result<(), String> {
    let client = Client::new();

    let metadata = json!({
        "name": CONFIG_FILE_NAME,
        "mimeType": "application/json",
        "parents": ["appDataFolder"]
    });

    let content = json!({
        "spreadsheet_id": spreadsheet_id
    });

    let form = multipart::Form::new()
        .part(
            "metadata",
            multipart::Part::text(metadata.to_string())
                .mime_str("application/json")
                .map_err(|e| format!("メタデータのMIMEタイプ設定に失敗: {}", e))?
        )
        .part(
            "media",
            multipart::Part::text(content.to_string())
                .mime_str("application/json")
                .map_err(|e| format!("コンテンツのMIMEタイプ設定に失敗: {}", e))?
        );

    let res = client
        .post("https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart")
        .bearer_auth(access_token)
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("アップロード失敗: {}", e))?;

    if !res.status().is_success() {
        let text = res.text().await.unwrap_or_default();
        return Err(format!("Drive保存失敗: {}", text));
    }

    Ok(())
}

/// スプレッドシートIDを取得する関数
/// 
/// # Arguments
/// 
/// * `access_token` - アクセストークン
/// 
/// # Returns
/// 成功したら `String` を返す
pub async fn get_book_id(access_token: &String) -> Result<String, String> {
    let client = Client::new();
    let url = format!("{}?q={}&spaces=appDataFolder&fields=files(id,name)", SHEET_URL, BOOK_NAME);
    let res = client
        .get(url)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| format!("リクエストに失敗: {}", e))?;
    if res.status().is_success() {
        let response_text = res.text().await
            .map_err(|e| format!("レスポンスの読み取りに失敗: {}", e))?;
        let created: SpreadsheetCreateResponse = serde_json::from_str(&response_text)
            .map_err(|e| format!("JSONの解析に失敗: {}\nレスポンス: {}", e, response_text))?;
        Ok(created.spreadsheet_id)
    }
    else {
        let text = res.text().await.unwrap_or_default();
        Err(format!("スプレッドシートの取得に失敗: {}", text))
    }
}

/// シートを追加する関数
/// 
/// # Arguments
/// 
/// * `access_token` - アクセストークン
/// * `book_id` - スプレッドシートID
/// * `sheet_name` - 追加するシート名
/// 
/// # Returns
/// 成功したら `i64` を返す
/// 失敗したら `Box<dyn std::error::Error>` を返す
pub async fn add_sheet(
    access_token: &String,
    book_id: &String, 
    sheet_name: &String) 
    -> Result<i64, Box<dyn std::error::Error>> {
        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}:batchUpdate",
            book_id
        );
    
        let body = json!({
            "requests": [
                {
                    "addSheet": {
                        "properties": {
                            "title": sheet_name
                        }
                    }
                }
            ]
        });
    
        let client = Client::new();
        let res = client
            .post(&url)
            .bearer_auth(access_token)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("リクエストに失敗: {}", e))?;
    
        if res.status().is_success() {
            let json: serde_json::Value = res.json().await?;
            let book_id = json["replies"][0]["addSheet"]["properties"]["sheetId"]
                .as_i64()
                .ok_or("Failed to get new sheetId")?;
            Ok(book_id)
        } else {
            let text = res.text().await
                .map_err(|e| format!("レスポンスの読み取りに失敗: {}", e))?;
            Err(format!("APIエラー: {}", text).into())
        }
}

/// シートを削除する関数
/// 
/// # Arguments
/// 
/// * `access_token` - アクセストークン
/// * `book_id` - スプレッドシートID
/// * `sheet_name` - 削除するシート名
/// 
/// # Returns
/// 成功したら `()` を返す
/// 失敗したら `String` を返す
pub async fn remove_sheet(
    access_token: &String, 
    book_id: &String, 
    sheet_name: &String) -> Result<(), String> {
        let client = Client::new();

        // 1. スプレッドシート情報を取得してシートIDを探す
        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}?fields=sheets.properties",
            book_id
        );
    
        let sheet_info: serde_json::Value = client
            .get(&url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| format!("リクエストに失敗: {}", e))?
            .json()
            .await
            .map_err(|e| format!("JSONの解析に失敗: {}", e))?;
    
        let sheets = sheet_info["sheets"]
            .as_array()
            .ok_or("sheetsが存在しません")?;
    
        let target_sheet = sheets.iter().find(|sheet| {
            sheet["properties"]["title"].as_str() == Some(sheet_name)
        });
    
        let sheet_id = match target_sheet {
            Some(sheet) => sheet["properties"]["sheetId"]
                .as_i64()
                .ok_or("sheetIdの取得に失敗しました")?,
            None => return Err(format!("シート '{}' が見つかりませんでした", sheet_name).into()),
        };
    
        // 2. 削除リクエスト送信
        let delete_url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}:batchUpdate",
            book_id
        );
    
        let body = json!({
            "requests": [
                {
                    "deleteSheet": {
                        "sheetId": sheet_id
                    }
                }
            ]
        });
    
        let res = client
            .post(&delete_url)
            .bearer_auth(access_token)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("リクエストに失敗: {}", e))?;
    
        if res.status().is_success() {
            Ok(())
        } else {
            let text = res.text().await
                .map_err(|e| format!("レスポンスの読み取りに失敗: {}", e))?;
            Err(format!("削除失敗: {}", text).into())
        }
}


/// スプレッドシート内の全シート名とIDを取得する関数
/// 
/// # Arguments
/// 
/// * `access_token` - アクセストークン
/// * `book_id` - スプレッドシートID
/// 
/// # Returns
/// シート名 → シートID の HashMap
pub async fn get_sheet_list(
    access_token: &str,
    book_id: &str,
) -> Result<HashMap<String, i64>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}?fields=sheets.properties",
        book_id
    );

    let client = Client::new();
    let res = client
        .get(&url)
        .bearer_auth(access_token)
        .send()
        .await?;

    if !res.status().is_success() {
        let err = res.text().await?;
        return Err(format!("シート一覧取得失敗: {}", err).into());
    }

    let sheet_list: SheetListResponse = res.json().await?;

    // 名前 → ID の HashMap に変換
    let sheet_map = sheet_list
        .sheets
        .into_iter()
        .map(|s| (s.properties.title, s.properties.sheetId))
        .collect();

    Ok(sheet_map)
}