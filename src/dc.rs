use crossterm::{
    execute,
    event::{read, Event, KeyCode},
    terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, ClearType},
    ExecutableCommand,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction as Dire, Layout, Rect, Alignment},
    style::{Color, Style, Modifier},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;

const TILE_WIDTH: u16 = 6;  // 方块的宽度
const TILE_HEIGHT: u16 = 3;  // 方块的高度

pub fn draw_double_board<B: Backend>(frame: &mut Frame<B>, board1: &Vec<Vec<u32>>, board2: &Vec<Vec<u32>>, pipe_data: &[u8]) {
    let size = frame.size();
    let block = Block::default().title("Double 2048 Game").borders(Borders::ALL);
    frame.render_widget(block, size);

    let pipe_tiles_count = 5;  // 管道由五个格子组成
    let pipe_width = TILE_WIDTH * pipe_tiles_count;  // 管道的宽度为五个格子宽

    let board_width = TILE_WIDTH * 5;  // 每个棋盘的总宽度
    let total_width = board_width * 2 + pipe_width;  // 总宽度包括两个棋盘和一个管道

    let start_x = if size.width > total_width { (size.width - total_width) / 2 } else { 0 };

    // 定义两个棋盘和管道的位置和尺寸
    let board1_area = Rect::new(start_x, size.y + 2, board_width, TILE_HEIGHT * 4);
    let pipe_area = Rect::new(start_x + board_width, size.y + 2 + TILE_HEIGHT * 2, pipe_width, TILE_HEIGHT);  // 管道在第三行
    let board2_area = Rect::new(start_x + board_width + pipe_width, size.y + 2, board_width, TILE_HEIGHT * 4);

    draw_board(frame, board1_area, board1);
    draw_pipe(frame, pipe_area, pipe_data);  // 现在传递数据
    draw_board(frame, board2_area, board2);
}


pub fn draw_board<B: Backend>(frame: &mut Frame<B>, area: Rect, board: &Vec<Vec<u32>>) {
    let tile_width = TILE_WIDTH;
    let tile_height = TILE_HEIGHT;
    let gap: u16 = 1;  // 调整间隙尺寸以达到视觉平衡

    for (i, row) in board.iter().enumerate() {
        for (j, &num) in row.iter().enumerate() {
            let x = area.x + j as u16 * (tile_width + gap + 1);
            let y = area.y + i as u16 * (tile_height + gap);
            let tile_rect = Rect::new(x, y, tile_width, tile_height);

            let bg_color = get_bg_color(num);
            let fg_color = if num > 4 { Color::White } else { Color::Black };

            let number = if num > 0 { format_number(num) } else { String::new() };
            let content = format!("\n{}\n", number);
            let para = Paragraph::new(content)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::NONE).style(Style::default().bg(bg_color)))
                .style(Style::default().fg(fg_color).bg(bg_color).add_modifier(Modifier::BOLD));

            frame.render_widget(para, tile_rect);
        }
    }
}

pub fn draw_pipe<B: Backend>(frame: &mut Frame<B>, area: Rect, data: &[u8]) {
    let pipe_color = Color::Rgb(255, 0, 127);  // 管道颜色

    // 总是绘制5个格子
    for i in 0..5 {
        let x = area.x + i as u16 * TILE_WIDTH;  // 计算每个格子的横坐标
        let y = area.y;  // 维持在第三行位置
        let tile_rect = Rect::new(x, y + 2, TILE_WIDTH, TILE_HEIGHT);  // 定义格子的位置和尺寸

        let content = if i < data.len() {
            format_number(data[i] as u32)  // 格式化存在的数据
        } else {
            String::from(" ")  // 数据不存在则显示空格
        };

        let para = Paragraph::new(content)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::NONE).style(Style::default().bg(pipe_color)))
            .style(Style::default().fg(Color::White));  // 设置文字和背景颜色

        frame.render_widget(para, tile_rect);  // 将段落渲染到对应的矩形区域
    }
}


fn format_number(num: u32) -> String {
    num.to_string().chars()
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
