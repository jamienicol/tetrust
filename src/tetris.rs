extern crate rand;
extern crate sdl2;

use rand::Rng;
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

#[derive(Copy, Clone)]
pub struct Tetromino {
    x: i32,
    y: i32,
    blocks: [(i32, i32, Block); 4]
}

impl Tetromino {
    pub fn new(shape: TetrominoShape) -> Tetromino {
        Tetromino {
            x: 4,
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

    fn check_collision(&self, board: &Board) -> bool {
        for &(x, y, _) in self.blocks.iter() {
            let block_x = self.x + x;
            let block_y = self.y + y;

            if block_x < 0 {
                return true;
            }
            if block_x >= BOARD_WIDTH as i32 {
                return true;
            }
            if block_y >= BOARD_HEIGHT as i32 {
                return true;
            }
            if block_y >= 0 {
                if board.grid[block_x as usize][block_y as usize].is_some() {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn move_left(&mut self, board: &Board) {
        let orig_x = self.x;

        self.x -= 1;

        if self.check_collision(board) {
            self.x = orig_x;
        }
    }

    pub fn move_right(&mut self, board: &Board) {
        let orig_x = self.x;

        self.x += 1;

        if self.check_collision(board) {
            self.x = orig_x;
        }
    }

    pub fn rotate_clockwise(&mut self, board: &Board) {
        let orig_x = self.x;
        let orig_blocks = self.blocks;

        for (i, &(x, y, block)) in orig_blocks.iter().enumerate() {
            self.blocks[i] = (-y, x, block);
        }

        if self.check_collision(board) {
            self.x = orig_x - 1;
            if self.check_collision(board) {
                self.x = orig_x + 1;
                if self.check_collision(board) {
                    self.x = orig_x;
                    self.blocks = orig_blocks;
                }
            }
        }
    }

    pub fn move_down(&mut self, board: &Board) -> bool {
        let orig_y = self.y;

        self.y += 1;

        if self.check_collision(board) {
            self.y = orig_y;
            return false;
        }

        return true;
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

    pub fn add_tetromino(&mut self, tetromino: Tetromino) {
        for &(x, y, block) in tetromino.blocks.iter() {
            let block_x = (tetromino.x + x) as usize;
            let block_y = (tetromino.y + y) as usize;

            self.grid[block_x][block_y] = Some(block);
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

pub struct Game {
    board: Board,
    tetromino: Tetromino,
    time: u32
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            tetromino: Tetromino::new(Game::get_random_shape()),
            time: 0
        }
    }

    pub fn input_left(&mut self) {
        self.tetromino.move_left(&self.board);
    }

    pub fn input_right(&mut self) {
        self.tetromino.move_right(&self.board);
    }

    pub fn input_rotate(&mut self) {
        self.tetromino.rotate_clockwise(&self.board)
    }

    pub fn input_down(&mut self) {
        self.time = 0;

        let did_move = self.tetromino.move_down(&self.board);
        if !did_move {
            self.board.add_tetromino(self.tetromino);
            self.tetromino = Tetromino::new(Game::get_random_shape());
        }
    }

    pub fn advance_ms(&mut self, ms: u32) {
        self.time += ms;

        if self.time >= 1000 {
            self.input_down();
        }
    }

    pub fn draw(&self, drawer: &mut RenderDrawer) {
        self.board.draw(drawer);
        self.tetromino.draw(drawer);
    }

    fn get_random_shape() -> TetrominoShape {
        match rand::thread_rng().gen_range(0, 7) {
            0 => TetrominoShape::I,
            1 => TetrominoShape::O,
            2 => TetrominoShape::T,
            3 => TetrominoShape::S,
            4 => TetrominoShape::Z,
            5 => TetrominoShape::J,
            6 => TetrominoShape::L,
            _ => panic!()
        }
    }
}
