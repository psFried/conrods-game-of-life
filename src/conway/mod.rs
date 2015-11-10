
#[cfg(test)]
mod test;

use std::cmp;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Cell {
    x: usize,
    y: usize
}

impl Cell {

    pub fn new(x: usize, y: usize) -> Cell {
        Cell{x: x, y: y}
    }

}

pub struct Game {
    matrix: Vec<Vec<bool>>
}

impl Game {

    pub fn new(size: usize) -> Game {
        let mut matrix: Vec<Vec<bool>> = Vec::with_capacity(size);

        for i in 0..size {
            matrix.push(vec![false; size]);
        }
    
        Game{ matrix: matrix }
    }

    pub fn contains(&self, cell: Cell) -> bool {
        self.matrix.len() > cell.x &&
            self.matrix[0].len() > cell.y
    }

    pub fn is_alive(&self, cell: Cell) -> bool {
        self.matrix[cell.x][cell.y]
    }

    pub fn set_state(&mut self, cell: Cell, alive: bool) {
        self.matrix[cell.x][cell.y] = alive;
    }

    pub fn count_adjacent_live(&self, cell: Cell) -> usize {
        let min_x = cell.x.saturating_sub(1);
        let min_y = cell.y.saturating_sub(1);
        
        let mut count: usize = 0;
        for current_x in min_x..(cell.x + 2) {
            for current_y in min_y..(cell.y + 2) {
                let current_cell = Cell::new(current_x, current_y);
                if self.contains(current_cell) && 
                    self.is_alive(current_cell) &&
                    current_cell != cell {
                        count += 1;
                }
            }
        }

        count
    }

}
