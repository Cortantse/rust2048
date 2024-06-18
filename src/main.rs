mod game_board;
mod game_controller;
mod io_manager;
// mod gui;

use game_board::Direction;

pub use crate::game_board::GameBoard;
pub use crate::game_controller::GameController;
pub use crate::io_manager::IOManager;
// pub use crate::gui::GUI;


fn main(){
    // 允许 10ms 后续这种参数放config
    let mut io_manager = IOManager::new(10);
    let mut game_board = GameBoard::new();
    game_board.spawn_tile();
    game_board.print_state();
    loop {
        if let Some(action) = io_manager.read_input() {
            match action {
                Direction::None => continue,
                _ => {
                    // 非None 才管
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
                        return;
                    }
                }
            }
        }
    }
}