use rand::{random, Rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::None => Direction::None,
        }
    }
}

pub struct GameBoard {
    tiles: Vec<Vec<u32>>,        // 用二维向量表示棋盘
    history: Vec<Vec<Vec<u32>>>, // 存储历史棋盘状态
    check_should_be_used_after_spawn: bool,
    reach_2048: bool,
}

impl GameBoard {
    pub fn new() -> Self {
        Self {
            tiles: vec![vec![0; 4]; 4], // 默认为4x4的棋盘
            history: Vec::new(),        // 初始化空的历史记录
            check_should_be_used_after_spawn: false,
            reach_2048: false,
        }
    }

    pub fn spawn_tile(&mut self) {
        self.check_should_be_used_after_spawn = true;
        // 先检查是否还有空位
        if !self.if_have_empty_tile() {
            return;
        }

        // 在棋盘上随机位置生成新的数字块
        // 10%概率生成4 90%概率生成2
        let mut rng = rand::thread_rng();
        let mut num = rng.gen_range(0..9); // 生成一个0到9之间的随机整数
        match num {
            0 => num = 4,
            _ => num = 2,
        }
        let mut empty_space = vec![];
        for i in 0..4 {
            for j in 0..4 {
                if self.tiles[i][j] == 0 {
                    empty_space.push((i, j));
                }
            }
        }
        let random_choice = rng.gen_range(0..empty_space.len());
        let (i, j) = empty_space[random_choice];
        self.tiles[i][j] = num;

        //警告，这个函数最后写，因为先要测试移动功能，而移动功能答案是固定的，而spawn_tile会产生波动！！！！！！！！！！！！！！！！！！！！！！！！！！
        //只有spawn_tile后才能运行check_game_over
        self.check_should_be_used_after_spawn = true;
    }

    pub fn move_tiles(&mut self, direction: Direction) {
        // 根据用户输入的方向移动和合并数字块
        self.save_current_state();
        let last_score = self.return_score();
        match direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::None => panic!("Should not go to move_tiles function with None direction"),
        }
        // 内置检查，理论上移动前后不会有数据差别
        let score = self.return_score();
        if last_score.0 != score.0 {
            print!("Score {:?} is not equal to {:?}", last_score, score);
            panic!();
        }
        // 放置新tile
        // self.spawn_tile();

        // 检查游戏是否结束
        // if self.check_game_over() {
        //     println!("Game over!");
        //     std::process::exit(0);
        // }
    }

    pub fn return_if_win(&self) -> bool {
        self.reach_2048
    }

    pub fn get_tiles(&self) -> &Vec<Vec<u32>> {
        &self.tiles
    }

    pub fn get_tiles_mut(&mut self) -> &mut Vec<Vec<u32>> {
        &mut self.tiles
    }

    pub fn check_game_over(&mut self) -> bool {
        // 检查游戏是否结束，这个函数应该只在spawn后被使用
        // 机制很简单，首先分为成功结束和失败结束：
        // 1、先检查是否由空区域，若有则还没结束 2、若没有，移动四个方向看看为不为空，若还没有也结束 —— 失败结束
        // 1、若最大值2048及以上，则结束，并且设置reach_2048为真 —— 成功结束
        if !self.check_should_be_used_after_spawn {
            print!("funcction check_game_over should be used after spawning");
        }
        self.check_should_be_used_after_spawn = false;
        // 获取最大值
        let (_, max) = self.return_score();
        if max >= 2048 {
            self.reach_2048 = true;
            return true;
        }
        // 检查四个方向是否有空位置
        // 若有，则还没结束
        // 若没有，则结束

        if !self.if_have_empty_tile() {
            // 没有位置了

            self.save_current_state(); // 便于还原
            self.move_down();
            if self.if_have_empty_tile() {
                self.undo_move();
                return false;
            }
            self.undo_move();

            self.save_current_state(); // 便于还原
            self.move_left();
            if self.if_have_empty_tile() {
                self.undo_move();
                return false;
            }
            self.undo_move();

            self.save_current_state();
            self.move_up();
            if self.if_have_empty_tile() {
                self.undo_move();
                return false;
            }
            self.undo_move();

            self.save_current_state();
            self.move_right();
            if self.if_have_empty_tile() {
                self.undo_move();
                return false;
            }
            self.undo_move();

            return true;
        }

        false
    }

    pub fn reset_board(&mut self) {
        // 重置棋盘到初始状态
        self.tiles = vec![vec![0; 4]; 4];
        self.history = Vec::new(); // 清空历史记录
    }

    pub fn return_score(&self) -> (u32, u32) {
        // 返回 总分数和最大分数
        let mut max = 0;
        let mut score = 0;
        for i in 0..4 {
            for j in 0..4 {
                score += self.tiles[i][j];
                if self.tiles[i][j] > max {
                    max = self.tiles[i][j];
                }
            }
        }
        (score, max)
    }
    // 添加一个新的函数用于保存当前棋盘到历史记录
    pub fn save_current_state(&mut self) {
        // 将当前棋盘状态复制并添加到历史记录中
        // vec是堆空间，由vec管理，内存安全
        let current_board = self.tiles.clone();
        self.history.push(current_board);
    }

    // 添加一个新的函数用于撤销上一步操作
    pub fn undo_move(&mut self) {
        // 恢复到上一步的棋盘状态
        if self.history.len() > 0 {
            // 默认考虑直接pop出，方便多次还原
            self.tiles = self.history.pop().unwrap();
        } else {
            // no tiles
            println!("no tiles");
        }
    }

    // pub fn print_state(&mut self) {
    //     // 打印棋盘，方便做调试
    //     println!("===================="); //换个行
    //     for i in 0..4 {
    //         for j in 0..4 {
    //             print!("{} ", self.tiles[i][j]);
    //         }
    //         println!("");
    //     }
    //     println!("===================="); //换个行
    // }

    pub fn set_tiles(&mut self, tiles: Vec<Vec<u32>>) {
        self.tiles = tiles;
    }

    pub fn print_state(&self) {
        println!("===================="); // 换个行
        for row in &self.tiles {
            for &tile in row {
                print!("{} ", tile);
            }
            println!("");
        }
        println!("===================="); // 换个行
    }

    pub fn print_state_with(&mut self, other: &GameBoard, animated_vector: Option<Vec<u32>>) {
        // 打印棋盘，方便做调试
        println!("===================="); //换个行
        for i in 0..4 {
            for j in 0..4 {
                print!("{} ", self.tiles[i][j]);
            }
            let j = 0;
            if i == 1 || i == 3 {
                print!(
                    "---------- {} {} {} {}",
                    other.tiles[i][j],
                    other.tiles[i][j + 1],
                    other.tiles[i][j + 2],
                    other.tiles[i][j + 3]
                );
            } else if i == 2 {
                if let Some(ref print_vector) = animated_vector {
                    let space_occupied = print_vector.len() * 2;
                    for item in print_vector {
                        print!("{} ", item);
                    }
                    for i in 0..10 - space_occupied {
                        print!(" ");
                    }
                    print!(
                        " {} {} {} {}",
                        other.tiles[i][j],
                        other.tiles[i][j + 1],
                        other.tiles[i][j + 2],
                        other.tiles[i][j + 3]
                    );
                } else {
                    print!(
                        "           {} {} {} {}",
                        other.tiles[i][j],
                        other.tiles[i][j + 1],
                        other.tiles[i][j + 2],
                        other.tiles[i][j + 3]
                    );
                }
            } else {
                print!(
                    "           {} {} {} {}",
                    other.tiles[i][j],
                    other.tiles[i][j + 1],
                    other.tiles[i][j + 2],
                    other.tiles[i][j + 3]
                );
            }
            println!("");
        }
        println!("===================="); //换个行
    }

    pub fn move_abstract(&mut self, mut line: Vec<u32>) -> Vec<u32> {
        // 返回一个向左的合并数组
        let mut new_line = vec![];
        let len_of_line = line.len();
        for i in 0..len_of_line {
            if line[i] != 0 {
                // 如果为0那么不管了
                // 进行合并
                let mut if_find = false;
                for j in i + 1..len_of_line {
                    if line[j] == line[i] {
                        // 相等，清空
                        new_line.push(line[i] * 2);
                        line[i] = 0;
                        line[j] = 0;
                        if_find = true;
                        break;
                    }
                    if line[j] != 0 {
                        // 匹配失败且中间格非空，则失败
                        break;
                    }
                }
                // 判断是否合并成功
                if !if_find {
                    new_line.push(line[i]);
                }
            }
        }
        // 填充0
        let fill_zero_size = len_of_line - new_line.len();
        for i in 0..fill_zero_size {
            new_line.push(0);
        }
        new_line
    }
    fn move_left(&mut self) {
        for i in 0..4 {
            self.tiles[i] = self.move_abstract(self.tiles[i].clone());
        }
    }
    fn move_right(&mut self) {
        for i in 0..4 {
            // 需要反向使用abstract
            let mut line = vec![];
            for j in (0..4).rev() {
                line.push(self.tiles[i][j]);
            }
            line = self.move_abstract(line);
            for j in (0..4).rev() {
                self.tiles[i][j] = line[4 - 1 - j];
            }
            // 2 2 2 0
            // 0 0 2 4
            // using abstract method
            // 0 2 2 2
            // 4 2 0 0
        }
    }
    fn move_up(&mut self) {
        for i in 0..4 {
            // 需要反向使用abstract
            let mut line = vec![];
            for j in (0..4) {
                line.push(self.tiles[j][i]);
            }
            line = self.move_abstract(line);
            for j in (0..4) {
                self.tiles[j][i] = line[j];
            }
        }
    }
    fn move_down(&mut self) {
        for i in (0..4) {
            // 需要反向使用abstract
            let mut line = vec![];
            for j in (0..4).rev() {
                line.push(self.tiles[j][i]);
            }
            line = self.move_abstract(line);
            for j in (0..4).rev() {
                self.tiles[j][i] = line[4 - 1 - j];
            }
        }
    }
    fn if_have_empty_tile(&mut self) -> bool {
        for i in (0..4) {
            for j in (0..4) {
                if self.tiles[i][j] == 0 {
                    return true;
                }
            }
        }
        false
    }
}

// 基础单元测试，移动测试请移步下方
#[cfg(test)]
mod tests_base {
    use super::*;
    #[test]
    fn test_reset_board() {
        let mut game = GameBoard::new();
        game.tiles[0][0] = 2;
        game.reset_board();
        assert!(
            game.tiles.iter().flatten().all(|&x| x == 0),
            "重置棋盘后，所有格子应为0"
        );
    }

    #[test]
    fn test_save_and_undo() {
        let mut game = GameBoard::new();
        game.tiles[0][0] = 2;
        game.save_current_state();
        game.tiles[0][0] = 4;
        game.undo_move();
        assert_eq!(game.tiles[0][0], 2, "撤销操作后，应恢复到上一状态");
    }

    #[test]
    fn test_return_score() {
        let mut game = GameBoard::new();
        game.tiles = vec![
            vec![2, 4, 8, 16],
            vec![32, 64, 128, 256],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let points = game.return_score();
        assert_eq!(points.0, 510, "计算分数失败");
    }

    #[test]
    fn test_check_game_over() {
        // 设计什么时候棋盘算失败
        // 棋盘满时就失败吗？应该不对
        // 个人感觉应该是棋盘满了，并且四个方向移动都无法生成空白空间才算失败
        let mut game = GameBoard::new();
        game.tiles = vec![
            vec![2, 4, 8, 16],
            vec![32, 64, 128, 256],
            vec![2, 4, 8, 16],
            vec![32, 64, 128, 256],
        ];
        game.spawn_tile();
        print!("{:?}", game.check_game_over());
    }

    #[test]
    fn test_move_abstract() {
        let line_ori = vec![2, 4, 4, 2];
        let line_new = vec![2, 8, 2, 0];
        let mut game = GameBoard::new();
        let new_line = game.move_abstract(line_ori);
        assert_eq!(
            new_line, line_new,
            "合并失败，实际{:?}, 期望{:?}",
            new_line, line_new
        );
    }
}

// 对移动功能的单元测试
// 注意，使用该单元测试时，请关闭 检测棋盘功能/生成tile函数 否则无法正常进行
#[cfg(test)]
mod tests_move {
    use super::*;

    #[test]
    fn test_move_tiles_complicated_right() {
        let mut game = GameBoard::new();
        game.tiles = vec![
            vec![2, 2, 4, 4],
            vec![4, 4, 2, 2],
            vec![0, 4, 4, 4],
            vec![2, 0, 0, 2],
        ];
        game.move_tiles(Direction::Right);
        let expected = vec![
            vec![0, 0, 4, 8],
            vec![0, 0, 8, 4],
            vec![0, 0, 4, 8],
            vec![0, 0, 0, 4],
        ];
        assert_eq!(
            game.tiles, expected,
            "向右合并失败: 实际 {:?}, 期望 {:?}",
            game.tiles, expected
        );
    }

    #[test]
    fn test_move_tiles_complicated_left() {
        let mut game = GameBoard::new();
        game.tiles = vec![
            vec![2, 2, 4, 4],
            vec![4, 4, 2, 2],
            vec![4, 4, 4, 0],
            vec![2, 0, 0, 2],
        ];
        game.move_tiles(Direction::Left);
        let expected = vec![
            vec![4, 8, 0, 0],
            vec![8, 4, 0, 0],
            vec![8, 4, 0, 0],
            vec![4, 0, 0, 0],
        ];
        assert_eq!(
            game.tiles, expected,
            "向左合并失败: 实际 {:?}, 期望 {:?}",
            game.tiles, expected
        );
    }

    #[test]
    fn test_move_tiles_complicated_up() {
        let mut game = GameBoard::new();
        game.tiles = vec![
            vec![2, 4, 0, 2],
            vec![2, 4, 0, 2],
            vec![4, 0, 4, 0],
            vec![4, 2, 4, 2],
        ];
        game.move_tiles(Direction::Up);
        let expected = vec![
            vec![4, 8, 8, 4],
            vec![8, 2, 0, 2],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(
            game.tiles, expected,
            "向上合并失败: 实际 {:?}, 期望 {:?}",
            game.tiles, expected
        );
    }

    #[test]
    fn test_move_tiles_complicated_down() {
        let mut game = GameBoard::new();
        game.tiles = vec![
            vec![2, 4, 0, 2],
            vec![2, 4, 0, 2],
            vec![4, 0, 4, 0],
            vec![4, 2, 4, 2],
        ];
        game.move_tiles(Direction::Down);
        let expected = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![4, 8, 0, 2],
            vec![8, 2, 8, 4],
        ];
        assert_eq!(
            game.tiles, expected,
            "向下合并失败: 实际 {:?}, 期望 {:?}",
            game.tiles, expected
        );
    }

    #[test]
    fn test_move_tiles_complicated_down_1() {
        let mut game = GameBoard::new();
        game.tiles = vec![
            vec![2, 4, 0, 2],
            vec![2, 4, 0, 0],
            vec![4, 0, 4, 2],
            vec![4, 2, 4, 2],
        ];
        game.move_tiles(Direction::Down);
        let expected = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![4, 8, 0, 2],
            vec![8, 2, 8, 4],
        ];
        assert_eq!(
            game.tiles, expected,
            "向下合并失败: 实际 {:?}, 期望 {:?}",
            game.tiles, expected
        );
    }

    #[test]
    fn test_move_tiles_really_complicated_right() {
        let mut game = GameBoard::new();
        game.tiles = vec![
            vec![2, 2, 2, 2],
            vec![2, 2, 2, 2],
            vec![2, 2, 2, 2],
            vec![2, 2, 2, 2],
        ];
        game.move_tiles(Direction::Right);
        let expected = vec![
            vec![0, 0, 4, 4],
            vec![0, 0, 4, 4],
            vec![0, 0, 4, 4],
            vec![0, 0, 4, 4],
        ];
        assert_eq!(
            game.tiles, expected,
            "向右合并失败: 实际 {:?}, 期望 {:?}",
            game.tiles, expected
        );
    }

    //write same testing functions in left, down, up directions
    #[test]
    fn test_move_tiles_really_complicated_left() {
        let mut game = GameBoard::new();
        game.tiles = vec![
            vec![2, 2, 2, 2],
            vec![2, 2, 2, 2],
            vec![2, 2, 2, 2],
            vec![2, 2, 2, 2],
        ];
        game.move_tiles(Direction::Left);
        let expected = vec![
            vec![4, 4, 0, 0],
            vec![4, 4, 0, 0],
            vec![4, 4, 0, 0],
            vec![4, 4, 0, 0],
        ];
        assert_eq!(
            game.tiles, expected,
            "向左合并失败: 实际 {:?}, 期望 {:?}",
            game.tiles, expected
        );
    }

    #[test]
    fn test_move_tiles_really_complicated_up() {
        let mut game = GameBoard::new();
        game.tiles = vec![
            vec![2, 2, 2, 2],
            vec![2, 2, 2, 2],
            vec![2, 2, 2, 2],
            vec![2, 2, 2, 2],
        ];
        game.move_tiles(Direction::Up);
        let expected = vec![
            vec![4, 4, 4, 4],
            vec![4, 4, 4, 4],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(
            game.tiles, expected,
            "向上合并失败: 实际 {:?}, 期望 {:?}",
            game.tiles, expected
        );
    }

    #[test]
    fn test_move_tiles_really_complicated_do_not_mix() {
        let mut game = GameBoard::new();
        game.tiles = vec![
            vec![2, 2, 2, 2],
            vec![4, 4, 4, 4],
            vec![2, 2, 2, 2],
            vec![4, 4, 4, 4],
        ];
        game.move_tiles(Direction::Up);
        game.move_tiles(Direction::Down);
        let expected = vec![
            vec![2, 2, 2, 2],
            vec![4, 4, 4, 4],
            vec![2, 2, 2, 2],
            vec![4, 4, 4, 4],
        ];
        assert_eq!(
            game.tiles, expected,
            "向上下合并失败: 实际 {:?}, 期望 {:?}",
            game.tiles, expected
        );
    }

    #[test]
    fn test_move_tiles_really_complicated_complete() {
        let mut game = GameBoard::new();
        game.tiles = vec![
            vec![2, 2, 2, 2],
            vec![4, 4, 4, 4],
            vec![2, 2, 2, 2],
            vec![4, 4, 4, 4],
        ];
        game.move_tiles(Direction::Up);
        game.move_tiles(Direction::Down);
        game.move_tiles(Direction::Left);
        game.move_tiles(Direction::Right);
        let expected = vec![
            vec![0, 0, 0, 8],
            vec![0, 0, 0, 16],
            vec![0, 0, 0, 8],
            vec![0, 0, 0, 16],
        ];
        assert_eq!(
            game.tiles, expected,
            "合并失败: 实际 {:?}, 期望 {:?}",
            game.tiles, expected
        );
    }
}
