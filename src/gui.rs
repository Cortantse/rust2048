struct GUI {
    window: Window, // 假设Window是某个GUI库中的窗口类型
}

impl GUI {
    pub fn new() -> Self {
        Self { window: Window::new() }
    }

    pub fn draw_board(&self) {
        // 绘制棋盘GUI
    }

    pub fn update_score(&self, score: u32) {
        // 更新分数显示
    }

    pub fn handle_events(&self) {
        // 处理用户事件
    }
}
