use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::process::Command;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

fn draw_ui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    menu_items: &[ListItem],
    list_state: &mut ListState,
    instructions: &[Spans],
) -> Result<(), io::Error> {
    terminal
        .draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(size);

            let block = Block::default()
                .title("2048 游戏菜单")
                .borders(Borders::ALL);
            let list = List::new(menu_items.to_vec())
                .block(block)
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD | Modifier::ITALIC)
                        .fg(Color::Yellow),
                )
                .highlight_symbol(">> ")
                .style(Style::default().fg(Color::White));

            f.render_stateful_widget(list, chunks[0], list_state);

            let paragraph = Paragraph::new(instructions.to_vec())
                .block(Block::default().title("操作说明").borders(Borders::ALL))
                .alignment(tui::layout::Alignment::Center);
            f.render_widget(paragraph, chunks[1]);
        })
        .map(|_| ())
}

pub fn run_ui() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let menu_items = vec![
        ListItem::new("单人游戏").style(Style::default().add_modifier(Modifier::BOLD)),
        ListItem::new("双人游戏").style(Style::default().add_modifier(Modifier::BOLD)),
        ListItem::new("退出").style(Style::default().add_modifier(Modifier::BOLD)),
    ];
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    let instructions = vec![
        Spans::from(Span::styled(
            "WASD - 导航菜单",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Spans::from(Span::styled(
            "Enter - 选择",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Spans::from(Span::styled(
            "Q - 退出程序",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
    ];
    if let Event::Key(_) = event::read()? {
        // 忽略该事件
    }

    let mut game_running = true;
    while game_running {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(size);

            let block = Block::default()
                .title("2048 游戏菜单")
                .borders(Borders::ALL);
            let list = List::new(menu_items.clone())
                .block(block)
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD | Modifier::ITALIC)
                        .fg(tui::style::Color::Yellow),
                )
                .highlight_symbol(">> ")
                .style(Style::default().fg(tui::style::Color::White));

            f.render_stateful_widget(list, chunks[0], &mut list_state);

            let paragraph = Paragraph::new(instructions.clone())
                .block(Block::default().title("操作说明").borders(Borders::ALL))
                .alignment(tui::layout::Alignment::Center); // 设置文字水平居中
            f.render_widget(paragraph, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    game_running = false;
                }
                KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up => {
                    if let Some(selected) = list_state.selected() {
                        let previous = if selected == 0 {
                            menu_items.len() - 1
                        } else {
                            selected - 1
                        };
                        list_state.select(Some(previous));
                    }
                    if let Event::Key(_) = event::read()? {
                        // 忽略该事件
                    }
                }
                KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => {
                    if let Some(selected) = list_state.selected() {
                        let next = if selected >= menu_items.len() - 1 {
                            0
                        } else {
                            selected + 1
                        };
                        list_state.select(Some(next));
                    }
                    if let Event::Key(_) = event::read()? {
                        // 忽略该事件
                    }
                }
                KeyCode::Enter => {
                    if let Some(selected) = list_state.selected() {
                        match selected {
                            0 => {
                                // println!("启动单人游戏...");
                                Command::new("cargo")
                                    .args(&["run", "--bin", "sc"])
                                    .spawn()?
                                    .wait()?;
                                terminal.clear()?;
                                draw_ui(
                                    &mut terminal,
                                    &menu_items,
                                    &mut list_state,
                                    &instructions,
                                )?;
                            }
                            1 => {
                                // println!("启动双人游戏...");
                                Command::new("cargo")
                                    .args(&["run", "--bin", "client"])
                                    .spawn()?
                                    .wait()?;
                                terminal.clear()?;
                                draw_ui(
                                    &mut terminal,
                                    &menu_items,
                                    &mut list_state,
                                    &instructions,
                                )?;
                            }
                            2 => {
                                // println!("操作: 退出");
                                // sleep(Duration::from_secs(3));
                                game_running = false;
                            }
                            _ => {}
                        }
                    }
                    if let Event::Key(_) = event::read()? {
                        // 忽略该事件
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn main() {
    if let Err(e) = run_ui() {
        eprintln!("Error running UI: {}", e);
    }
}
