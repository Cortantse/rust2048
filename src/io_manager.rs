pub struct IOManager {
    last_input_time: std::time::Instant,
    io_response_interval: std::time::Duration,
}

impl IOManager {
    pub fn new(response_interval_ms: u64) -> Self {
        Self {
            last_input_time: std::time::Instant::now(),
            io_response_interval: std::time::Duration::from_millis(response_interval_ms),
        }
    }

    pub fn read_input(&mut self) -> Option<String> {
        // 读取用户输入，处理输入频率限制
        None
    }

    pub fn write_output(&self, message: &str) {
        // 输出信息到终端或GUI
    }

    pub fn clear_screen(&self) {
        // 清除屏幕
    }

    pub fn update_last_input_time(&mut self) {
        // 更新最后输入时间
        self.last_input_time = std::time::Instant::now();
    }
}
