use std::sync::{Arc};
use tauri::async_runtime::block_on;
use tokio::sync::Mutex;
use tokio::net::{TcpStream, tcp::OwnedReadHalf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time;
use std::time::{Duration}; 
use reqwest::Client;
use roxmltree::Document;


// 假设这些结构体已经在你的 models 中定义
 use crate::models::{game_info::GameInfo, connect_info::ConnectInfo, login_info::LoginInfo};

pub struct UserClient {
    pub uin: String,
    pub angel_key: String,
    pub game_info: GameInfo,
    pub connect_info: ConnectInfo,
    pub login_info: Arc<Mutex<LoginInfo>>,
    pub is_online: bool,
    // 移除了内部 buffer，因为解析逻辑可以放在后台任务里
    tcp_writer: Option<tokio::net::tcp::OwnedWriteHalf>, // 只保留写的一半
}

impl UserClient {
    pub fn new(login_info: LoginInfo) -> Self {
        let uin_hex_string = login_info.angel_uin
            .parse::<u32>()           // 解析为数值
            .map(|n| format!("{:08X}", n)) // 格式化为16进制字符串
            .unwrap_or_else(|_| "00000000".to_string()); // 失败保底
        let key = login_info.angel_key.clone();
        Self {
            uin: uin_hex_string,
            angel_key: key,
            game_info: GameInfo::default(),
            connect_info: ConnectInfo::default(),
            login_info: Arc::new(Mutex::new(login_info)),
            is_online: false,
            tcp_writer: None,
        }
    }

    pub async fn connect(mut self, channel_id: u32) -> Result<(Arc<Mutex<Self>>, OwnedReadHalf), Box<dyn std::error::Error>> {
        let server_ip = match channel_id {
            1..=50 => "221.181.80.76",
            51..=100 => "221.181.80.169",
            101..=150 => "221.181.81.72",
            _ => "221.181.80.76",
        };
        let server_id = match channel_id {
            1..=50 => 5,
            51..=100 => 6,
            101..=150 => 7,
            _ => 5,
        };
        let address = format!("{}:443", server_ip);
        let stream = TcpStream::connect(address).await?;
        stream.set_nodelay(true)?;

        let (reader, writer) = stream.into_split();

                // 更新当前实例的状态
        self.is_online = true;
        self.tcp_writer = Some(writer);

        let tgw_hex = format!("7467775F6C375F666F72776172640D0A486F73743A207A6F6E653{}2E3137726F636F2E71712E636F6D3A3434330D0A0D0A", server_id);
        self.send_package(tgw_hex.as_str()).await?;

        // 【关键】将当前实例的所有权移动进 Arc，这样它才能被多个任务共享
        let client_ptr = Arc::new(Mutex::new(self));
        Ok((client_ptr, reader))
    }

    pub async fn start_receive_task(
            client_handle: Arc<Mutex<Self>>, 
            mut reader: tokio::net::tcp::OwnedReadHalf
        ) {
        // 启动后台异步任务
        tokio::spawn(async move {
            let mut buffer = Vec::new();
            let mut temp_buf = [0u8; 8192];
            println!("开始接收数据...");
            loop {
                match reader.read(&mut temp_buf).await {
                    Ok(0) => {
                        println!("连接被服务端主动关闭。通常是 TGW 包格式错或登录超时。");
                        let mut c = client_handle.lock().await;
                        c.is_online = false;
                        break;
                    }
                    Ok(n) => {
                        buffer.extend_from_slice(&temp_buf[..n]);

                        // 第一层：确保至少有 20 字节（ADF 协议头长度）
                        while buffer.len() >= 20 {
                            
                            // 1. 验证魔数 (95 27)
                            if buffer[0] != 0x95 || buffer[1] != 0x27 {
                                // 如果不是魔数开头，说明流乱了，删掉第一个字节继续找
                                buffer.remove(0);
                                continue;
                            }

                            // 2. 只有走到这里，访问 buffer[18..20] 才是安全的
                            let body_len = u16::from_be_bytes([buffer[18], buffer[19]]) as usize;
                            let total_packet_len = 20 + body_len;

                            // 第二层：确保当前 buffer 已经包含了一整个包（头+身体）
                            if buffer.len() >= total_packet_len {
                                // 此时访问 buffer[4..8] 也是绝对安全的
                                let cmd_id = u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
                                
                                // 提取 Body
                                let body = buffer[20..total_packet_len].to_vec();

                                // 处理包逻辑
                                let mut c = client_handle.lock().await;
                                c.handle_packet(body).await;

                                // 移除已处理完毕的包
                                buffer.drain(0..total_packet_len);
                            } else {
                                // 包头够了，但身体还没传完，跳出循环等下一次 Read
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        println!("网络底层错误: {}", e);
                        break
                    },
                }
            }
        });
    }

    async fn handle_packet(&mut self, full_data: Vec<u8>) {
        if let Ok(text) = std::str::from_utf8(&full_data) {
            if text.contains("ok") {
                println!("TGW 握手成功 (收到 ok)，准备发送 Access...");
                // 握手成功，立即发送 Access 包（登录）
                self.access(100).await.ok();
                return;
            }
        }
        // 2. 如果不是 "ok"，再按照 20 字节的 ADF 协议解析
        if full_data.len() < 20 { 
            println!("收到短包或非标准包: {:?}", full_data);
            return; 
        }


        // 2. 提取 CmdID (ADF 协议标准偏移是 4..8)
        // 之前你写的 8,9,10 那个位置在洛克王国协议里通常是 UIN 的一部分
        let cmd_u32 = u32::from_be_bytes([full_data[4], full_data[5], full_data[6], full_data[7]]);
        let cmd_id = format!("{:08X}", cmd_u32);

        // 3. 提取包体 (Data)
        // 必须确保 full_data 至少有 20 字节才能截取 [20..]
        let body = if full_data.len() >= 20 {
            &full_data[20..]
        } else {
            &[] // 如果是空包或短包，body 为空
        };

        println!("收到协议号: 0x{} | 包体长度: {}", cmd_id, body.len());
        match cmd_id.as_str() {
            "0000030001"=> {
                todo!("处理 人物 包")
            }
            _ => {
                println!("未处理协议号: 0x{}", cmd_id);
            }
        }

        println!("收到协议号: 0x{}", cmd_id);
    }

    // 这是一个帮助函数，用来生成带 ADF 头的 Hex
    pub fn build_adf_hex(cmd_id: u32, uin: u32, body_hex: &str) -> String {
        let body = hex::decode(body_hex).unwrap_or_default();
        let mut head = vec![0u8; 20];
        
        head[0..2].copy_from_slice(&0x9527u16.to_be_bytes()); // Magic
        head[4..8].copy_from_slice(&cmd_id.to_be_bytes());    // CmdID
        head[8..12].copy_from_slice(&uin.to_be_bytes());      // Uin
        head[18..20].copy_from_slice(&(body.len() as u16).to_be_bytes()); // Length
        
        format!("{}{}", hex::encode(head), body_hex)
    }

    pub async fn send_package(&mut self, hex_package: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("[发送指令] {}", hex_package);
        let writer = self.tcp_writer.as_mut().ok_or("Not connected")?;
        let mut bytes = hex::decode(hex_package.replace(" ", ""))?;

        // 如果是 ADF 业务包 (Magic: 0x9527)
        if bytes.len() >= 20 && bytes[0] == 0x95 && bytes[1] == 0x27 {
            // 1. 获取并转换 UIN
            let login_info = self.login_info.lock().await;
            // 尝试解析字符串为 u32，如果解析失败则用 0
            let uin_num: u32 = login_info.angel_uin.parse().unwrap_or(0);
            
            // 2. 修正 UIN 字段 (偏移 8-11)
            bytes[8..12].copy_from_slice(&uin_num.to_be_bytes());
            
            // 3. 修正长度字段 (偏移 18-19)
            let body_len = (bytes.len() - 20) as u16;
            bytes[18..20].copy_from_slice(&body_len.to_be_bytes());
            
            println!("[发送指令] UIN: {}, 长度: {}", uin_num, body_len);
        }

        writer.write_all(&bytes).await?;
        writer.flush().await?;
        Ok(())
    }
    pub fn start_heartbeat(client_arc: Arc<Mutex<Self>>) {
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            // 设置 30 秒间隔
            let mut interval = time::interval(Duration::from_secs(30));
            println!("心跳任务已启动...");

            loop {
                // 等待下一个周期
                interval.tick().await;

                // 1. 获取锁并检查在线状态
                let res = {
                    let mut c = client_arc.lock().await;
                    if !c.is_online {
                        println!("检测到离线，停止心跳。");
                        break; // 退出循环，任务结束
                    }

                    // 2. 构造心跳包
                    // 逻辑：9527 (Magic) + 0000 (Ver) + 00030033 (CmdID) + UIN + 0000 (Serial) + 0000 (Check) + 0000 (Len)
                    // 注意：心跳包的 Body 长度通常是 0
                    let uin = c.uin.clone();
                    c.send_package(&format!("9527000000030033{}0000000000000000", uin)).await
                };

                if let Err(e) = res {
                    println!("心跳发送失败: {}，停止任务。", e);
                    break;
                }
            }
        });
    }
    pub async fn access(&mut self, channel_id: u16) -> Result<(), Box<dyn std::error::Error>> {
        // 1. 准备 hex_id
        let hex_id = &self.uin; 

        // 2. 准备频道 ID 
        let hex_channel_id = format!("{:04X}", channel_id);

        // 3. 处理 angel_key

        let hex_key_string: String = self.angel_key
            .chars()
            .map(|c| format!("{:02X}", c as u8))
            .collect();
        // 4. 拼接完整包
        // 结构：Magic(4) + CmdID(8) + UIN(8) + Serial(8) + Checksum(4) + Length(4) + Body
        // 9527000000030001 679D27A7 0000000000000042 006636304434413846333845424635463730414144463734314244374146364135413134433933303938444631354445354343464345383936463745303643383643
        // 9527000000030001 679D27A7 0000000000000042 0064624DF1D823463BB7A0FF65F4D9D10AB84EE96A6CB25E4B0E9F5973A709B48091
        println!("hexid:{}      channel:{}   key:{}", hex_id, hex_channel_id, hex_key_string);
        let packet = format!(
            "9527000000030001{}0000000000000042{}{}",
            hex_id,
            hex_channel_id,
            hex_key_string
        );

        println!("[发送 进入游戏包] {}", packet);
        self.send_package(&packet).await?;
        
        Ok(())
    }
}

