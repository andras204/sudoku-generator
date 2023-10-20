mod cell;

use cell::Cell;
use rand::prelude::*;


pub struct Board {
    matrix: Vec<Vec<Cell>>,
}


impl Board {
    pub fn new() -> Board {
        Board { matrix: vec![vec![Cell::new(); 9]; 9] }
    }

    pub fn play_move(&mut self, x: usize, y: usize, num: u8) -> Result<u8, &str>{
        let backup_board = self.matrix.to_owned();
        self.matrix[x][y].collapse(num).unwrap();

        match self.propagate_collapse((x, y, num)) {
            Ok(_ok) => return Ok(0),
            Err(_err) => {
                self.matrix = backup_board;
                return Err("invalid move");
            }
        }
    }

    fn propagate_collapse(&mut self, collapse: (usize, usize, u8)) -> Result<u8, &str> {
        let mut propagation_queue: Vec<(usize, usize, u8)> = Vec::new();

        for y in 0..9 {
            for x in 0..9 {
                if !self.matrix[collapse.0][y].is_collapsed() {
                    match self.matrix[collapse.0][y].reduce(collapse.2) {
                        Ok(colled) => if colled {
                                propagation_queue.push((collapse.0, y, self.matrix[collapse.0][y].get_number().unwrap()));
                            },
                        Err(_err) => return Err("invalid move")
                    }
                }

                if !self.matrix[x][collapse.1].is_collapsed() {
                    match self.matrix[x][collapse.1].reduce(collapse.2) {
                        Ok(colled) => if colled {
                                propagation_queue.push((x, collapse.1, self.matrix[x][collapse.1].get_number().unwrap()));
                            },
                        Err(_err) => return Err("invalid move")
                    }
                }
            }
        }

        let sbsq = self.get_subsquare(collapse.0, collapse.1);

        for y in sbsq.1..(sbsq.1 + 3) {
            for x in sbsq.0..(sbsq.0 + 3) {
                if !self.matrix[x][y].is_collapsed() {
                    match self.matrix[x][y].reduce(collapse.2) {
                        Ok(colled) => if colled {
                                propagation_queue.push((x, y, self.matrix[x][y].get_number().unwrap()));
                            },
                        Err(_err) => return Err("invalid move")
                    }
                }
            }
        }

        while propagation_queue.len() > 0 {
            match self.propagate_collapse(propagation_queue.pop().unwrap()) {
                Ok(_ok) => continue,
                Err(_err) => return Err("invalid move")
            }
        }
        Ok(0)
    }

    fn get_subsquare(&self, x: usize, y: usize) -> (usize, usize) {
        (x / 3 * 3, y / 3 * 3)
    }

    pub fn select_random_number(&self, cell: (usize, usize)) -> (usize, usize, u8) {
        let p_vec = self.matrix[cell.0][cell.1].get_possible_numbers();
        let mut rng = rand::thread_rng();

        (cell.0, cell.1, p_vec[rng.gen_range(0..p_vec.len())])
    }

    pub fn select_random_cell(&self) -> (usize, usize) {
        let min_vec = self.get_min_space_vec();
        let mut rng = rand::thread_rng();
        let index: usize = rng.gen_range(0..min_vec.len());

        (min_vec[index].0, min_vec[index].1)
    }

    pub fn get_min_space_vec(&self) -> Vec<(usize, usize)> {
        let mut min_vec: Vec<(usize, usize, u8)> = Vec::with_capacity(81);
        min_vec.push((10, 10, 10));

        for y in 0..9 {
            for x in 0..9 {
                if !self.matrix[x][y].is_collapsed() {
                    if self.matrix[x][y].get_count() < min_vec[0].2 {
                        min_vec.clear();
                        min_vec.push((x, y, self.matrix[x][y].get_count()));
                    }
                    if self.matrix[x][y].get_count() == min_vec[0].2 {
                        min_vec.push((x, y, self.matrix[x][y].get_count()));
                    }
                }
            }
        }

        let mut final_vec: Vec<(usize, usize)> = Vec::with_capacity(min_vec.len());

        while min_vec.len() > 0 {
            let value = min_vec.pop().unwrap();
            final_vec.push((value.0, value.1));
        }

        return final_vec;
    }

    pub fn get_collapsed_vec(&self) -> Vec<(usize, usize)> {
        let mut colled_vec: Vec<(usize, usize)> = Vec::with_capacity(81);
        
        for y in 0..9 {
            for x in 0..9 {
                if self.matrix[x][y].is_collapsed() {
                    colled_vec.push((x, y));
                }
            }
        }

        return colled_vec;
    }

    pub fn reset_cell(&mut self, cell: &(usize, usize)) {
        self.matrix[cell.0][cell.1].reset();
    }

    pub fn is_finished(&self) -> bool {
        for y in 0..9 {
            for x in 0..9 {
                if !self.matrix[x][y].is_collapsed() {
                    return false;
                }
            }
        }
        true
    }

    pub fn to_string(&self) -> String {
        let mut string: String = "".to_owned();
        for y in 0..9 {
            for x in 0..9 {
                let s = match self.matrix[x][y].get_number() {
                    Ok(num) => (num + 1).to_string() + " ",
                    Err(_err) => ". ".to_owned()
                };
                string += &s;
            }
            string += "\n"
        }
        string
    }

    pub fn debug_print_counts(&self) -> String {
        let mut string: String = "".to_owned();
        for y in 0..9 {
            for x in 0..9 {
                string += &(self.matrix[x][y].get_count().to_string() + " ");
            }
            string += "\n"
        }
        string
    }
}

