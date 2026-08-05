/**
 * @file    notion.rs
 * @brief   Fetches Blueprint entries (with nested sub-goals) from the Notion
 *          "LifeFarm BluePrint" page.
 *
 * @author  hansaem, oh (praivesi@gmail.com)
 * @date    2026/08/06 created.
 *
**/
use std::env;

use chrono::{FixedOffset, NaiveDate, TimeZone};
use log::warn;
use serde_json::Value;

use crate::util::http_client;

const NOTION_PAGE_ID: &str = "3b339935544c80458bbeee8cb7c07fd7";
const NOTION_VERSION: &str = "2022-06-28";
const FIELD_LABELS: [&str; 3] = ["desc", "start_dt", "end_dt"];

pub struct NotionBlueprint {
    pub goal: String,
    pub desc: String,
    pub start_dt: i64,
    pub end_dt: i64,
    pub children: Vec<NotionBlueprint>,
}

fn kst() -> FixedOffset {
    FixedOffset::east_opt(9 * 3600).unwrap()
}

fn notion_token() -> Option<String> {
    env::var("NOTION_TOKEN").ok().filter(|token| !token.is_empty())
}

async fn fetch_children(block_id: &str) -> Vec<Value> {
    let Some(token) = notion_token() else {
        return Vec::new();
    };

    let mut results = Vec::new();
    let mut cursor: Option<String> = None;

    loop {
        let url = match &cursor {
            Some(c) => format!("https://api.notion.com/v1/blocks/{}/children?page_size=100&start_cursor={}", block_id, c),
            None => format!("https://api.notion.com/v1/blocks/{}/children?page_size=100", block_id),
        };

        let auth_header = format!("Bearer {}", token);
        let headers = [
            ("Authorization", auth_header.as_str()),
            ("Notion-Version", NOTION_VERSION),
        ];

        let body = match http_client::get_with_headers(&url, &headers).await {
            Ok(body) => body,
            Err(status) => {
                warn!("Notion API request failed. (url: {}, status: {})", url, status);
                break;
            }
        };

        let parsed: Value = match serde_json::from_str(&body) {
            Ok(v) => v,
            Err(e) => {
                warn!("Failed to parse Notion response as JSON: {}", e);
                break;
            }
        };

        if let Some(arr) = parsed.get("results").and_then(|v| v.as_array()) {
            results.extend(arr.clone());
        }

        if parsed.get("has_more").and_then(|v| v.as_bool()) != Some(true) {
            break;
        }
        cursor = parsed.get("next_cursor").and_then(|v| v.as_str()).map(|s| s.to_string());
        if cursor.is_none() {
            break;
        }
    }

    results
}

fn block_type(block: &Value) -> &str {
    block.get("type").and_then(|v| v.as_str()).unwrap_or("")
}

fn block_text(block: &Value) -> String {
    let block_type = block_type(block);
    let rich_text = block.get(block_type).and_then(|v| v.get("rich_text")).and_then(|v| v.as_array());

    match rich_text {
        Some(items) => items
            .iter()
            .filter_map(|rt| rt.get("plain_text").and_then(|v| v.as_str()))
            .collect::<Vec<_>>()
            .join(""),
        None => String::new(),
    }
}

fn is_goal_block(block: &Value) -> bool {
    matches!(block_type(block), "to_do" | "bulleted_list_item" | "toggle")
}

fn has_children(block: &Value) -> bool {
    block.get("has_children").and_then(|v| v.as_bool()).unwrap_or(false)
}

fn block_id(block: &Value) -> String {
    block.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string()
}

// 라벨 블록(desc/start_dt/end_dt)의 자식들을 모두 읽어 줄바꿈으로 합친다.
async fn fetch_field_value(field_block: &Value) -> String {
    if !has_children(field_block) {
        return String::new();
    }

    let children = fetch_children(&block_id(field_block)).await;

    children
        .iter()
        .map(block_text)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn date_to_kst_epoch(date_str: &str) -> Option<i64> {
    let date = NaiveDate::parse_from_str(date_str.trim(), "%Y-%m-%d").ok()?;
    let datetime = date.and_hms_opt(0, 0, 0)?;

    kst().from_local_datetime(&datetime).single().map(|dt| dt.timestamp())
}

// goal_block: to_do 또는 bulleted_list_item. 텍스트가 goal이고,
// 자식들 중 desc/start_dt/end_dt 라벨은 필드로, 그 외 goal형 블록은 하위 goal로 재귀 처리한다.
async fn parse_goal_block(goal_block: &Value) -> Option<NotionBlueprint> {
    let goal = block_text(goal_block).trim().to_string();
    if goal.is_empty() {
        return None;
    }

    let mut desc = String::new();
    let mut start_dt_raw = String::new();
    let mut end_dt_raw = String::new();
    let mut children = Vec::new();

    for field_block in fetch_children(&block_id(goal_block)).await {
        let is_field_label = block_type(&field_block) == "bulleted_list_item"
            && FIELD_LABELS.contains(&block_text(&field_block).as_str());

        if is_field_label {
            let label = block_text(&field_block);
            let value = fetch_field_value(&field_block).await;
            match label.as_str() {
                "desc" => desc = value,
                "start_dt" => start_dt_raw = value,
                "end_dt" => end_dt_raw = value,
                _ => {}
            }
        } else if is_goal_block(&field_block) {
            if let Some(child) = Box::pin(parse_goal_block(&field_block)).await {
                children.push(child);
            }
        }
    }

    if desc.is_empty() || start_dt_raw.is_empty() || end_dt_raw.is_empty() {
        warn!("Skipping Notion blueprint '{}': missing fields", goal);
        return None;
    }

    let (Some(start_dt), Some(end_dt)) = (date_to_kst_epoch(&start_dt_raw), date_to_kst_epoch(&end_dt_raw)) else {
        warn!("Skipping Notion blueprint '{}': invalid date format", goal);
        return None;
    };

    Some(NotionBlueprint { goal, desc, start_dt, end_dt, children })
}

pub async fn fetch_blueprints() -> Result<Vec<NotionBlueprint>, String> {
    if notion_token().is_none() {
        return Err("NOTION_TOKEN is not configured".to_string());
    }

    let top_blocks = fetch_children(NOTION_PAGE_ID).await;
    let mut blueprints = Vec::new();

    for block in &top_blocks {
        if !is_goal_block(block) {
            continue;
        }

        if let Some(blueprint) = parse_goal_block(block).await {
            blueprints.push(blueprint);
        }
    }

    Ok(blueprints)
}
