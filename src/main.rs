use std::env;
use anyhow::Result;
use dotenv::dotenv;
use log::{info, warn};
use reqwest::Client;
use reqwest::Proxy;
use serde::Deserialize;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "支持的命令:")]
enum Command {
    #[command(description = "显示此帮助信息")]
    Help,
    #[command(description = "获取你的真实公网IP地址")]
    Ip,
    #[command(description = "启动机器人")]
    Start,
}

#[derive(Deserialize, Debug)]
struct IpInfo {
    #[serde(rename = "ip")]
    ip: String,
    // 可选字段，取决于API返回的数据
    #[serde(rename = "country", default)]
    country: Option<String>,
    #[serde(rename = "city", default)]
    city: Option<String>,
    #[serde(rename = "isp", default)]
    isp: Option<String>,
}

async fn get_public_ip() -> Result<IpInfo> {
    // 创建一个不使用代理的客户端
    let client = Client::builder()
        .no_proxy() // 确保不使用任何代理
        .build()?;
    
    // 使用多个IP查询服务，如果一个失败可以尝试另一个
    let ip_services = [
        "https://api.ip.sb/jsonip",
        "https://api.myip.com",
        "https://ipinfo.io/json",
    ];
    
    // 尝试每个服务直到成功
    for service in ip_services {
        match client.get(service).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<IpInfo>().await {
                        Ok(ip_info) => return Ok(ip_info),
                        Err(e) => warn!("Failed to parse response from {}: {}", service, e),
                    }
                }
            },
            Err(e) => warn!("Failed to connect to {}: {}", service, e),
        }
    }
    
    // 如果所有服务都失败，返回一个错误
    Err(anyhow::anyhow!("Failed to get public IP from any service"))
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help | Command::Start => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?
        },
        Command::Ip => {
            let chat_id = msg.chat.id;
            
            // 发送"正在查询..."消息
            let processing_msg = bot
                .send_message(chat_id, "正在查询你的真实公网IP地址，请稍候...")
                .await?;
            
            // 获取IP信息
            match get_public_ip().await {
                Ok(ip_info) => {
                    // 构建响应消息
                    let mut response = format!("你的真实公网IP地址是: {}\n", ip_info.ip);
                    
                    // 添加额外信息（如果有）
                    if let Some(country) = ip_info.country {
                        response.push_str(&format!("国家/地区: {}\n", country));
                    }
                    if let Some(city) = ip_info.city {
                        response.push_str(&format!("城市: {}\n", city));
                    }
                    if let Some(isp) = ip_info.isp {
                        response.push_str(&format!("ISP: {}\n", isp));
                    }
                    
                    // 编辑之前的消息
                    bot.edit_message_text(chat_id, processing_msg.id, response).await?
                },
                Err(e) => {
                    // 如果查询失败，发送错误消息
                    bot.edit_message_text(
                        chat_id,
                        processing_msg.id,
                        format!("查询IP地址时出错: {}", e),
                    )
                    .await?
                },
            }
        },
    };
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // 加载.env文件
    dotenv().ok();
    
    // 初始化日志
    pretty_env_logger::init_timed();
    info!("Starting Telegram bot");
    
    // 获取环境变量
    let bot_token = env::var("TELOXIDE_TOKEN")
        .expect("TELOXIDE_TOKEN not set in .env file");
    let socks_proxy = env::var("SOCKS_PROXY")
        .expect("SOCKS_PROXY not set in .env file");
    
    // 创建带有SOCKS代理的客户端
    let client = Client::builder()
        .proxy(Proxy::all(socks_proxy)?)
        .build()?;
    
    // 创建带有自定义客户端的机器人
    let bot = Bot::with_client(bot_token, client);
    
    // 注册命令
    bot.set_my_commands(Command::bot_commands()).await?;
    
    // 启动机器人
    info!("Bot started successfully");
    Command::repl(bot, answer).await;
    
    Ok(())
}
