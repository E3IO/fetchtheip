# Fetch Real IP Telegram Bot

这个 Rust 实现的 Telegram 机器人可以帮助用户获取其真实的公网 IP 地址，即使用户通过 SOCKS 代理连接到 Telegram。

## 功能特点

- 通过 SOCKS 代理连接 Telegram API
- 获取不经过代理的真实公网 IP 地址
- 支持多个 IP 查询服务，提高可靠性
- 返回 IP 地址及相关地理位置信息（如可用）

## 安装与配置

### 前提条件

- Rust 和 Cargo 已安装
- 已创建 Telegram 机器人并获取 API Token

### 配置步骤

1. 克隆此仓库
2. 编辑 `.env` 文件，填入以下信息：
   ```
   TELOXIDE_TOKEN=your_telegram_bot_token_here
   SOCKS_PROXY=socks5h://127.0.0.1:1080
   ```
   - `TELOXIDE_TOKEN`: 你的 Telegram 机器人 Token
   - `SOCKS_PROXY`: SOCKS 代理地址（格式为 `socks5h://host:port`）

### 编译与运行

```bash
cargo build --release
./target/release/fetch-real-ip
```

或者直接运行：

```bash
cargo run --release
```

## 使用方法

在 Telegram 中与机器人交互，支持以下命令：

- `/start` 或 `/help` - 显示帮助信息
- `/ip` - 获取你的真实公网 IP 地址

## 工作原理

1. 机器人通过配置的 SOCKS 代理连接到 Telegram API
2. 当用户发送 `/ip` 命令时，机器人创建一个不使用代理的 HTTP 客户端
3. 该客户端直接连接到公共 IP 查询服务（如 ipify.org 等）
4. 获取到的 IP 地址是用户的真实公网 IP，而非代理 IP
5. 机器人将查询结果返回给用户

## 注意事项

- 确保你的系统允许直接（不通过代理）访问互联网
- 如果所有 IP 查询服务都无法访问，机器人将返回错误信息
