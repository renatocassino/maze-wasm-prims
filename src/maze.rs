use rand::Rng;
use crate::block::Block;
use crate::constants::DIRECTION;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use wasm_bindgen::prelude::*;

pub struct Maze {
    pub cols: usize,
    pub rows: usize,
    pub blocks: Vec<Block>,
    pub possible_ways: HashMap<usize, bool>,
}

impl Maze {
    pub fn new(cols: usize, rows: usize) -> Maze {
        let mut maze = Maze {
            cols,
            rows,
            blocks: vec![],
            possible_ways: HashMap::new(),
        };

        for y in 0..rows {
            for x in 0..cols {
                maze.blocks.push(Block::new(x, y))
            }
        }

        let mut rng = rand::thread_rng();
        let index_first_block = rng.gen_range(0..maze.blocks.len());

        maze.blocks[index_first_block].visited = true;
        maze.append_positions_next_block(index_first_block);

        maze
    }

    fn get_positions_next_block(&mut self, index: usize) -> Vec<usize> {
        let mut positions_next_block = vec![];
        let block = &self.blocks[index];

        if block.y > 0 {
            let index_top_block = (block.y - 1) * self.cols + block.x;
            if !self.blocks[index_top_block].visited {
                positions_next_block.push(index_top_block);
            }
        }

        if block.x < self.cols - 1 {
            let index_right_block = block.y * self.cols + block.x + 1;
            if !self.blocks[index_right_block].visited {
                positions_next_block.push(index_right_block);
            }
        }

        if block.y < self.rows - 1 {
            let index_bottom_block = (block.y + 1) * self.cols + block.x;
            if !self.blocks[index_bottom_block].visited {
                positions_next_block.push(index_bottom_block);
            }
        }

        if block.x > 0 {
            let index_left_block = block.y * self.cols + block.x - 1;
            if !self.blocks[index_left_block].visited {
                positions_next_block.push(index_left_block);
            }
        }

        positions_next_block
    }

    fn append_positions_next_block(&mut self, index: usize) {
        let new_list = self.get_positions_next_block(index);
        for new_index in new_list {
            self.possible_ways.insert(new_index, true);
        }
    }

    pub fn draw_maze(&self, context: &web_sys::CanvasRenderingContext2d) {
        for block in &self.blocks {
            block.draw(&context);
        }
    }

    pub fn possible_directions(&self, index: usize) -> Vec<usize> {
        let mut directions: Vec<usize> = vec![];
        let block = &self.blocks[index];

        if block.y > 0 && block.walls[DIRECTION::UP as usize] {
            let up = &self.blocks[index - self.cols];
            if up.visited {
                directions.push(index - self.cols);
            }
        }

        if block.x < self.cols - 1 && block.walls[DIRECTION::RIGHT as usize] {
            let right = &self.blocks[index + 1];

            if right.visited {
                directions.push(index + 1);
            }
        }

        if block.y < self.rows - 1 && block.walls[DIRECTION::DOWN as usize] {
            let down = &self.blocks[index + self.cols];
            if down.visited {
                directions.push(index + self.cols);
            }
        }

        if block.x > 0 && block.walls[DIRECTION::LEFT as usize] {
            let left = &self.blocks[index - 1];
            if left.visited {
                directions.push(index - 1);
            }
        }

        directions
    }

    pub fn get_visited_neighborhood(&self, index: usize) -> Option<usize> {
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

    pub fn get_random_possible_block(&mut self) -> Option<usize> {
        if self.possible_ways.len() == 0 {
            return None;
        }

        let keys: Vec<usize> = self.possible_ways.keys().cloned().collect();
        let random_item = keys.choose(&mut rand::thread_rng()).unwrap();
        self.possible_ways.remove(random_item);
        return Some(*random_item);
    }

    pub fn run(&mut self, context: &web_sys::CanvasRenderingContext2d) {
        if self.possible_ways.len() == 0 {
            return;
        }

        let random_way = self.get_random_possible_block();
        match random_way {
            Some(index) => {
                let next_block = self.get_visited_neighborhood(index).unwrap();
                self.append_positions_next_block(index);
                self.break_wall(index, next_block);
                self.draw_blocks(index, next_block, context);
            }
            None => {}
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

    pub fn draw_blocks(&mut self, current_index: usize, next_index: usize, context: &web_sys::CanvasRenderingContext2d) {
        self.blocks[current_index].draw(context);
        self.blocks[next_index].draw(context);
    }
}
