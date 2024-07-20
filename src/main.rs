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


    // 定期处理信息
    loop {
        if let Some((action)) = io_manager.read_input(1) {
            match (action) {
                (Direction::None) => continue,
                _ => {
                    // 非None 才管
                    // io_manager.clear_screen();
                    game_board.move_tiles(action);
                    game_board.spawn_tile();
                    game_board.print_state();
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
        // 处理后的是否要操作

    }
}