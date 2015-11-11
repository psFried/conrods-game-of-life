use super::*;

#[test]
fn new_game_should_create_game_of_given_size() {
    let game: Game = Game::new(8);

    assert_eq!(8, game.matrix.len());
    for row in game.matrix {
        assert_eq!(8, row.len());
    }
}

#[test]
fn should_be_able_to_get_and_set_cell_state() {

    let mut game = Game::new(10);

    let cell = CellLocation::new(0, 0);
    assert_eq!(false, game.is_alive(cell));
    
    game.set_state(cell, true);
    assert_eq!(true, game.is_alive(cell));
}

#[test]
fn game_should_count_the_number_of_live_adjacent_cells() {

    let game = Game{
        matrix: vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true]
        ]
    };

    assert_eq!(8, game.count_adjacent_live(CellLocation::new(1, 1)));
    assert_eq!(3, game.count_adjacent_live(CellLocation::new(0, 0)));
    assert_eq!(5, game.count_adjacent_live(CellLocation::new(2, 1)));

    let game2 = Game{
        matrix: vec![
            vec![false, false, true],
            vec![false, true, true],
            vec![true, true, false]
        ]
    };

    assert_eq!(4, game2.count_adjacent_live(CellLocation::new(1, 1)));
}

#[test]
fn game_should_kill_a_cell_with_fewer_than_2_adjacent_live_neighbors() {
    let mut game = Game{
        matrix: vec![
            vec![false, false, false],
            vec![false, true, false],
            vec![false, true, false]
        ]
    };
    game = game.update();

    // All cells should now be dead
    for row in game.matrix {
        for cell in row {
            assert!(!cell);
        }
    }
}

#[test]
fn game_should_keep_cells_with_two_or_three_live_neighbors() {
    let mut game = Game{
        matrix: vec![
            vec![true, false, false],
            vec![true, true, false],
            vec![false, true, false]
        ]
    };
    game = game.update();
    assert!(game.is_alive(CellLocation::new(0, 0)));
    assert!(game.is_alive(CellLocation::new(0, 1)));
    assert!(game.is_alive(CellLocation::new(1, 1)));
    assert!(game.is_alive(CellLocation::new(1, 2)));
}

#[test]
fn game_should_kill_cells_with_more_than_three_live_neighbors() {
    let mut game = Game{
        matrix: vec![
            vec![true, true, false],
            vec![true, true, false],
            vec![false, true, false]
        ]
    };
    game = game.update();
    assert!(game.is_alive(CellLocation::new(0, 0)));
    assert!(game.is_alive(CellLocation::new(1, 0)));
    assert!(game.is_alive(CellLocation::new(1, 2)));

    assert!(!game.is_alive(CellLocation::new(1, 1)));
}

#[test]
fn game_should_create_life_at_dead_cells_with_three_live_neighbors() {
    let mut game = Game{
        matrix: vec![
            vec![false, true, false],
            vec![true, false, false],
            vec![false, true, false]
        ]
    };
    game = game.update();
    assert!(game.is_alive(CellLocation::new(1, 1)));
}

