use serde::{Serialize, Deserialize};
use game_board::Direction;
use tokio::io::{self, AsyncReadExt};
use tokio::net::{TcpStream};
use std::str;

use crate::game_board;

// 定义游戏棋盘的状态
#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub board1: Vec<Vec<u32>>,
    pub board2: Vec<Vec<u32>>,
    pub board1_reach_2048: bool,
    pub board2_reach_2048: bool,
    pub animated_vector: Option<Vec<u32>>,
}

// 玩家的操作，将内部Direction封装
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerAction {
    pub direction: Direction,
}

// 消息枚举，用于区分不同类型的消息
#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    GameState(GameState),
    PlayerAction(PlayerAction),
    PlayerIdentity(PlayerIdentity),
}

// 序列化消息
pub fn serialize_message(message: &Message) -> Result<String, serde_json::Error> {
    serde_json::to_string(message)
}

// 反序列化消息
pub fn deserialize_message(json_data: &str) -> Result<Message, serde_json::Error> {
    serde_json::from_str(json_data)
}

// 接受协议报函数，无重复buffer设计
pub async fn receive_message(stream: &mut TcpStream) -> Result<Message, io::Error> {
    let mut buffer = Vec::new();  // 使用动态数组来处理可能的多个数据块
    let mut temp_buf = [0; 1024];  // 临时缓冲区

    // 持续从流中读取数据，直到找到消息终止符
    loop {
        let n = stream.read(&mut temp_buf).await?;
        if n == 0 {
            if buffer.is_empty() {
                eprintln!("No data read; stream might be closed.");
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Connection was closed by the server"));
            } else {
                break;  // 如果已经读到了数据但没有更多数据了，则尝试解析它
            }
        }
        buffer.extend_from_slice(&temp_buf[..n]);

        if buffer.ends_with(b"\n") {  // 检查是否达到消息的结尾
            break;  // 完成消息读取
        }
    }

    // 从buffer中移除终止符，并尝试解析消息
    let message_str = str::from_utf8(&buffer[..buffer.len() - 1])  // 去除末尾的'\n'
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    match deserialize_message(message_str) {
        Ok(message) => {
            eprintln!("Successfully deserialized message.");
            Ok(message)
        },
        Err(e) => {
            println!("{:?}", temp_buf);
            eprintln!("Failed to deserialize message: {}", e);
            Err(io::Error::new(io::ErrorKind::InvalidData, format!("Failed to deserialize message: {}", e)))
        }
    }
}

pub fn prevent_sticky_message(ori: String) -> String {
    ori + "\n"
}


// 接受协议报函数，有大量数据的设计
/// 接收消息并处理缓冲区，返回第一个完整的消息
pub async fn receive_message_with_buffer(stream: &mut TcpStream, buffer: &mut Vec<u8>) -> Result<Message, io::Error> {
    // 首先检查缓冲区是否已包含至少一个完整的消息
    if let Some(pos) = buffer.iter().position(|&x| x == b'\n') {
        let message = process_message(&buffer[..pos])?;  // 处理第一个完整消息，不包括 '\n'
        buffer.drain(..=pos);  // 清空包括 '\n' 在内的已处理部分
        return Ok(message);
    }

    // 从流中读取数据，直到找到一个完整的消息
    let mut temp_buf = [0; 1024];  // 临时缓冲区

    let n = stream.read(&mut temp_buf).await?;
    if n == 0 {
        // 如果没有更多数据可读，检查缓冲区是否空，如果是则报错
        println!("Connection closed");
        if buffer.is_empty() {
            eprintln!("No data read; stream might be closed.");
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Connection was closed by the server"));
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Data does not end with a newline and no more data to read"));
        }
    }
    buffer.extend_from_slice(&temp_buf[..n]);  // 将新读取的数据追加到缓冲区后面

    // 再次检查是否存在完整的消息
    if let Some(pos) = buffer.iter().position(|&x| x == b'\n') {
        let message = process_message(&buffer[..pos])?;  // 处理第一个完整消息，不包括 '\n'
        buffer.drain(..=pos);  // 清空包括 '\n' 在内的已处理部分
        return Ok(message);
    }else {
        println!("Unable to give any message");
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Data does not end with a newline"));
    }
    
}

/// 解析消息并返回
fn process_message(data: &[u8]) -> Result<Message, io::Error> {
    let message_str = str::from_utf8(data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    eprintln!("Received string for deserialization: '{}'", message_str);
    serde_json::from_str(message_str).map_err(|e| {
        eprintln!("Failed to deserialize message: {}", e);
        io::Error::new(io::ErrorKind::InvalidData, format!("Failed to deserialize message: {}", e))
    })
}


// 写入，序列化方法
// let game_state = GameState {
//     board1,
//     board2,
//     board1_reach_2048: bool,
//     board2_reach_2048: bool,
// };
// let message = Message::GameState(game_state);
// let serialized = serialize_message(&message).unwrap();
// socket.write_all(serialized.as_bytes()).await.unwrap();



// 接受，反序列化
// let received_json = String::from_utf8(buffer[..n].to_vec()).unwrap();

//     解析接收到的消息
//     match deserialize_message(&received_json) {
//         Ok(Message::PlayerAction(action)) => {
//             println!("Received player action: {:?}", action.direction);
//             // 处理玩家操作...
//         },
//         Ok(_) => println!("Unexpected message type"),
//         Err(e) => println!("Failed to deserialize message: {:?}", e),
//     }

// 告诉玩家是player1还是2
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerIdentity {
    pub player_number: u8,  // 用数字1或2表示玩家1或玩家2
}