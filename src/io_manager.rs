use crate::Direction;
use crossterm::{
    event::{self, poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

// use crossterm::event::{self, poll, read, Event, KeyCode, KeyEventKind};
// use std::time::{Duration, Instant};

pub struct IOManager {
    last_input_time: Instant,
    io_response_interval: Duration,
}

impl IOManager {
    pub fn new(response_interval_ms: u64) -> Self {
        Self {
            last_input_time: Instant::now(),
            io_response_interval: Duration::from_millis(response_interval_ms),
        }
    }

    // pub fn read_input(&mut self) -> (Option<(Direction, i32)>) {
    //     // Check if the current time since the last input is less than the set interval
    //     if self.last_input_time.elapsed() < self.io_response_interval {
    //         return None;
    //     }

    //     // Check for pending events with the specified timeout
    //     if poll(self.io_response_interval).unwrap_or(false) {
    //         if let Ok(Event::Key(key_event)) = read() {
    //             // Only process key down events
    //             if key_event.kind == KeyEventKind::Press {
    //                 self.update_last_input_time(); // Update the last input time
    //                 return Some(match key_event.code {
    //                     KeyCode::Up => {
    //                         println!("left board chossing ↑");
    //                         (Direction::Up, 0)
    //                     }
    //                     KeyCode::Left => {
    //                         println!("left board choosing ←");
    //                         (Direction::Left, 0)
    //                     }
    //                     KeyCode::Down => {
    //                         println!("left board choosing ↓");
    //                         (Direction::Down, 0)
    //                     }
    //                     KeyCode::Right => {
    //                         println!("left board choosing →");
    //                         (Direction::Right, 0)
    //                     }
    //                     KeyCode::Char('w') => {
    //                         println!("right board choosing ↑");
    //                         (Direction::Up, 1)
    //                     }
    //                     KeyCode::Char('a') => {
    //                         println!("right board choosing ←");
    //                         (Direction::Left, 1)
    //                     }
    //                     KeyCode::Char('s') => {
    //                         println!("right board choosing ↓");
    //                         (Direction::Down, 1)
    //                     }
    //                     KeyCode::Char('d') => {
    //                         println!("right board choosing →");
    //                         (Direction::Right, 1)
    //                     }
    //                     _ => (Direction::None, -1),
    //                 });
    //             }
    //         }
    //     }

    //     None
    // }
    

    pub fn read_input(&mut self, character: u8) -> Option<Direction> {
        // Check if the current time since the last input is less than the set interval
        if self.last_input_time.elapsed() < self.io_response_interval {
            return None;
        }
        // Check for pending events with the specified timeout
        if poll(self.io_response_interval).unwrap_or(false) {
            if let Ok(Event::Key(key_event)) = read() {
                // println!("Key event: {:?}", key_event); // 打印整个键事件
                // Only process key down events
                if key_event.kind == KeyEventKind::Press {
                    self.update_last_input_time(); // Update the last input time
                    return Some(match key_event.code {
                        KeyCode::Up => {
                            print_info(character);
                            println!("↑");
                            Direction::Up
                        }
                        KeyCode::Left => {
                            print_info(character);
                            println!("←");
                            Direction::Left
                        }
                        KeyCode::Down => {
                            print_info(character);
                            println!("↓");
                            Direction::Down
                        }
                        KeyCode::Right => {
                            print_info(character);
                            println!("→");
                            Direction::Right
                        }
                        KeyCode::Char('w') => {
                            print_info(character);
                            println!("↑");
                            Direction::Up
                        }
                        KeyCode::Char('a') => {
                            print_info(character);
                            println!("←");
                            Direction::Left
                        }
                        KeyCode::Char('s') => {
                            print_info(character);
                            println!("↓");
                            Direction::Down
                        }
                        KeyCode::Char('d') => {
                            print_info(character);
                            println!("→");
                            Direction::Right
                        }
                        _ => Direction::None,
                    });
                }
            }
        }
        None
    }

    pub fn write_output(&self, message: &str) {
        println!("{}", message);
    }

    pub fn clear_screen(&self) {
        execute!(io::stdout(), Clear(ClearType::All)).expect("Failed to clear screen");
    }

    pub fn update_last_input_time(&mut self) {
        self.last_input_time = Instant::now();
    }
}

pub fn print_info(character: u8){
    // if character == 1 {
    //     print!("left board choosing ");
    // }else if character == 2  {
    //     print!("right board choosing ");
    // }else {
    //     panic!("character must be 1 or 2");
    // }
    print!("We choose ");
}