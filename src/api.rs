use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{Value};

const URL_TRANSLATE: &str= "https://translate.google.com";

fn header_translate() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert("Accept-Charset", HeaderValue::from_static("UTF-8"));
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/4.0"));
    
    headers
}


pub async fn translate_japan_to_indonesia(japan: &str) -> String {
    let url_api = URL_TRANSLATE.to_string();
    let encoded = urlencoding::encode(japan);
    let url = format!("{}/translate_a/single?=&client=gtx&sl=ja&tl=in&q={}&dt=t", url_api, encoded);
    let headers = header_translate();

    let client = reqwest::Client::new();
    let response = client.get(url).headers(headers).send().await.unwrap();

    if response.status().is_success() {
        let body = response.text().await.unwrap();
        let json: Value = serde_json::from_str(&body).unwrap();
        let kotoba = json[0][0][0].as_str().unwrap().to_string();
        kotoba
    } else {
        "".to_string()
    }
}

pub async fn translate_japan_to_english(japan: &str) -> String {
    let url_api = URL_TRANSLATE.to_string();
    let encoded = urlencoding::encode(japan);
    let url = format!("{}/translate_a/single?=&client=gtx&sl=ja&tl=en&q={}&dt=t", url_api, encoded);
    let headers = header_translate();

    let client = reqwest::Client::new();
    let response = client.get(url).headers(headers).send().await.unwrap();

    if response.status().is_success() {
        let body = response.text().await.unwrap();
        let json: Value = serde_json::from_str(&body).unwrap();
        let kotoba = json[0][0][0].as_str().unwrap().to_string();
        kotoba
    } else {
        "".to_string()
    }
}

