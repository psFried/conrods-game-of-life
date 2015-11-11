
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

    pub fn new(size: usize) -> Game {
        let mut matrix: Vec<Vec<bool>> = Vec::with_capacity(size);

        for i in 0..size {
            matrix.push(vec![false; size]);
        }
    
        Game{ matrix: matrix }
    }

    pub fn size(&self) -> usize {
        self.matrix.len()
    }

    pub fn contains(&self, cell: CellLocation) -> bool {
        self.matrix.len() > cell.y &&
            self.matrix[0].len() > cell.x
    }

    pub fn is_alive(&self, cell: CellLocation) -> bool {
        self.matrix[cell.y][cell.x]
    }

    pub fn set_state(&mut self, cell: CellLocation, alive: bool) {
        self.matrix[cell.y][cell.x] = alive;
    }

    pub fn count_adjacent_live(&self, cell: CellLocation) -> usize {
        let min_x = cell.x.saturating_sub(1);
        let min_y = cell.y.saturating_sub(1);
        
        let mut count: usize = 0;
        for current_x in min_x..(cell.x + 2) {
            for current_y in min_y..(cell.y + 2) {
                let current_cell = CellLocation::new(current_x, current_y);
                if self.contains(current_cell) && 
                    self.is_alive(current_cell) &&
                    current_cell != cell {
                        count += 1;
                }
            }
        }

        count
    }

    pub fn locations(&self) -> Vec<CellLocation> {
        let mut locations: Vec<CellLocation> = Vec::with_capacity(self.size() * self.size());
        for (y, row) in self.matrix.iter().enumerate() {
            for (x, state) in row.iter().enumerate() {
                locations.push(CellLocation::new(y, x));
            }
        }
        locations
    }

    pub fn update(self) -> Game {
        let mut new_game = Game::new(self.size());
        for location in self.locations() {
            let state = self.is_alive(location);
            let adjacent_live_cells = self.count_adjacent_live(location);
            let new_state = Game::get_new_state(state, adjacent_live_cells);
            println!("location: {:?}, alive?: {:?}, adj: {:?}, new_state: {:?}", location, state,
                     adjacent_live_cells, new_state);
            new_game.set_state(location, new_state);
        }
        new_game

    }

    fn get_new_state(is_alive: bool, live_neighbors_count: usize) -> bool {
        if is_alive {
            live_neighbors_count >= 2 && live_neighbors_count <= 3 
        } else {
            live_neighbors_count == 3
        }
    }

}
