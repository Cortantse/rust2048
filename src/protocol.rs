use serde::{Serialize, Deserialize};
use game_board::Direction;
use tokio::io::{self, AsyncReadExt};
use tokio::net::{TcpStream};

use crate::game_board;

// 定义游戏棋盘的状态
#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub board1: Vec<Vec<u32>>,
    pub board2: Vec<Vec<u32>>,
    pub board1_reach_2048: bool,
    pub board2_reach_2048: bool,
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

// 接受协议报函数
pub async fn receive_message(stream: &mut TcpStream) -> Result<Message, io::Error> {
    let mut buffer = vec![0; 1024];
    match stream.read(&mut buffer).await {
        Ok(0) => {
            eprintln!("No data read; stream might be closed.");
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Connection was closed by the server"))
        },
        Ok(n) => {
            eprintln!("Read {} bytes from the stream.", n);
            let message_str = String::from_utf8_lossy(&buffer[..n]);
            match deserialize_message(&message_str) {
                Ok(message) => {
                    eprintln!("Successfully deserialized message.");
                    Ok(message)
                },
                Err(e) => {
                    eprintln!("Failed to deserialize message: {}", e);
                    Err(io::Error::new(io::ErrorKind::InvalidData, format!("Failed to deserialize message: {}", e)))
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to read from socket: {}", e);
            Err(e)
        }
    }
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