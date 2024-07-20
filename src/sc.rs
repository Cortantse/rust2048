use crossterm::{
    execute,
    event::{read, Event, KeyCode},
    terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, ClearType},
    ExecutableCommand,
};
use tui::{
    backend::{Backend, CrosstermBackend}, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::{Span, Spans}, widgets::{Block, Borders, Paragraph}, Frame, Terminal
};
use std::io;
use rand::Rng;
use game::{Grid, Move};

mod game;

/// 绘制4x4棋盘的函数
fn draw_board<B: tui::backend::Backend>(frame: &mut Frame<B>, board: [[u32; 4]; 4]) {
    let size = frame.size();
    let block = Block::default().title("2048").borders(Borders::ALL);
    frame.render_widget(block, size);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
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
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let board = [
        [2, 4, 8, 16],
        [32, 64, 128, 256],
        [512, 1024, 2048, 2],
        [4, 64, 32, 256],
    ];

    terminal.draw(|f| {
        draw_board(f, board);
    })?;

    // 等待用户按任意键退出
    loop {
        if let Event::Key(_) = read()? {
            break;
        }
    }

    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    Ok(())
}