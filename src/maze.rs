use crate::block::Block;
use crate::constants::DIRECTION;
use rand::seq::SliceRandom;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn cancelInterval(token: f64);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Maze {
    pub cols: usize,
    pub rows: usize,
    pub blocks: Vec<Block>,
    pub stack: Vec<usize>,
}

impl Maze {
    pub fn new(cols: usize, rows: usize) -> Maze {
        let mut maze = Maze {
            cols,
            rows,
            blocks: vec![],
            stack: vec![0],
        };

        for y in 0..rows {
            for x in 0..cols {
                maze.blocks.push(Block::new(x, y))
            }
        }

        maze
    }

    pub fn draw_maze(&self, context: &web_sys::CanvasRenderingContext2d) {
        for block in &self.blocks {
            block.draw(&context);
        }
    }

    pub fn possible_directions(&self, index: usize) -> Vec<DIRECTION> {
        let mut directions: Vec<DIRECTION> = vec![];
        let block = &self.blocks[index];

        if block.y > 0 && block.walls[DIRECTION::UP as usize] {
            let up = &self.blocks[index - self.cols];
            if !up.visited {
                directions.push(DIRECTION::UP);
            }
        }

        if block.x < self.cols - 1 && block.walls[DIRECTION::RIGHT as usize] {
            let right = &self.blocks[index + 1];

            if !right.visited {
                directions.push(DIRECTION::RIGHT);
            }
        }

        if block.y < self.rows - 1 && block.walls[DIRECTION::DOWN as usize] {
            let down = &self.blocks[index + self.cols];
            if !down.visited {
                directions.push(DIRECTION::DOWN);
            }
        }

        if block.x > 0 && block.walls[DIRECTION::LEFT as usize] {
            let left = &self.blocks[index - 1];
            if !left.visited {
                directions.push(DIRECTION::LEFT);
            }
        }

        directions
    }

    pub fn get_random_way(&self, index: usize) -> Option<DIRECTION> {
        let possible_directions = self.possible_directions(index);

        if possible_directions.len() == 0 {
            return None;
        }

        if possible_directions.len() == 1 {
            return Some(possible_directions[0]);
        }

        let random_item = possible_directions.choose(&mut rand::thread_rng()).unwrap();
        Some(*random_item)
    }

    pub fn run(&mut self) {
        if self.stack.len() == 0 {
            return;
        }
        let current_index = *self.stack.last().unwrap();
        let random_way = self.get_random_way(current_index);

        match random_way {
            Some(value) => {
                let next_index = match value {
                    DIRECTION::UP => current_index - self.cols,
                    DIRECTION::RIGHT => current_index + 1,
                    DIRECTION::DOWN => current_index + self.cols,
                    DIRECTION::LEFT => current_index - 1,
                };
                self.break_wall(current_index, next_index);
                self.stack.push(next_index);
            }
            None => {
                while self.stack.len() > 1
                    && self
                        .possible_directions(self.stack.last().unwrap().clone())
                        .len()
                        == 0
                {
                    self.stack.pop();
                }
                return;
            }
        }
    }

    pub fn break_wall(&mut self, current_index: usize, next_index: usize) {
        self.blocks[current_index].visited = true;
        self.blocks[next_index].visited = true;

        if current_index == next_index + self.cols {
            self.blocks[current_index].walls[DIRECTION::UP as usize] = false;
            self.blocks[next_index].walls[DIRECTION::DOWN as usize] = false;
            return;
        }

        if next_index > 1 && current_index == next_index - 1 {
            self.blocks[current_index].walls[DIRECTION::RIGHT as usize] = false;
            self.blocks[next_index].walls[DIRECTION::LEFT as usize] = false;
            return;
        }

        if next_index > self.cols && current_index == next_index - self.cols {
            self.blocks[current_index].walls[DIRECTION::DOWN as usize] = false;
            self.blocks[next_index].walls[DIRECTION::UP as usize] = false;
            return;
        }

        self.blocks[current_index].walls[DIRECTION::LEFT as usize] = false;
        self.blocks[next_index].walls[DIRECTION::RIGHT as usize] = false;
        return;
    }
}
