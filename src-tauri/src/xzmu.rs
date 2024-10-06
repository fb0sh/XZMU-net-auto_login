use regex::Regex;
use reqwest::{self, header};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use std::{fs::File, io::BufReader};

use tauri::{AppHandle, Manager, Runtime};
use url::Url;
#[derive(Debug, Serialize, Deserialize)]
pub struct XZMUAccount {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XZMUNetConfig {
    wlan_user_ip: String,
    wlan_user_mac: String,
    wlan_ac_ip: String,
    wlan_ac_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XZMUM {
    account: Option<XZMUAccount>,
    config: Option<XZMUNetConfig>,
}

#[tauri::command]
pub async fn init_app<R: Runtime>(app: tauri::AppHandle<R>) -> Result<XZMUM, String> {
    let mut xzmu = XZMUM {
        account: None,
        config: None,
    };

    match get_xzmu_config(&app) {
        Ok(config_path) => {
            xzmu.account =
                read_credentials(&config_path).map_err(|e| e.to_string() + " | Init config")?;
        }
        Err(_) => {
            xzmu.account = None;
        }
    }

    let login_url = get_login_url().await?;
    xzmu.config = Some(parse_login_url(&login_url)?);

    Ok(xzmu)
}

#[tauri::command]
pub async fn test_xzmu_connection() -> bool {
    let test_url = "http://120.95.80.23:8080/Self/login/";
    // 创建带有超时设置的客户端
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(1000)) // 设置超时为 1.5 秒
        .build();

    match client {
        Ok(client) => match client.get(test_url).send().await {
            Ok(res) => {
                println!("{:?}", res);
                true
            }
            Err(_) => false,
        },
        Err(_) => false,
    }
}

#[tauri::command]
pub async fn test_internet_connection() -> bool {
    let test_url = "http://www.163.com/";
    // 创建带有超时设置的客户端
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(1000)) // 设置超时为 1.5 秒
        .build();

    match client {
        Ok(client) => match client.get(test_url).send().await {
            Ok(res) => {
                let body = res.text().await.unwrap();
                if body.contains("http://10.1.0.212") {
                    return false;
                }
                true
            }
            Err(_) => false,
        },
        Err(_) => false,
    }
}

pub fn get_xzmu_config<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map_err(|e| e.to_string())
        .and_then(|path| Ok(path.join("xzmu_auto_login.json")))
}

pub fn read_credentials(config_path: &PathBuf) -> Result<Option<XZMUAccount>, String> {
    if !config_path.exists() {
        return Ok(None);
    }
    let file_reader = BufReader::new(File::open(config_path).map_err(|e| e.to_string())?);
    let account: XZMUAccount = serde_json::from_reader(file_reader).map_err(|e| e.to_string())?;
    Ok(Some(account))
}

pub async fn get_login_url() -> Result<String, String> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "User-Agent",
        header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36"),
    );

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .timeout(Duration::from_secs(2))
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .get("http://10.10.0.163/")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body = response.text().await.map_err(|e| e.to_string())?;

    if !body.contains("http://10.1.0.212?wlanusermac=") {
        return Err("Error: Login URL not found in the response body!".to_string());
    }

    let re = Regex::new(r#"location\.href="(.*?)""#).map_err(|e| e.to_string())?;
    if let Some(login_url) = re.captures(&body) {
        Ok(login_url[1].to_string())
    } else {
        Err("Error: Could not extract login URL!".to_string())
    }
}

pub fn parse_login_url(url: &str) -> Result<XZMUNetConfig, String> {
    let s = Url::parse(url).map_err(|e| e.to_string())?;

    let a: HashMap<Cow<str>, Cow<str>> = s.query_pairs().collect();
    Ok(XZMUNetConfig {
        wlan_user_ip: a
            .get("wlanuserip")
            .ok_or("wlanuserip not found")?
            .to_string(),
        wlan_user_mac: a
            .get("wlanusermac")
            .ok_or("wlanusermac not found")?
            .to_string(),
        wlan_ac_ip: a.get("wlanacip").ok_or("wlanacip not found")?.to_string(),
        wlan_ac_name: a
            .get("wlanacname")
            .ok_or("wlanacname not found")?
            .to_string(),
    })
}

#[tauri::command]
pub async fn save_account<R: Runtime>(
    app: tauri::AppHandle<R>,
    username: String,
    password: String,
) -> Result<(), String> {
    let config_path = get_xzmu_config(&app).map_err(|e| e.to_string())?;
    let account = XZMUAccount { username, password };
    let file_writer = File::create(config_path).map_err(|e| e.to_string())?;
    serde_json::to_writer(file_writer, &account).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn login(account: XZMUAccount, config: XZMUNetConfig) -> Result<String, String> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "User-Agent",
        header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36"),
    );
    headers.insert(
        "Referer",
        header::HeaderValue::from_static("http://10.1.0.212/"),
    );
    let login_url = format!(
        "http://10.1.0.212:801/eportal/portal/login?callback=dr1003&login_method=1&user_account=,0,{}&user_password={}&wlan_user_ip={}&wlan_user_ipv6=&wlan_user_mac={}&wlan_ac_ip={}&wlan_ac_name={}&jsVersion=4.2&terminal_type=1&lang=zh-cn&v=2833&lang=zh"
        ,account.username
        ,account.password
        ,config.wlan_user_ip
        ,config.wlan_user_mac.replace("-","")
        ,config.wlan_ac_ip
        ,config.wlan_ac_name
    );

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .timeout(Duration::from_secs(2))
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .get(&login_url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body = response
        .text()
        .await
        .map_err(|e| e.to_string() + " | login body")?;

    Ok(body)
}
