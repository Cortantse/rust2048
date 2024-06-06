pub struct GameBoard {
    tiles: Vec<Vec<u32>>, // 用二维向量表示棋盘
}

impl GameBoard {
    pub fn new() -> Self {
        Self { tiles: vec![vec![0; 4]; 4] } // 默认为4x4的棋盘
    }

    pub fn spawn_tile(&mut self) {
        // 在棋盘上随机位置生成新的数字块
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
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
