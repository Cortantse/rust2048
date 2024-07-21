use bincode;
use protocol::{GameState, Message, PlayerIdentity};
use tokio::time::sleep;
use std::ops::ControlFlow;
use std::sync::{Arc, MutexGuard};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex, Semaphore};
use tokio::select;

mod bridge;
mod config;
mod game_board;
mod io_manager;
mod protocol;

use game_board::Direction;
use protocol::{deserialize_message, serialize_message, prevent_sticky_message};

pub use crate::bridge::Bridge;
pub use crate::game_board::GameBoard;
pub use crate::io_manager::IOManager;

// 服务器函数采用 1+1+k体系
// 1 为主任务循环，负责快速匹配，此处流量最大，只负责匹配，匹配后迅速通过管道发送到第一层异步任务
// 1 为第一层异步任务，负责生成新的异步任务给 每匹配的两个客户端
// k 个异步任务，负责双方的通信

// 初始化双方状态：
async fn initiate_two_clients_status(
    socket1: &mut TcpStream,
    socket2: &mut TcpStream,
    game_board1: Arc<Mutex<GameBoard>>,
    game_board2: Arc<Mutex<GameBoard>>,
    bridge: Arc<Mutex<Bridge>>,
) {
    println!("into initiate_two_clients_status");
    // 创建两个新的游戏板，每个游戏板对应一个客户端

    // 初始化游戏板，随机放置一个瓷砖
    {
        let mut gb = game_board1.lock().await;
        gb.spawn_tile();
        println!("Game board after spawning tile: {:?}", gb.get_tiles());
    }
    {
        let mut ob = game_board2.lock().await;
        ob.spawn_tile();
        println!("Other board after spawning tile: {:?}", ob.get_tiles());
    }

    println!("now trying to send identity message to client");

    // 在发送棋盘数据前，先告诉玩家身份
    let mut player_identity = PlayerIdentity { player_number: 1 };
    // 这里unrap异常，给了err
    let mut serialized_identity = serialize_message(&Message::PlayerIdentity(player_identity)).unwrap();
    serialized_identity = prevent_sticky_message(serialized_identity);

    // 将第一个等待者作为玩家1
    socket1
        .write_all(serialized_identity.as_bytes())
        .await
        .unwrap();

    player_identity = PlayerIdentity { player_number: 2 };
    let mut serialized_identity = serialize_message(&Message::PlayerIdentity(player_identity)).unwrap();
    serialized_identity = prevent_sticky_message(serialized_identity);
    println!("{:?}", serialized_identity);
    socket2
        .write_all(serialized_identity.as_bytes())
        .await
        .unwrap();

    println!("successfully sending identity message to clients");
    println!("now trying to send initial board status message to clients");


    send_board_status(game_board1, game_board2, socket1, socket2, None).await;
    println!()
}


// 发送棋盘当前状态给双方
async fn send_board_status(game_board1: Arc<Mutex<GameBoard>>, game_board2: Arc<Mutex<GameBoard>>, socket1: &mut TcpStream, socket2: &mut TcpStream, animated_vector: Option<Vec<u32>>) {
    println!("Into sending board_status");
    // 序列化双方棋盘状态，传递给客户端，使用定制协议

    // 尝试获取锁，小心一点
    let mut game_board1_unlocked = game_board1.lock().await;
    let mut game_board2_unlocked = game_board2.lock().await;
    let game_state = GameState {
        board1: game_board1_unlocked.get_tiles().to_vec(),
        board2: game_board2_unlocked.get_tiles().to_vec(),
        board1_reach_2048: game_board1_unlocked.check_game_over(),
        board2_reach_2048: game_board2_unlocked.check_game_over(),
        animated_vector: animated_vector,
        action1: Direction::None,
        action2: Direction::None,
    };

    println!("already get the lock");
    // 协议初始化
    let message = Message::GameState(game_state);
    let mut serialized_message = serialize_message(&message).unwrap();
    serialized_message = prevent_sticky_message(serialized_message);

    // 将游戏板状态发送给两个客户端
    socket1
        .write_all(serialized_message.as_bytes())
        .await
        .unwrap();
    socket2
        .write_all(serialized_message.as_bytes())
        .await
        .unwrap();

    println!("successfully sent message about board status");
}


async fn send_board_status_safe(game_board1: Arc<Mutex<GameBoard>>, game_board2: Arc<Mutex<GameBoard>>, mut socket1: Arc<Mutex<TcpStream>>, mut socket2: Arc<Mutex<TcpStream>>, animated_vector: Option<Vec<u32>>, action: Direction, if_player2: bool) {
    println!("Into sending board_status");
    // 序列化双方棋盘状态，传递给客户端，使用定制协议

    let mut socket1 = socket1.lock().await;
    let mut socket2 = socket2.lock().await;

    // 尝试获取锁，小心一点
    let mut game_board1_unlocked = game_board1.lock().await;
    let mut game_board2_unlocked = game_board2.lock().await;
    let game_state = GameState {
        board1: game_board1_unlocked.get_tiles().to_vec(),
        board2: game_board2_unlocked.get_tiles().to_vec(),
        board1_reach_2048: game_board1_unlocked.check_game_over(),
        board2_reach_2048: game_board2_unlocked.check_game_over(),
        animated_vector: animated_vector,
        action1: if !if_player2 { action } else { Direction::None },
        action2: if if_player2 { action } else { Direction::None },
    };

    println!("already get the lock");
    // 协议初始化
    let message = Message::GameState(game_state);
    let mut serialized_message = serialize_message(&message).unwrap();
    serialized_message = prevent_sticky_message(serialized_message);



    // 将游戏板状态发送给两个客户端
    socket1
        .write_all(serialized_message.as_bytes())
        .await
        .unwrap();
    socket2
        .write_all(serialized_message.as_bytes())
        .await
        .unwrap();

    println!("successfully sent message about board status");
}

// k 个异步任务的定义：
async fn deal_with_two_clients(
    mut socket1: TcpStream,
    mut socket2: TcpStream,
    game_board1: Arc<Mutex<GameBoard>>,
    game_board2: Arc<Mutex<GameBoard>>,
    bridge: Arc<Mutex<Bridge>>,
) {
    let socket1 = Arc::new(Mutex::new(socket1));
    let socket2 = Arc::new(Mutex::new(socket2));

    loop {

        let mut buffer1 = vec![0; 1024];
        let mut buffer2 = vec![0; 1024];
        let mut n = 1;
        let mut m = 1;

        select! {
            result1 = async {
                let mut lock = socket1.lock().await;
                match lock.read(&mut buffer1).await {
                    Ok(k) => n = k,
                    Err(_) => n = 0, // 客户端1断开连接，返回0
                }
            } => {
                execute_action_and_update_to_clients(socket1.clone(), socket2.clone(), game_board1.clone(), game_board2.clone(), bridge.clone(), false, buffer1, n).await;
            },
            result2 = async {
                let mut lock = socket2.lock().await;
                match lock.read(&mut buffer2).await {
                    Ok(k) => m = k,
                    Err(_) => m = 0, // 客户端2断开连接，返回0
                }
            } => {
                execute_action_and_update_to_clients(socket2.clone(), socket1.clone(), game_board2.clone(), game_board1.clone(), bridge.clone(), true, buffer2, m).await;
            }
        }
    }
}

//用于处理客户端发送action的函数
async fn execute_action_and_update_to_clients(socket1: Arc<Mutex<TcpStream>>, socket2:  Arc<Mutex<TcpStream>>, game_board1: Arc<Mutex<GameBoard>>, game_board2: Arc<Mutex<GameBoard>>, bridge: Arc<Mutex<Bridge>>, if_player2: bool, buffer1: Vec<u8>, N: usize) -> ControlFlow<()> {
    // !!! 这里没有锁，后面send_board_status_safe才锁了，不知道会不会有竞争问题！！！！！！！
    match N {
        0 => {
            // 如果客户端1断开连接，尝试优雅关闭客户端2的连接
            let mut socket2 = socket2.lock().await;
            let _ = socket2.shutdown().await;
            return ControlFlow::Break(()); // 退出循环，结束任务
        }
        n => {
            // 尝试从接收的数据解析出玩家操作
            match deserialize_message(&String::from_utf8_lossy(&buffer1[..n])) {
                Ok(Message::PlayerAction(action)) => {
                    // 成功解析出玩家动作，处理游戏逻辑
                    let mut animated_vector = None;
                    {
                        //小心翼翼获取锁
                        let mut gb = game_board1.lock().await; // 锁定并获取第一个游戏板
                        let mut ob = game_board2.lock().await; // 锁定并获取第二个游戏板
                        let mut br = bridge.lock().await; // 锁定并获取桥接器对象
                        animated_vector = br.send_through_bridge(&mut ob, &mut gb, action.direction, if_player2); // 使用桥接器传递动作
                        gb.move_tiles(action.direction); // 移动第一个游戏板上的瓷砖
                        gb.spawn_tile(); // 在第一个游戏板上生成新的瓷砖
                        gb.print_state_with(&ob, animated_vector.clone()); // 打印当前游戏状态

                    }

                    // 更新双方情况，接受一个参数来判断是谁
                    if !if_player2 {
                        send_board_status_safe(game_board1.clone(), game_board2.clone(), socket1, socket2, animated_vector, action.direction, if_player2).await;
                    }
                    else {
                        // 反转，保持gameboard1还是player1，方便客户端区分
                        send_board_status_safe(game_board2.clone(), game_board1.clone(), socket2, socket1, animated_vector, action.direction, if_player2).await;
                    }
                }
                Ok(_) => eprintln!("Unexpected message type"), // 接收到非预期类型的消息
                Err(e) => {println!("{:?}",buffer1);eprintln!("Failed to parse client 1 input: {}", e)}, // 解析消息失败
            }
        }
    }
    ControlFlow::Continue(())
}


// 主函数
#[tokio::main]
async fn main() {
    // 创建一个信号量来保证并发数不超过 SERVER_CAPACITY
    let semaphore = Arc::new(Semaphore::new(config::SERVER_CAPACITY));

    let listener = TcpListener::bind("0.0.0.0:".to_owned() + config::SERVER_PORT)
        .await
        .unwrap();
    let mess = "Server is running on ".to_owned() + config::SERVER_IP + ":" + config::SERVER_PORT;
    println!("{}", mess);

    // 创造一个管道，用于传送匹配好的两个人
    let (tx, mut rx) = mpsc::channel::<(TcpStream, TcpStream)>(100);

    // 创造异步任务，专门用于处理一对客户端的通信
    tokio::spawn(async move {
        while let Some((mut socket1, mut socket2)) = rx.recv().await {
            // 从管道处获得了一组匹配的客户端，现在对它们进行初始化，然后交给异步任务处理

            // 创建线程安全的gameboard和bridge
            let game_board = Arc::new(Mutex::new(GameBoard::new()));
            let other_board = Arc::new(Mutex::new(GameBoard::new()));
            // 生成桥梁，此处后面的逻辑要改，因为桥梁参数应该是服务器动态随机的过程，但是为了简便，暂时桥梁固定
            let bridge: Arc<Mutex<Bridge>> = Arc::new(Mutex::new(Bridge::new(
                false,
                Direction::Right,
                true,
                2,
                2,
                999999,
            )));

            // sleep(Duration::from_secs(10)).await; // 控制循环速度

            // 初始化双方状态
            initiate_two_clients_status(
                &mut socket1,
                &mut socket2,
                game_board.clone(),
                other_board.clone(),
                bridge.clone(),
            ).await;

            println!("finished initiate_two_clients_status");
            // sleep(Duration::from_secs(10)).await; // 控制循环速度
            

            // 创建一个新的任务用于双方客户端的通信，本地任务继续等待主循环匹配并传递新任务
            tokio::spawn(async move {
                deal_with_two_clients(socket1, socket2, game_board, other_board, bridge).await;
            });
        }
    });

    // 主循环用于侦听和捕获连接
    let mut pending_socket: Option<TcpStream> = None;
    // 这里的匹配逻辑好像有bug！！！！！！

    loop {
        println!("Waiting for connections...");
        // 接收新的TCP连接
        let (mut socket, _) = listener.accept().await.unwrap();
        // 获取信号量的许可，用于控制同时处理的连接数，没有时会等待
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        // 检查是否有待处理的socket
        if let Some(mut pending) = pending_socket.take() {
            // 如果有待处理的socket，说明现在有两个连接可以配对

            // 将这对socket以及对应的游戏板发送到处理任务
            // 主循环快速处理，主循环完成匹配后将多余内容留给异步线程
            
            tx.send((pending, socket)).await.unwrap();
        } else {
            // 如果没有待处理的socket，将当前socket保存为待处理
            pending_socket = Some(socket);
        }

        // 释放信号量的许可，允许其他连接继续
        drop(permit);
    }
}
