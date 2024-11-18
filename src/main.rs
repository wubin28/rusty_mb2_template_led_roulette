#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

// 定义轮盘路径上的所有位置坐标
const ROULETTE_PATH: [(usize, usize); 16] = [
    (0, 0), (0, 1), (0, 2), (0, 3),  // 上边
    (0, 4), (1, 4), (2, 4), (3, 4),  // 右边
    (4, 4), (4, 3), (4, 2), (4, 1),  // 下边
    (4, 0), (3, 0), (2, 0), (1, 0),  // 左边
];

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    
    // 当前LED位置的索引
    let mut current_pos = 0;
    
    loop {
        // 创建空的显示矩阵
        let mut led_matrix = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];
        
        // 获取当前位置的坐标
        let (row, col) = ROULETTE_PATH[current_pos];
        // 在当前位置点亮LED
        led_matrix[row][col] = 1;
        
        // 显示当前LED矩阵
        display.show(&mut timer, led_matrix, 100);
        
        // 更新位置到下一个点
        current_pos = (current_pos + 1) % ROULETTE_PATH.len();
        
        // 延时一小段时间，控制旋转速度
        timer.delay_ms(1_u32);
    }
}