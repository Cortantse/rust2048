use crossterm::{
    terminal::{enable_raw_mode, EnterAlternateScreen},
    ExecutableCommand,
};

use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

mod bridge;
mod game;
mod game_board;
mod game_controller;
mod io_manager;

pub use crate::bridge::Bridge;
pub use crate::game_board::GameBoard;
pub use crate::game_controller::GameController;
pub use crate::io_manager::IOManager;
use game_board::Direction;

fn draw_board<B: Backend>(frame: &mut Frame<B>, board: &Vec<Vec<u32>>) {
    let size = frame.size();
    let block = Block::default().title("2048").borders(Borders::ALL);
    frame.render_widget(block, size);

    // 增加方块高度，减少宽度，调整间隙
    let tile_width: u16 = 13; // 方块的调整后的宽度
    let tile_height: u16 = 5; // 方块的调整后的高度
    let gap: u16 = 1; // 增加间隙尺寸以达到视觉平衡

    let start_x = (size
        .width
        .saturating_sub(tile_width * board[0].len() as u16 + (gap * (board[0].len() as u16 - 1))))
        / 2;
    let start_y = (size
        .height
        .saturating_sub(tile_height * board.len() as u16 + (gap * (board.len() as u16 - 1))))
        / 2;

    for (i, row) in board.iter().enumerate() {
        for (j, &num) in row.iter().enumerate() {
            let x = start_x + j as u16 * (tile_width + gap + 1);
            let y = start_y + i as u16 * (tile_height + gap);
            let tile_rect = Rect::new(x, y, tile_width, tile_height);

            let bg_color = get_bg_color(num);
            let fg_color = if num > 4 { Color::White } else { Color::Black };

            let number = if num > 0 {
                format_number(num)
            } else {
                String::new()
            };
            let content = format!("\n\n{}\n\n\n", number);
            let para = Paragraph::new(content)
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::NONE)
                        .style(Style::default().bg(bg_color)),
                )
                .style(
                    Style::default()
                        .fg(fg_color)
                        .bg(bg_color)
                        .add_modifier(Modifier::BOLD),
                );

            frame.render_widget(para, tile_rect);
        }
    }
}

/// 使用全角字符显示数字
fn format_number(num: u32) -> String {
    num.to_string()
        .chars()
        .map(|ch| match ch {
            '0' => '０',
            '1' => '１',
            '2' => '２',
            '3' => '３',
            '4' => '４',
            '5' => '５',
            '6' => '６',
            '7' => '７',
            '8' => '８',
            '9' => '９',
            _ => ch,
        })
        .collect()
}

/// 根据数字获取背景颜色
fn get_bg_color(n: u32) -> Color {
    match n {
        2 => Color::Rgb(239, 224, 200),
        4 => Color::Rgb(239, 200, 159),
        8 => Color::Rgb(242, 177, 121),
        16 => Color::Rgb(245, 149, 99),
        32 => Color::Rgb(246, 124, 95),
        64 => Color::Rgb(246, 94, 59),
        128 => Color::Rgb(237, 207, 114),
        256 => Color::Rgb(237, 204, 97),
        512 => Color::Rgb(237, 200, 80),
        1024 => Color::Rgb(237, 197, 63),
        2048 => Color::Rgb(237, 194, 46),
        _ => Color::Gray,
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
        if let Some(action) = io_manager.read_input(1) {
            match action {
                Direction::None => continue,
                Direction::Quit => {
                    // Command::new("cargo")
                    //     .args(&["run", "--bin", "menu"])
                    //     .spawn()?
                    //     .wait()?;
                    break;
                }
                _ => {
                    // 非None 才管
                    // io_manager.clear_screen();
                    game_board.move_tiles(action);
                    game_board.spawn_tile();

                    if game_board.check_game_over() == true {
                        println!("Game Over!");
                        if game_board.return_if_win() {
                            println!("You Win!");
                        } else {
                            print!("You lose!");
                        }
                        panic!();
                    }
                }
            }
        }
        terminal.draw(|f| {
            draw_board(f, game_board.get_tiles());
        })?;
    }

    // terminal.backend_mut().execute(LeaveAlternateScreen)?;
    Ok(())
}
