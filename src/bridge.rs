use crate::GameBoard;
use crate::Direction;

pub struct Bridge {
    if_bin_direction: bool, // 是否是双向通道，若是，则下方忽略
    direction: Direction,
    if_open: bool, // 当前通道的开启状态
    left_index: usize, // 左侧棋盘的index
    right_index: usize, // 右侧棋盘的index
    sending_limit: usize, // 该通道最多能发送的数量
}

impl Bridge {
    pub fn new(if_bin_direction: bool, direction: Direction, if_open: bool, left_index: usize, right_index: usize, sending_limit: usize) -> Self {
        Self {
            if_bin_direction: if_bin_direction,
            direction: direction,
            if_open: if_open,
            left_index: left_index,
            right_index: right_index,
            sending_limit: sending_limit,
        }
    }

    pub fn update_status(&mut self, if_bin_direction: bool, direction: Direction, if_open: bool, left_index: usize, right_index: usize, sending_limit: usize) {
        self.if_bin_direction = if_bin_direction;
        self.direction = direction;
        self.if_open = if_open;
        self.left_index = left_index;
        self.right_index = right_index;
        self.sending_limit = sending_limit;
    }

    pub fn send_through_bridge(&mut self, recei_board: &mut GameBoard, send_board: &mut GameBoard, direction: Direction) -> Option<Vec<u32>> {
        if !self.if_legal(recei_board, send_board, direction) {
            return None;
        }
    

        let length_of_line = recei_board.get_tiles().len();
        // 合法，那么开始处理移动
        // 发送方会将自己的方块也发送过去，直到对方该行已满
        // 如 0 2 0 0 ----- 2 2 2 0 会变成 0 2 2 2 ----- 2 0 0 0
        
        // 使用abstract functionm，这一块从右向左也能复用，上到下也能
        // 提供动画数组
        match direction {
            Direction::Left => {
                // 向左不需要做任何特殊适配
                return Some(self.send_abstract(&mut recei_board.get_tiles_mut()[self.left_index], &mut send_board.get_tiles_mut()[self.right_index]));
            }
            Direction::Right => {
                // 向右实际上是反向向左
                // 0200 => 0002 = 0000 0022     equals to    2000 <= 0020 = 2200  reverse direction to 0022 
                // 反转对应节，最后两个答案也得反转重新写入
                let mut line_receiver_reversed: Vec<u32> = recei_board.get_tiles()[self.left_index].iter().rev().cloned().collect();
                let mut line_sender_reversed: Vec<u32> = send_board.get_tiles()[self.right_index].iter().rev().cloned().collect();
                let mut animated_vector = self.send_abstract(&mut line_receiver_reversed, &mut line_sender_reversed);
                // 重新写入
                for i in 0..length_of_line {
                    recei_board.get_tiles_mut()[self.left_index][i] = line_receiver_reversed[length_of_line - 1 - i];
                }
                for i in 0..length_of_line {
                    send_board.get_tiles_mut()[self.right_index][i] = line_sender_reversed[length_of_line - 1 - i];
                }
                return Some(animated_vector);
            }
            _ => {return None;}
        }
    
    }

    fn send_abstract(&mut self, receiver_line: &mut Vec<u32>, sender_line: &mut Vec<u32>) -> Vec<u32>{
        // 先检查有几个空位
        let mut count = 0;
        let length_of_line = receiver_line.len();
        for i in (0..length_of_line).rev() {
            if receiver_line[i] == 0 {
                count += 1;
            } else {
                break;
            }
        }
        // 动画数组
        let mut animated_vector = vec![];
        // 将sender对应的数字拷走，注意需要检查是否为0
        let mut extra_num = 0;
        let mut i = 0;
        while i < count && i + extra_num < length_of_line {
            assert!(receiver_line[length_of_line - count + i] == 0);
            receiver_line[length_of_line - count + i] = sender_line[i + extra_num];
            // 方便可以到时候直接move
            if sender_line[i + extra_num] != 0 {
                animated_vector.push(sender_line[i + extra_num]);
            }else{
                // 空值，不行 发送方额外加一
                extra_num += 1;
                continue;
            }
            sender_line[i + extra_num] = 0;
            // 非空值，安全
            i += 1;
        }
        // 减去发送的数量
        self.sending_limit -= animated_vector.len();
        animated_vector
        // 删除额外数据 并整理sender
    }

    // 查看当前向尝试的通道操作是否合法
    fn if_legal(&mut self, board1: &mut GameBoard,  board2: &mut GameBoard, direction: Direction) -> bool {
        // 先检查方向是否合法
        if !self.if_bin_direction {
            if direction!= self.direction {
                return false;
            }
        }
        match direction {
            Direction::Up => {
                return false;
            }
            Direction::Down => {
                return false;
            }
            _ => {
            }
        }
        // 再检查是否还有发送数量
        if self.sending_limit <= 0 {
            return false;
        }

        // 再检查通道是否打开
        if !self.if_open {
            return false;
        }
        // 最后检查选择的方向上原有棋盘是否有内容
        if direction == Direction::Left {
            let value: u32 = board2.get_tiles()[self.right_index].iter().sum();
            if value == 0 {
                return false;
            } 
        }
        else if direction == Direction::Right {
            let value: u32 = board2.get_tiles()[self.left_index].iter().sum();
            if value == 0 {
                return false;
            } 
        }
        true
    }
}


#[cfg(test)]
mod test_bridge {
    use core::panic;

    use crate::bridge;

    use super::*;
    #[test]
    fn test_move_abstract() {
        let mut line1 = vec![0, 0, 0, 0];
        let mut line2 = vec![0, 0, 0, 0];
        let expect = vec![0, 0, 0, 0];
        let mut bridge = Bridge::new(false, Direction::Left, true, 0, 
            1, 2);
        bridge.send_abstract(&mut line1, &mut line2);
        assert_eq!(line1, expect);
    }

    #[test]
    fn test_send_through_bridge() {
        let mut game = GameBoard::new();
        *game.get_tiles_mut() = vec![
            vec![2, 2, 4, 4],
            vec![4, 4, 0, 0],
            vec![0, 4, 4, 4],
            vec![2, 0, 0, 2],
        ];
        let mut game2 = GameBoard::new();
        *game2.get_tiles_mut() = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 4],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];

        let mut bridge = Bridge::new(true, Direction::Left, true, 1,
             1, 200);
        let animinated_vector = bridge.send_through_bridge(&mut game2, &mut game, Direction::Right);

        let expected1 = vec![
            vec![2, 2, 4, 4],
            vec![0, 0, 0, 0],
            vec![0, 4, 4, 4],
            vec![2, 0, 0, 2],
        ];
        let expected2 = vec![
            vec![0, 0, 0, 0],
            vec![0, 4, 4, 4],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];

        assert_eq!(*game.get_tiles(), expected1);
        assert_eq!(*game2.get_tiles(), expected2);
        match animinated_vector {
            Some(animinated_vector) => {
                assert_eq!(animinated_vector, vec![4, 4]);
            }
            None => {
                assert!(false);
            }
        }

    }
    
}
