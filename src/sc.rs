use crossterm::{
    execute,
    event::{read, Event, KeyCode},
    terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, ClearType},
    ExecutableCommand,
};
use tui::{
    backend::{Backend, CrosstermBackend}, layout::{Constraint, Direction as Dire, Layout, Rect}, style::{Color, Style}, text::{Span, Spans}, widgets::{Block, Borders, Paragraph}, Frame, Terminal
};
use std::io;
use rand::Rng;
use game::{Grid, Move};

mod game;

mod game_board;
mod game_controller;
mod io_manager;
mod bridge;

// mod gui;

use game_board::Direction;

pub use crate::game_board::GameBoard;
pub use crate::game_controller::GameController;
pub use crate::io_manager::IOManager;
pub use crate::bridge::Bridge;
// pub use crate::gui::GUI;

use std::thread;
use std::time::Duration;

/// 绘制4x4棋盘的函数
fn draw_board<B: tui::backend::Backend>(frame: &mut Frame<B>, board: &Vec<Vec<u32>>) {
    let size = frame.size();
    let block = Block::default().title("2048").borders(Borders::ALL);
    frame.render_widget(block, size);

    let layout = Layout::default()
        .direction(Dire::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    for (i, row) in board.iter().enumerate() {
        for (j, &num) in row.iter().enumerate() {
            let tile_x = layout[0].x + j as u16 * 16;
            let tile_y = layout[0].y + i as u16 * 4;
            let tile_rect = tui::layout::Rect::new(tile_x, tile_y, 10, 5);
            let number = format!("{:^15}", num);
            let para = Paragraph::new(number)
                .style(Style::default().fg(Color::Black).bg(Color::White))
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(para, tile_rect);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 期盼逻辑
    // 允许 10ms 后续这种参数放config
    let mut io_manager = IOManager::new(10);
    let mut game_board = GameBoard::new();
    game_board.spawn_tile();



    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    // 等待用户按任意键退出
    loop {
        if let Some((action)) = io_manager.read_input(1) {
            match (action) {
                (Direction::None) => continue,
                _ => {
                    // 非None 才管
                    // io_manager.clear_screen();
                    game_board.move_tiles(action);
                    game_board.spawn_tile();


                    

                    if game_board.check_game_over() == true {
                        println!("Game Over!");
                        if game_board.return_if_win() {
                            println!("You Win!");
                        }
                        else {
                            print!("You lose!");
                        }
                        panic!();
                    }
                },
            }
        }
        terminal.draw(|f| {
            draw_board(f, game_board.get_tiles());
        })?;
    }

    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    Ok(())
}