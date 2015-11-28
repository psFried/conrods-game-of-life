
#[cfg(test)]
mod test;

use std::cmp;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CellLocation {
    x: usize,
    y: usize
}

impl CellLocation {

    pub fn new(x: usize, y: usize) -> CellLocation {
        CellLocation{x: x, y: y}
    }

}

pub struct Game {
    matrix: Vec<Vec<bool>>
}

impl Game {

    pub fn new(width: usize, height: usize) -> Game {
        let mut matrix: Vec<Vec<bool>> = Vec::with_capacity(height);

        for i in 0..height {
            matrix.push(vec![false; width]);
        }
    
        Game{ matrix: matrix }
    }

    pub fn height(&self) -> usize {
        self.matrix.len()
    }

    pub fn width(&self) -> usize {
        self.matrix[0].len()
    }

    pub fn is_alive(&self, cell: CellLocation) -> bool {
        self.matrix[cell.y][cell.x]
    }

    pub fn set_state(&mut self, cell: CellLocation, alive: bool) {
        self.matrix[cell.y][cell.x] = alive;
    }

    pub fn adjacent_cells(&self, center_cell: CellLocation) -> Vec<CellLocation> {
        let mut cells: Vec<CellLocation> = Vec::new();
        let max_x: usize = self.width() - 1;
        let max_y: usize = self.height() - 1;
        
        let mut x = Game::wrapping_sub(center_cell.x, 1, max_x);
        let mut y = Game::wrapping_sub(center_cell.y, 1, max_y);

        for yi in 0..3 {
            let cell_y = Game::wrapping_add(y, yi, max_y);
            for xi in 0..3 {
                let cell_x = Game::wrapping_add(x, xi, max_x);
                cells.push(CellLocation::new(cell_x, cell_y));
            }
        }
        cells
    }

    pub fn update(self) -> Game {
        let mut new_game = Game::new(self.width(), self.height());
        for location in self.locations() {
            let state = self.is_alive(location);
            let adjacent_live_cells = self.count_adjacent_live(location);
            let new_state = Game::get_new_state(state, adjacent_live_cells);
            new_game.set_state(location, new_state);
        }
        new_game

    }

    pub fn resize(&self, new_w: usize, new_h: usize) -> Game {
        let mut new_game = Game::new(new_w, new_h);

        for loc in self.locations() {
            if new_game.contains(loc) {
                new_game.set_state(loc, self.is_alive(loc));
            }
        }
        new_game
    }

    fn count_adjacent_live(&self, cell: CellLocation) -> usize {
        let mut count: usize = 0;
        for loc in self.adjacent_cells(cell) {
            if self.is_alive(loc) {
                count += 1;
            }
        }
        count
    }

    fn contains(&self, location: CellLocation) -> bool {
        self.matrix.len() > location.y && self.matrix[0].len() > location.x
    }

    fn locations(&self) -> Vec<CellLocation> {
        let mut locations: Vec<CellLocation> = Vec::with_capacity(self.width() * self.height());
        for (y, row) in self.matrix.iter().enumerate() {
            for (x, state) in row.iter().enumerate() {
                locations.push(CellLocation::new(x, y));
            }
        }
        locations
    }


    fn wrapping_sub(left: usize, right: usize, max: usize) -> usize {
        let initial_result = left.wrapping_sub(right);
        if (initial_result > left) {
            max - (usize::max_value() - initial_result)
        } else {
            initial_result
        }
    }

    fn wrapping_add(left: usize, right: usize, max: usize) -> usize {
        let initial_result = left.wrapping_add(right);
        if initial_result > max {
            initial_result - max - 1
        } else {
            initial_result
        }
    }

    fn get_new_state(is_alive: bool, live_cell_count: usize) -> bool {
        match live_cell_count {
            3 => true,
            4 => is_alive,
            _ => false
        }
    }

}
