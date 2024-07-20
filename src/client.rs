use bincode;
use crossterm::terminal::SetTitle;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};
use tokio::select;

mod bridge;
mod game_board;
mod game_controller;
mod io_manager;
mod config;
mod protocol;

use game_board::Direction;
use protocol::{receive_message, serialize_message, Message, PlayerAction, receive_message_with_buffer};

pub use crate::bridge::Bridge;
pub use crate::game_board::GameBoard;
pub use crate::game_controller::GameController;
pub use crate::io_manager::IOManager;

#[tokio::main]
async fn main() {
    let mut io_manager = IOManager::new(10);
    // 目标服务器地址
    let address = config::SERVER_IP.to_owned() + ":" + config::SERVER_PORT;
    let address:&str = address.as_str();

    // 本地创建双方棋盘实例，这里默认就是双人模式
    let mut game_board = GameBoard::new();
    let mut other_board = GameBoard::new();
    let ref mut other_board_ref = other_board;

    // 自身在服务器等级的identity号
    let mut our_identity = 0;

    // 尝试连接服务器，尝试k次
    for _ in 0..config::CLIENT_MAX_RETRIES {


        // 尝试连接
        match TcpStream::connect(address).await {
            Ok(mut stream) => {
                println!("Connected to the server at {}", address);
                // 成功连接服务器
                // 获取game board状态，以及是几号玩家


                // 1、先获取自己是几号玩家
                match receive_message(&mut stream).await {
                    Ok(message) => {
                        match message {
                            Message::PlayerIdentity(identity) => {
                                println!("Received identity: Player {}", identity.player_number);
                                our_identity = identity.player_number;
                            },
                            _ => {
                                println!("Received invalid message: {:?}, Should receive Player Identity", message);
                                panic!("Client Should receive Player Identity");
                            }
                        }
                    }
                    Err(e) => eprintln!("Failed to receive message: {}", e),
                }
                
                // 2、获取双方的初始矩阵
                match receive_message(&mut stream).await {
                    Ok(message) => {
                        match message {
                            Message::GameState(game_state) => {
                                // 这里要根据不同的角色
                                game_board.set_tiles(game_state.board1);
                                other_board_ref.set_tiles(game_state.board2);
                                // 展示棋盘
                                // 暂时未none，animated_vector实际不为none！！！！！！！
                                game_board.print_state_with(&other_board_ref, game_state.animated_vector);
                            },
                            _ => {
                                println!("Received invalid message: {:?}, Should receive GameState", message);
                                panic!("Client Should receive GameState");
                            }
                        }
                    }
                    Err(e) => eprintln!("Failed to receive message: {}", e),
                }
    

                // 开始接收并处理来自服务器的更新，主循环逻辑

                // 先初始化缓冲区，注意缓冲区内可能包括之前未读完/不完整的数据
                let mut buffer: Vec<u8> = vec![0; 4096];
                loop {
                    // 接收用户输入操作
                    select! {
                        input_result = io_manager.read_input_async(our_identity) => {
                            match input_result {
                                Some(action) => match action {
                                    Direction::None => {
                                        println!("No direction input detected.");
                                    },
                                    _ => {
                                        // 序列化方向并发送到服务器
                                        let player_action = PlayerAction { direction: action };
                                        let message = Message::PlayerAction(player_action);
                                        let serialized = serialize_message(&message).unwrap();
                                        stream.write_all(serialized.as_bytes()).await.unwrap();
                                    }
                                },
                                None => {
                                    continue;
                                },
                            }
                        },
                        message_result = receive_message_with_buffer(&mut stream, &mut buffer) => {
                            match message_result {
                                Ok(message) => {
                                    match message {
                                        Message::GameState(game_state) => {
                                            game_board.set_tiles(game_state.board1);
                                            other_board_ref.set_tiles(game_state.board2);
                                            // 展示，注意暂时没有animated vector！！！！！！
                                            game_board.print_state_with(other_board_ref, game_state.animated_vector);
                                        },
                                        _ => {
                                            println!("Received invalid message: {:?}, Should receive GameState", message);
                                            panic!("Client Should receive GameState");
                                        }
                                    }
                                },
                                Err(e) => {
                                    eprintln!("Failed to receive message: {}", e);
                                },
                            }
                        },
                    }
                }
                // end of loop
            }
            Err(e) => {
                println!("Failed to connect: {:?}", e);
                // 失败，暂停一会
                sleep(Duration::from_secs(config::CLIENT_MAX_RETRIES_PER_REQUEST)).await;
            }
        }
    }
}
