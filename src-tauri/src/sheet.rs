use crate::drive;
use serde::{Serialize, Deserialize};
use reqwest::Client;
use serde_json::json;
use reqwest::multipart;


// pub struct TableType{
//     pub name: String,
//     pub start_time: String,
//     pub end_time: String,
//     pub role: Role,
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemberStatus {
    pub name: String,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Role {
    MANAGER,
    PARTTIME,
}

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

const CONFIG_FILE_NAME: &str = "shiftbuilder_config.json";
const BOOK_NAME: &str = "shiftbuilder_data";
//const MEMBER_LIST_SHEET_NAME: &str = "member_list";
const SHEET_URL: &str = "https://sheets.googleapis.com/v4/spreadsheets";

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

pub async fn upload_config_to_drive(access_token: &str, spreadsheet_id: &str) -> Result<(), String> {
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
