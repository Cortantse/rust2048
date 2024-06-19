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


fn main(){
    // 允许 10ms 后续这种参数放config
    let mut io_manager = IOManager::new(10);
    let mut game_board = GameBoard::new();
    let mut other = GameBoard::new();
    let ref mut other_board = other;
    game_board.spawn_tile();
    other_board.spawn_tile();
    game_board.print_state_with(other_board, None);

    // 做一个定时桥
    let mut bridge = Bridge::new(true, Direction::Left, 
        true, 2, 2, 999999);

    // 定期处理信息
    loop {
        if let Some((action, number)) = io_manager.read_input() {
            match (action, number) {
                (Direction::None, _) => continue,
                (_, 0) => {
                    // 非None 才管
                    // io_manager.clear_screen();
                    let animated_vector = bridge.send_through_bridge(other_board, &mut game_board, action);
                    game_board.move_tiles(action);
                    game_board.spawn_tile();
                    game_board.print_state_with(other_board, animated_vector);
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
                _ => {
                    // 非None 才管
                    // io_manager.clear_screen();
                    let animated_vector = bridge.send_through_bridge(&mut game_board, other_board, action);
                    other_board.move_tiles(action);
                    other_board.spawn_tile();
                    game_board.print_state_with(other_board, animated_vector);
                    if other_board.check_game_over() == true {
                        println!("Game Over!");
                        if other_board.return_if_win() {
                            println!("You Win!");
                        }
                        else {
                            print!("You lose!");
                        }
                        panic!();
                        return;
                    }
                }
            }
        }
        // 处理后的是否要操作

    }
}