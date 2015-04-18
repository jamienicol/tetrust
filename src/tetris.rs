extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::RenderDrawer;

const BLOCK_SIZE: usize = 48;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

#[derive(Copy, Clone)]
enum BlockColor {
    Cyan,
    Yellow,
    Purple,
    Green,
    Red,
    Blue,
    Orange
}

#[derive(Copy, Clone)]
struct Block {
    color: BlockColor,
}

impl Block {
    fn new(color: BlockColor) -> Block {
        Block { color: color }
    }

    fn draw(&self, drawer: &mut RenderDrawer, x: i32, y: i32) {
        let r = Rect {
            x: x,
            y: y,
            w: BLOCK_SIZE as i32,
            h: BLOCK_SIZE as i32
        };
        drawer.set_draw_color(
            match self.color {
                BlockColor::Cyan => Color::RGB(0, 255, 255),
                BlockColor::Yellow => Color::RGB(255, 255, 0),
                BlockColor::Purple => Color::RGB(255, 0, 255),
                BlockColor::Green => Color::RGB(0, 255, 0),
                BlockColor::Red => Color::RGB(255, 0, 0),
                BlockColor::Blue => Color::RGB(0, 0, 255),
                BlockColor::Orange => Color::RGB(255, 165, 0)
            });
        drawer.fill_rect(r);
    }
}

pub struct Board {
    grid: [[Option<Block>; BOARD_HEIGHT]; BOARD_WIDTH]
}

impl Board {
    pub fn new() -> Board {
        Board {
            grid: [[None; BOARD_HEIGHT]; BOARD_WIDTH]
        }
    }

    pub fn draw(&self, drawer: &mut RenderDrawer) {
        for (x, column) in self.grid.iter().enumerate() {
            for (y, &square) in column.iter().enumerate() {
                match square {
                    Some(block) => block.draw(drawer,
                                              (x * BLOCK_SIZE) as i32,
                                              (y * BLOCK_SIZE) as i32),
                    None => {}
                }
            }
        }
    }
}
