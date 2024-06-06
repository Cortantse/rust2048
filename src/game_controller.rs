use crate::GameBoard;

pub struct GameController {
    board: GameBoard,
    score: u32,
}

impl GameController {
    pub fn new() -> Self {
        Self { board: GameBoard::new(), score: 0 }
    }

    pub fn start(&mut self) {
        // 开始新游戏
    }

    pub fn update(&mut self) {
        // 更新游戏状态，响应用户输入
    }

    pub fn render(&self) {
        // 绘制游戏状态到命令行或GUI
    }
}
