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


// 定义方块的宽度和高度，可以根据终端的显示效果调整这些值
const TILE_WIDTH: u16 = 8;  // 方块的宽度
const TILE_HEIGHT: u16 = 4;  // 方块的高度，通常小于宽度以适应终端字符的实际比例

/// 绘制两个棋盘和一个连接它们的管道
/// * `frame`: tui库的Frame引用，用于在终端中绘制界面
/// * `board1`: 第一个棋盘的数据，4x4的二维向量
/// * `board2`: 第二个棋盘的数据，4x4的二维向量
pub fn draw_double_board<B: Backend>(frame: &mut Frame<B>, board1: &Vec<Vec<u32>>, board2: &Vec<Vec<u32>>) {
    let size = frame.size();
    let block = Block::default().title("Double 2048 Game").borders(Borders::ALL);
    frame.render_widget(block, size);

    let pipe_width = 20;  // 管道的宽度，增加或减少此值可以改变两个棋盘之间的间距

    // 计算整体布局宽度，以确保内容居中
    let board_width = TILE_WIDTH * 4;  // 每个棋盘的总宽度
    let total_width = board_width * 2 + pipe_width;  // 包含两个棋盘和一个管道的总宽度

    // 计算起始x坐标，以居中显示所有内容
    let start_x = if size.width > total_width { (size.width - total_width) / 2 } else { 0 };

    // 定义两个棋盘和管道的位置和尺寸
    let board1_area = Rect::new(start_x, size.y + 2, board_width, TILE_HEIGHT * 4);
    let board2_area = Rect::new(start_x + board_width + pipe_width, size.y + 2, board_width, TILE_HEIGHT * 4);
    let pipe_area = Rect::new(start_x + board_width, size.y + 2 + TILE_HEIGHT * 2, pipe_width, TILE_HEIGHT);  // 管道位置

    draw_board(frame, board1_area, board1);
    draw_board(frame, board2_area, board2);

    // 绘制管道
    let pipe_block = Block::default().borders(Borders::ALL).title("Pipe");
    frame.render_widget(pipe_block, pipe_area);
}

/// 绘制单个棋盘
/// * `area`: 棋盘应该绘制的区域
/// * `board`: 棋盘的数据
pub fn draw_board<B: Backend>(frame: &mut Frame<B>, area: Rect, board: &Vec<Vec<u32>>) {
    for (i, row) in board.iter().enumerate() {
        for (j, &num) in row.iter().enumerate() {
            // 计算每个方块的位置
            let tile_x = area.x + j as u16 * TILE_WIDTH;
            let tile_y = area.y + i as u16 * TILE_HEIGHT;
            let tile_rect = Rect::new(tile_x, tile_y, TILE_WIDTH, TILE_HEIGHT);
            let number = format!("{:^width$}", num, width = TILE_WIDTH as usize);
            let para = Paragraph::new(number)
                .style(Style::default().fg(Color::Black).bg(Color::White))
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(para, tile_rect);
        }
    }
}


fn tem() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 测试用的两个4x4数组
    let board1 = vec![
        vec![2, 4, 8, 16],
        vec![32, 64, 128, 256],
        vec![512, 1024, 2048, 48],
        vec![32, 8, 16, 32],
    ];
    let board2 = vec![
        vec![2, 4, 8, 16],
        vec![32, 64, 128, 256],
        vec![512, 1024, 2048, 48],
        vec![32, 8, 16, 32],
    ];

    terminal.draw(|f| {
        draw_double_board(f, &board1, &board2);
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