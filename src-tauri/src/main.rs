// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use tauri::{webview::{WebviewWindowBuilder, PageLoadEvent}, utils::config::WebviewUrl};
use regex::Regex;
use reqwest::{Client};
use app_lib::{models::login_info::LoginInfo, user_client::{self, UserClient}};



// 自定义错误类型（不变）
#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("HTTP请求错误: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("正则匹配失败: {0}")]
    RegexError(#[from] regex::Error),
    #[error("Cookie不存在: {0}")]
    CookieNotFound(String),
    #[error("响应头Location不存在")]
    LocationHeaderNotFound,
    #[error("登录逻辑已过期（超过2025-07-20）")]
    LoginExpired,
    #[error("参数解析失败: {0}")]
    ParamParseError(String),
}

async fn qq_login(url: &str) -> Option<LoginInfo> {
        if let Some(pos) = url.find("code=") {
            let code = &url[pos..];
            let login_url = format!("https://web2.17roco.qq.com/fcgi-bin/login3?{}&platfrom_src=2", code);
            let client = Client::new();
            let response = client.get(&login_url).send().await.unwrap();
            println!("response: {:?}", response);
            if response.status().is_success() {
                    let result = response.text().await.unwrap_or_default();
                    let flash_vars_re = Regex::new(r#"flashVars="([^"]*)""#).unwrap();
                    if let Some(caps) = flash_vars_re.captures(&result) {
                        let mut login_info = LoginInfo::default();
                        login_info.config = caps[1].to_string();
                        let config_re = Regex::new(r"config=([^&]+)&angel_uin=([^&]+)&angel_key=([^&]+)&skey=([^&]+)&pskey=([^&]+)").unwrap();
                        
                        if let Some(inner_caps) = config_re.captures(&login_info.config) {
                            login_info.angel_uin = inner_caps[2].to_string();
                            login_info.angel_key = inner_caps[3].to_string();
                            login_info.skey = inner_caps[4].to_string();
                            login_info.pskey = inner_caps[5].to_string();
                            login_info.key = format!(
                                "angel_uin={}&angel_key={}&skey={}&unkown={}",
                                login_info.angel_uin, login_info.angel_key, login_info.skey, login_info.pskey
                            );
                            
                            println!("config: {}", login_info.config);
                            println!("uin: {} key: {} skey: {} pskey: {}", 
                                login_info.angel_uin, login_info.angel_key, login_info.skey, login_info.pskey);
                            
                            return Some(login_info);
                        }
                    }
                }
        }
    None
}

/// # 为前端提供用户登录所需的窗口
/// 作者: 陈慢慢
#[tauri::command]
async fn navigate_to_login(app: tauri::AppHandle) {
            WebviewWindowBuilder::new(&app, "core", WebviewUrl::App("https://graph.qq.com/oauth2.0/authorize?response_type=code&client_id=102061779&redirect_uri=https%3A%2F%2F17roco.qq.com%2Flogintarget.html&scope=all".into()))
            .title("请先登录")
            // .additional_browser_args("--disable-web-security,--allow-running-insecure-content,--disable-features=CrossSiteDocumentBlockingIfIsolating")
            .on_navigation(|url| {
                let url_string = url.as_str().to_string();
                // // 检查URL是否包含特定的登录目标地址
                if url_string.contains("17roco.qq.com/logintarget.html?code") {
                    tauri::async_runtime::spawn(async move {
                        // 直接使用这个URL调用登录函数
                        let login_info = qq_login(&url_string).await;
                        if let Some(ref login_info) = login_info {
                            // 登录成功，关闭窗口
                            let client = UserClient::new(login_info.clone());
                            // 1. 连接并拿到接收器 (就像订阅 OnReceive 事件)
                            let (client_arc, reader) = client.connect(100).await.expect("Connect failed");
                            
                            UserClient::start_receive_task(Arc::clone(&client_arc), reader).await;

                            UserClient::start_heartbeat(Arc::clone(&client_arc));
                        }
                        
                        
                    });
                }
                true
            })
            .on_page_load(|window, event| {
            // 仅在页面完全加载后执行（DOMContentLoaded）
            if let PageLoadEvent::Finished = event.event() {
                // 注入Ruffle的核心代码（CDN方式，无需本地资源）
                let ruffle_inject_js = r#"
                    (function() {
                        const script = document.createElement('script');
                        script.src = 'https://unpkg.com/@ruffle-rs/ruffle';
                        script.type = 'text/javascript';
                        // 2. Ruffle加载完成后配置自动替换Flash标签
                        script.onload = function() {
                            window.RufflePlayer = window.RufflePlayer || {};
                            window.RufflePlayer.config = {
                                allowFileAccess: false, // 远程页面无需本地文件访问
                                autoplay: "on",
                                logLevel: "warn", // 降低日志级别
                                // 强制替换所有Flash标签
                                polyfill: true,
                                // 核心配置：忽略跨域iframe，避免扫描时触发跨域错误
                                ignoreCrossOriginIframes: true,
                                // 可选：仅扫描主文档，不扫描子框架
                                scanOnlyMainDocument: true
                            };
                            // 手动触发Ruffle的自动检测（确保替换生效）
                            window.RufflePlayer.polyfill();
                        };
                        // 3. 将脚本插入页面头部
                        document.head.appendChild(script);
                    })();
                "#;

                // 执行注入的JS代码
                let _ = window.eval(ruffle_inject_js);
                println!("已为远程页面注入Ruffle脚本");
            }
        })
            .build()
            .unwrap();
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![navigate_to_login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
  
}
