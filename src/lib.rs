use std::{default, usize, string};

use wasm_bindgen::prelude::*;
// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/rnd.js")]
extern "C" {
    fn rnd(max: usize) -> usize;
}

#[wasm_bindgen]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum GameStatus {
    Won,
    Lost,
    Played,
}

#[wasm_bindgen]
#[derive(PartialEq, Clone)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body = vec![];

        for i in 0..size {
            if (spawn_index as i32) - i as i32 >= 0 {
                body.push(SnakeCell(spawn_index - i));
            }
        }

        Snake {
            body,
            direction: Direction::Right,
        }
    }

    fn change_direction(&mut self, dir: Direction) {
        self.direction = dir;
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: Option<usize>,
    points: usize,
    status: Option<GameStatus>,
}

#[wasm_bindgen]
impl World {
    pub fn game_start(&mut self){
        self.status = Some(GameStatus::Played);
    }

    pub fn game_status(& self) -> Option<GameStatus>{
        self.status
    }

    pub fn get_display_status(&self) -> String{
        match self.game_status() {
            None => {String::from("No status")},
            Some(GameStatus::Played) =>{String::from("Playing...")}, 
            Some(GameStatus::Won) =>{String::from("You have won!")}, 
            Some(GameStatus::Lost) =>{String::from("WASTED")}, 
 
        }
    }

    pub fn new(width: usize, snake_idx: usize) -> World {
        let snake = Snake::new(snake_idx, 3);
        let size = width * width;

        let reward_cell = World::generate_reaward_cell(&snake, size);

        World {
            width,
            size,
            snake,
            next_cell: None,
            reward_cell: Some(reward_cell),
            status: Option::None,
            points:0
        }
    }

    pub fn points(&self) -> usize{
        self.points
    }

    pub fn reaward_cell(&self) -> Option<usize> {
        self.reward_cell
    }

    pub fn set_reaward_cell(&mut self, reward_cell: Option<usize>) {
        self.reward_cell = reward_cell;
    }

    pub fn new_reaward_cell(&mut self) {
        let reward_cell = World::generate_reaward_cell(&self.snake, self.size);
        self.set_reaward_cell(Some(reward_cell));
    }

    fn generate_reaward_cell(snake: &Snake, size: usize) -> usize {
        let mut reward_cell;
        loop {
            reward_cell = rnd(size);
            if !snake.body.contains(&SnakeCell(reward_cell)) {
                return reward_cell;
            }
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_dir(&mut self, dir: Direction) {
        let next_cell = self.gen_next_snake_cell(&dir);

        if self.snake.body[1].0 == next_cell.0 {
            return;
        }

        // if self.snake.body.contains(&next_cell) {return;}

        self.next_cell = Some(next_cell);
        self.snake.change_direction(dir);
    }

    pub fn snake_len(&self) -> usize {
        self.snake.body.len()
    }

    // *const is raw pointer
    // boprrowing rules doesn't apply to it
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    // cannot return reference to JS because of borrowing rules
    //     pub fn snake_cells(&self) -> &Vec<SnakeCell>{
    //         &self.snake.body
    // }

    pub fn step(&mut self) {
        match self.status {
            Some(GameStatus::Played) => {
                let next_cell = self
                    .next_cell
                    .get_or_insert(self.gen_next_snake_cell(&self.snake.direction));

                self.snake.body.insert(0, next_cell.clone());

                if Some(next_cell.0) == self.reaward_cell() {
                    if self.snake_len() < self.size {
                        self.new_reaward_cell();
                    } else {
                        self.set_reaward_cell(None);
                        self.status = Some(GameStatus::Won);
                    }

                    self.points= self.points+ 1;

                } else {
                    self.snake.body.pop();
                }

                self.next_cell = None;
                // let next_cell = self.gen_next_snake_cell(&self.snake.direction);
                // self.snake.body.insert(0, next_cell);
                // self.snake.body.pop();

                if self.snake.body[1..].contains(&self.snake.body[0]){
                    self.status = Some(GameStatus::Lost);
                }
            }
            _ => {}
        }
    }

   

    fn gen_next_snake_cell(&self, dir: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let current_row = snake_idx / self.width();

        match dir {
            Direction::Right => {
                SnakeCell((current_row * self.width()) + (snake_idx + 1) % self.width())
            }
            Direction::Left => {
                let next_row = (snake_idx - 1) / self.width();
                if next_row == current_row {
                    SnakeCell((current_row * self.width()) + (snake_idx - 1) % self.width())
                } else {
                    SnakeCell((current_row * self.width()) + self.width() - 1)
                }
            }
            Direction::Up => {
                if (snake_idx as i32 - self.width() as i32) >= 0 {
                    SnakeCell(snake_idx - self.width())
                } else {
                    SnakeCell(snake_idx + self.size - self.width())
                }
            }
            Direction::Down => SnakeCell((snake_idx + self.width()) % self.size),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_your_code() {
        // TODO: whrite tests
    }
}
