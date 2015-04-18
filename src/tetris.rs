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

pub enum TetrominoShape { I, O, T, S, Z, J, L }

pub struct Tetromino {
    x: i32,
    y: i32,
    blocks: [(i32, i32, Block); 4]
}

impl Tetromino {
    pub fn new(shape: TetrominoShape) -> Tetromino {
        Tetromino {
            x: 0,
            y: 0,
            blocks: match shape {
                TetrominoShape::I => [(-1, -1, Block::new(BlockColor::Cyan)),
                                      (0, -1, Block::new(BlockColor::Cyan)),
                                      (1, -1, Block::new(BlockColor::Cyan)),
                                      (2, -1, Block::new(BlockColor::Cyan))],

                TetrominoShape::O => [(0, -1, Block::new(BlockColor::Yellow)),
                                      (1, -1, Block::new(BlockColor::Yellow)),
                                      (0, 0, Block::new(BlockColor::Yellow)),
                                      (1, 0, Block::new(BlockColor::Yellow))],

                TetrominoShape::T => [(-1, 0, Block::new(BlockColor::Purple)),
                                      (0, 0, Block::new(BlockColor::Purple)),
                                      (1, 0, Block::new(BlockColor::Purple)),
                                      (0, -1, Block::new(BlockColor::Purple))],

                TetrominoShape::S => [(-1, 0, Block::new(BlockColor::Green)),
                                      (0, 0, Block::new(BlockColor::Green)),
                                      (0, -1, Block::new(BlockColor::Green)),
                                      (1, -1, Block::new(BlockColor::Green))],

                TetrominoShape::Z => [(-1, -1, Block::new(BlockColor::Red)),
                                      (0, -1, Block::new(BlockColor::Red)),
                                      (0, 0, Block::new(BlockColor::Red)),
                                      (1, 0, Block::new(BlockColor::Red))],

                TetrominoShape::J => [(-1, -1, Block::new(BlockColor::Blue)),
                                      (-1, 0, Block::new(BlockColor::Blue)),
                                      (0, 0, Block::new(BlockColor::Blue)),
                                      (1, 0, Block::new(BlockColor::Blue))],

                TetrominoShape::L => [(1, -1, Block::new(BlockColor::Orange)),
                                      (-1, 0, Block::new(BlockColor::Orange)),
                                      (0, 0, Block::new(BlockColor::Orange)),
                                      (1, 0, Block::new(BlockColor::Orange))]
            }
        }
    }

    fn check_collision(&self) -> bool {
        for &(x, _, _) in self.blocks.iter() {
            if (self.x + x as i32) < 0 {
                return true;
            }
            if (self.x + x as i32) >= BOARD_WIDTH as i32 {
                return true;
            }
        }

        return false;
    }

    pub fn move_left(&mut self, ) {
        let orig_x = self.x;

        self.x -= 1;

        if self.check_collision() {
            self.x = orig_x;
        }
    }

    pub fn move_right(&mut self) {
        let orig_x = self.x;

        self.x += 1;

        if self.check_collision() {
            self.x = orig_x;
        }
    }

    pub fn draw(&self, drawer: &mut RenderDrawer) {
        for &(x, y, block) in self.blocks.iter() {
            block.draw(drawer,
                       (self.x + x) * BLOCK_SIZE as i32,
                       (self.y + y) * BLOCK_SIZE as i32);
        }
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
