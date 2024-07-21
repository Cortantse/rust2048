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
use std::{io, thread, time::Duration};

use crate::game_board::{Position, TileMovement};

const TILE_WIDTH: u16 = 6;  // 方块的宽度
const TILE_HEIGHT: u16 = 3;  // 方块的高度

/// 执行瓷砖的动画移动
pub fn animate_double_move<B: Backend>(
    terminal: &mut Terminal<B>,
    movements1: Vec<TileMovement>,
    movements2: Vec<TileMovement>,
    board1: &Vec<Vec<u32>>,
    board2: &Vec<Vec<u32>>,
    pipe_data: Vec<u32>,
) -> Result<(), std::io::Error> {
    let size = terminal.size()?;

    let tile_width: u16 = TILE_WIDTH;
    let tile_height: u16 = TILE_HEIGHT;
    let gap: u16 = 1;

    let pipe_tiles_count = 5;
    let pipe_width = tile_width * pipe_tiles_count;
    let board_width = tile_width * 5;
    let total_width = board_width * 2 + pipe_width;

    let start_x = if size.width > total_width { (size.width - total_width) / 2 } else { 0 };

    let board1_area = Rect::new(start_x, size.y + 2, board_width, tile_height * 4);
    let pipe_area = Rect::new(start_x + board_width, size.y + 2 + tile_height * 2, pipe_width, tile_height);
    let board2_area = Rect::new(start_x + board_width + pipe_width, size.y + 2, board_width, tile_height * 4);

    let num_steps = 12;

    for step in 1..=num_steps {
        terminal.draw(|frame| {
            // 清除整个屏幕
            frame.render_widget(Block::default().borders(Borders::NONE).style(Style::default().bg(Color::Black)), frame.size());

            // 绘制背景的所有方块，值为0
            for i in 0..4 {
                for j in 0..4 {
                    let x1 = board1_area.x + j as u16 * (tile_width + gap + 1);
                    let y1 = board1_area.y + i as u16 * (tile_height + gap);
                    draw_tile(frame, 0, x1, y1, tile_width, tile_height);

                    let x2 = board2_area.x + j as u16 * (tile_width + gap + 1);
                    let y2 = board2_area.y + i as u16 * (tile_height + gap);
                    draw_tile(frame, 0, x2, y2, tile_width, tile_height);
                }
            }

            // 绘制每个移动的瓷砖动画 - 第一个棋盘
            for movement in &movements1 {
                let TileMovement { start_pos, end_pos, value } = movement;
                let (start_x_abs, start_y_abs) = calculate_absolute_position(*start_pos, tile_width, tile_height, gap, board1_area.x, board1_area.y);
                let (end_x_abs, end_y_abs) = calculate_absolute_position(*end_pos, tile_width, tile_height, gap, board1_area.x, board1_area.y);
                let x = interpolate(start_x_abs, end_x_abs, step, num_steps);
                let y = interpolate(start_y_abs, end_y_abs, step, num_steps);
                draw_tile(frame, *value, x, y, tile_width, tile_height);
            }

            // 绘制每个移动的瓷砖动画 - 第二个棋盘
            for movement in &movements2 {
                let TileMovement { start_pos, end_pos, value } = movement;
                let (start_x_abs, start_y_abs) = calculate_absolute_position(*start_pos, tile_width, tile_height, gap, board2_area.x, board2_area.y);
                let (end_x_abs, end_y_abs) = calculate_absolute_position(*end_pos, tile_width, tile_height, gap, board2_area.x, board2_area.y);
                let x = interpolate(start_x_abs, end_x_abs, step, num_steps);
                let y = interpolate(start_y_abs, end_y_abs, step, num_steps);
                draw_tile(frame, *value, x, y, tile_width, tile_height);
            }

            // 绘制管道
            draw_pipe(frame, pipe_area, pipe_data.clone());
        })?;

        // 等待一段时间，形成动画效果
        thread::sleep(Duration::from_millis(50));
    }

    // 动画结束后再次绘制静态的棋盘状态
    terminal.draw(|frame| {
        draw_double_board(frame, board1, board2, pipe_data.clone());
    })?;

    Ok(())  // 确保返回一个 Result
}

/// 绘制单个瓷砖
fn draw_tile<B: Backend>(frame: &mut Frame<B>, tile_value: u32, x: u16, y: u16, tile_width: u16, tile_height: u16) {
    let tile_rect = Rect::new(x, y, tile_width, tile_height);
    let bg_color = get_bg_color(tile_value);
    let fg_color = if tile_value > 4 { Color::White } else { Color::Black };
    let number = if tile_value > 0 { format_number(tile_value) } else { String::new() };
    let content = format!("\n\n{}\n\n\n", number);
    let para = Paragraph::new(content)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE).style(Style::default().bg(bg_color)))
        .style(Style::default().fg(fg_color).bg(bg_color).add_modifier(Modifier::BOLD));

    frame.render_widget(para, tile_rect);
}

/// 计算瓷砖在屏幕上的绝对位置
fn calculate_absolute_position(pos: Position, tile_width: u16, tile_height: u16, gap: u16, start_x: u16, start_y: u16) -> (u16, u16) {
    let x = start_x + pos.x as u16 * (tile_width + gap + 1);  // 注意这里的加1，以对齐静态绘制
    let y = start_y + pos.y as u16 * (tile_height + gap);
    (x, y)
}

/// 线性插值计算当前步骤的瓷砖位置
fn interpolate(start: u16, end: u16, step: usize, num_steps: usize) -> u16 {
    if start <= end {
        start + ((end - start) as usize * step / num_steps) as u16
    } else {
        start - ((start - end) as usize * step / num_steps) as u16
    }
}

pub fn draw_double_board<B: Backend>(frame: &mut Frame<B>, board1: &Vec<Vec<u32>>, board2: &Vec<Vec<u32>>, pipe_data: Vec<u32>) {
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
            let fg_color = Color::Black ;

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

pub fn draw_pipe<B: Backend>(frame: &mut Frame<B>, area: Rect, data: Vec<u32>) {
    let pipe_color = Color::Rgb(255, 0, 127);  // 管道颜色

    // 总是绘制5个格子
    for i in 0..5 {
        let x = area.x + i as u16 * TILE_WIDTH;  // 计算每个格子的横坐标
        let y = area.y;  // 维持在第三行位置
        let tile_rect = Rect::new(x, y + 2, TILE_WIDTH, TILE_HEIGHT);  // 定义格子的位置和尺寸

        // 格式化数字，确保垂直居中，加两行换行符作为上下填充
        let content = if i < data.len() {
            format!("\n{}\n", format_number(data[i] ))  // 在数字前后添加换行符以实现垂直居中
        } else {
            String::from("\n \n")  // 没有数据时显示空行，保持格子不显示乱码或旧数据
        };

        let para = Paragraph::new(content)
            .alignment(Alignment::Center)  // 水平居中
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
