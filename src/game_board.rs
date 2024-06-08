pub struct GameBoard {
    tiles: Vec<Vec<u32>>, // 用二维向量表示棋盘
    history: Vec<Vec<Vec<u32>>>, // 存储历史棋盘状态
}

impl GameBoard {
    pub fn new() -> Self {
        Self {
            tiles: vec![vec![0; 4]; 4], // 默认为4x4的棋盘
            history: Vec::new(), // 初始化空的历史记录
        }
    }

    pub fn spawn_tile(&mut self) {
        // 在棋盘上随机位置生成新的数字块

        //!!!警告，这个函数最后写，因为先要测试移动功能，而移动功能答案是固定的，而spawn_tile会产生波动！！！！！！！！！！！！！！！！！！！！！！！！！！
    }

    pub fn move_tiles(&mut self, direction: Direction) {
        // 根据用户输入的方向移动和合并数字块
    }

    pub fn check_game_over(&self) -> bool {
        // 检查游戏是否结束
        false
    }

    pub fn reset_board(&mut self) {
        // 重置棋盘到初始状态
    }

    pub fn return_score(&self) -> u32 {
        // 返回当前分数
        0
    }
    // 添加一个新的函数用于保存当前棋盘到历史记录
    pub fn save_current_state(&mut self) {
        // 将当前棋盘状态复制并添加到历史记录中
    }

    // 添加一个新的函数用于撤销上一步操作
    pub fn undo_move(&mut self) {
        // 恢复到上一步的棋盘状态
    }

    fn print_state(&mut self){
        // 打印棋盘，方便做调试
        for i in 0..4{
            for j in 0..4{
                print!("{} ",self.tiles[i][j]);
            }
            println!("");
        }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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
        assert!(game.tiles.iter().flatten().all(|&x| x == 0), "重置棋盘后，所有格子应为0");
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
        assert_eq!(points, 510, "计算分数失败");
    }


    #[test]
    fn test_check_game_over() {
        // 设计什么时候棋盘算失败
        // 棋盘满时就失败吗？
        // 还是满了并且四个方向移动都无法合并产生新的空间的时候算失败？
    }
}




// 对移动功能的单元测试
// 注意，使用该单元测试时，请关闭 检测棋盘功能/生成tile函数 否则无法正常进行
#[cfg(test)]
mod tests_move {
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
        assert_eq!(game.tiles, expected, "向右合并失败: 实际 {:?}, 期望 {:?}", game.tiles, expected);
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
        assert_eq!(game.tiles, expected, "向左合并失败: 实际 {:?}, 期望 {:?}", game.tiles, expected);
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
        assert_eq!(game.tiles, expected, "向上合并失败: 实际 {:?}, 期望 {:?}", game.tiles, expected);
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
            vec![4, 8, 0, 4],
            vec![8, 2, 8, 2],
        ];
        assert_eq!(game.tiles, expected, "向下合并失败: 实际 {:?}, 期望 {:?}", game.tiles, expected);
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
        assert_eq!(game.tiles, expected, "向下合并失败: 实际 {:?}, 期望 {:?}", game.tiles, expected);
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
        game.move_tiles(Direction::Down);
        let expected = vec![
            vec![0, 0, 4, 4],
            vec![0, 0, 4, 4],
            vec![0, 0, 4, 4],
            vec![0, 0, 4, 4],
        ];
        assert_eq!(game.tiles, expected, "向右合并失败: 实际 {:?}, 期望 {:?}", game.tiles, expected);
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
        assert_eq!(game.tiles, expected, "向左合并失败: 实际 {:?}, 期望 {:?}", game.tiles, expected);
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
        assert_eq!(game.tiles, expected, "向上合并失败: 实际 {:?}, 期望 {:?}", game.tiles, expected);
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
        assert_eq!(game.tiles, expected, "向上下合并失败: 实际 {:?}, 期望 {:?}", game.tiles, expected);
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
        assert_eq!(game.tiles, expected, "合并失败: 实际 {:?}, 期望 {:?}", game.tiles, expected);
    }


}