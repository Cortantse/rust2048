use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, EnterAlternateScreen},
    ExecutableCommand,
};
use std::io;
use std::process;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tui::{
    backend::CrosstermBackend,
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use game_board::Direction;

mod bridge;
mod config;
mod dc;
mod draw;
mod game_board;
mod io_manager;
mod protocol;
use dc::{animate_double_move, draw_double_board, animate_pipe};

use protocol::{
    receive_message, receive_message_with_buffer, serialize_message, Message, PlayerAction,
};

pub use crate::bridge::Bridge;
pub use crate::game_board::GameBoard;
pub use crate::io_manager::IOManager;



async fn show_loading_screen(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    running: &mut bool,
    mut rx_main: mpsc::Receiver<String>,
    tx_main: mpsc::Sender<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut frame_count = 0;
    let spinner_frames = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let spinner_length = spinner_frames.len();

    while *running {
        if let Ok(message) = rx_main.try_recv() {
            if message != "Connected successfully".to_string() {
                terminal.draw(|f| {
                    let size = f.size();
                    let block = Block::default().title("错误").borders(Borders::ALL);
                    let text = vec![Spans::from(Span::styled(
                        "匹配失败，將返回主界面",
                        Style::default().fg(Color::Red),
                    ))];
                    let paragraph = Paragraph::new(text)
                        .block(block)
                        .alignment(Alignment::Center);
                    f.render_widget(paragraph, size);
                })?;
                let _ = sleep(Duration::from_secs(3));
            }
            break;
        }

        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    *running = false;
                    let _ = tx_main.send("User requested exit".to_string()).await;
                    break;
                }
            }
        }

        let frame_index = frame_count % spinner_length;
        let spinner = spinner_frames[frame_index];
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("等待匹配中...")
                .borders(Borders::ALL);
            let text = vec![
                Spans::from(Span::styled(spinner, Style::default().fg(Color::Yellow))),
                Spans::from(Span::styled(
                    "按 'q' 退出",
                    Style::default().fg(Color::White),
                )),
            ];
            let paragraph = Paragraph::new(text)
                .block(block)
                .alignment(Alignment::Center);
            f.render_widget(paragraph, size);
        })?;
        frame_count += 1;
        sleep(Duration::from_millis(100)).await;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut stdout1 = io::stdout();
    stdout1.execute(EnterAlternateScreen)?;

    let backend1 = CrosstermBackend::new(stdout1);
    let mut terminal1 = Terminal::new(backend1)?;

    let mut io_manager = IOManager::new(10);
    let address = config::SERVER_IP.to_owned() + ":" + config::SERVER_PORT;
    let address: &str = address.as_str();

    let mut game_board = GameBoard::new();
    let mut other_board = GameBoard::new();

    let ref mut other_board_ref = other_board;

    let mut our_identity = 0;
    let mut a = 0;

    let mut running = true;

    let (tx_to_async, rx_from_main) = mpsc::channel(1);
    let (tx_to_main, mut rx_from_async) = mpsc::channel(1);

    let _ = tokio::spawn({
        async move {
            let _ =
                show_loading_screen(&mut terminal1, &mut running, rx_from_main, tx_to_main).await;
        }
    });

    for _ in 0..config::CLIENT_MAX_RETRIES {
        if let Ok(_) = rx_from_async.try_recv() {
            // println!("Received error message: {}", error_message);
            terminal.draw(|f| {
                let size = f.size();
                let block = Block::default().title("错误").borders(Borders::ALL);
                let text = vec![Spans::from(Span::styled(
                    "匹配失败，將返回主界面",
                    Style::default().fg(Color::Red),
                ))];
                let paragraph = Paragraph::new(text)
                    .block(block)
                    .alignment(Alignment::Center);
                f.render_widget(paragraph, size);
            })?;
            let _ = sleep(Duration::from_secs(3));
            break;
        }
        match TcpStream::connect(address).await {
            Ok(mut stream) => {
                match receive_message(&mut stream).await {
                    Ok(message) => {
                        match message {
                            Message::PlayerIdentity(identity) => {
                                our_identity = identity.player_number;
                            }
                            _ => {
                                println!("Received invalid message: {:?}, Should receive Player Identity", message);
                                panic!("Client Should receive Player Identity");
                            }
                        }
                    }
                    Err(e) => eprintln!("Failed to receive message: {}", e),
                }
                match receive_message(&mut stream).await {
                    Ok(message) => match message {
                        Message::GameState(game_state) => {
                            game_board.set_tiles(game_state.board1);
                            other_board_ref.set_tiles(game_state.board2);
                            let pipe_data_raw = game_state.animated_vector;
                            let pipe_data = match pipe_data_raw {
                                Some(data) => data,
                                None => {
                                    vec![]
                                }
                            };
                            let _ = tx_to_async.send("Connected successfully".to_string()).await;
                            terminal.clear()?;
                            terminal.draw(|f| {
                                draw_double_board(
                                    f,
                                    game_board.get_tiles(),
                                    other_board_ref.get_tiles(),
                                    pipe_data,
                                );
                            })?;
                        }
                        _ => {
                            println!(
                                "Received invalid message: {:?}, Should receive GameState",
                                message
                            );
                            panic!("Client Should receive GameState");
                        }
                    },
                    Err(e) => eprintln!("Failed to receive message: {}", e),
                }
                // 由于动画需要，需要存储上一次的棋盘状态
                let mut last_game_board_status = game_board.get_tiles().clone();
                let mut last_other_board_status = other_board_ref.get_tiles().clone();


                let mut buffer = Vec::new();
                loop {
                    select! {
                        input_result = io_manager.read_input_async(our_identity) => {
                            match input_result {
                                Some(action) => match action {
                                    Direction::None => {},
                                    Direction::Quit => {
                                        terminal.clear()?;
                                        process::exit(0); // 终止进程
                                    },
                                    _ => {
                                        let player_action = PlayerAction { direction: action };
                                        let message = Message::PlayerAction(player_action);
                                        let serialized = serialize_message(&message).unwrap();
                                        stream.write_all(serialized.as_bytes()).await.unwrap();
                                    }
                                },
                                None => continue,
                            }
                        },
                        message_result = receive_message_with_buffer(&mut stream, &mut buffer) => {
                            match message_result {
                                Ok(message) => match message {
                                    Message::GameState(game_state) => {
                                        game_board.set_tiles(game_state.board1);
                                        other_board_ref.set_tiles(game_state.board2);
                                        let pipe_data_raw = game_state.animated_vector;
                                        let pipe_data = match pipe_data_raw {
                                            Some(data) => data,
                                            None => {
                                                vec![]
                                            }
                                        };
                                        // terminal.draw(|f| {
                                        //     draw_double_board(f, game_board.get_tiles(), other_board_ref.get_tiles(), pipe_data);
                                        // })?;
                                        //测试
                                        let movements1 = game_board.get_tile_movements(last_game_board_status, game_board.get_tiles().clone(), game_state.action1, vec![]);
                                        let movements2 = game_board.get_tile_movements(last_other_board_status, other_board_ref.get_tiles().clone(), game_state.action2, vec![]);
                                        //获取有效动作
                                        let action = match (game_state.action1, game_state.action2) {
                                            (Direction::None, x) => x,
                                             (x, Direction::None) => x,
                                             _ => Direction::None,
                                        };
                                        // 在绘制函数内部调用动画函数
                                        animate_double_move(&mut terminal, movements1, movements2, game_board.get_tiles().clone().as_ref(), other_board_ref.get_tiles().clone().as_ref(), pipe_data, action);

                                        // 存储本次
                                        last_game_board_status = game_board.get_tiles().clone();
                                        last_other_board_status = other_board_ref.get_tiles().clone();
                                    },
                                    _ => {
                                        // println!("Received invalid message: {:?}, Should receive GameState", message);
                                        panic!("Client Should receive GameState");
                                    }
                                },
                                Err(_) => {
                                    terminal.draw(|f| {
                                        let size = f.size();
                                        let block = Block::default().title("返回").borders(Borders::ALL);
                                        let text = vec![Spans::from(Span::styled(
                                            "您的对手已离开，將返回主界面",
                                            Style::default().fg(Color::Red),
                                        ))];
                                        let paragraph = Paragraph::new(text)
                                            .block(block)
                                            .alignment(Alignment::Center);
                                        f.render_widget(paragraph, size);
                                    })?;
                                    let _ = sleep(Duration::from_secs(3));
                                    terminal.clear()?;
                                    process::exit(0); // 终止进程
                                },
                            }
                        },
                    }
                }
            }
            Err(e) => {
                a += 1;
                // println!("{}, Failed to connect: {:?}", a, e);
                if a == config::CLIENT_MAX_RETRIES {
                    let _ = tx_to_async
                        .send(format!("Failed to connect: {:?}", e))
                        .await;
                }

                sleep(Duration::from_secs(config::CLIENT_MAX_RETRIES_PER_REQUEST)).await;
            }
        }
    }
    // loading_task.await.unwrap(); // 确保加载动画任务正确结束

    Ok(())
}
