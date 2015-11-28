use super::*;

#[test]
fn game_should_be_resized_down() {
    let mut game = Game{
        matrix: vec![
            vec![false, false, false, true],
            vec![false, false, false, true],
            vec![false, false, false, true],
            vec![true, true, true, true]
        ]
    };

    game = game.resize(3, 3);

    assert_eq!(3, game.height());
    assert_eq!(3, game.width());
    for i in 0..3 {
        assert!(!game.is_alive(CellLocation::new(i, 2)));
    }
    for i in 0..3 {
        assert!(!game.is_alive(CellLocation::new(2, i)));
    }
}


#[test]
fn game_should_be_resized_up() {
    let mut game = Game{
        matrix: vec![
            vec![false, false, false, true],
            vec![false, false, false, true],
            vec![false, false, false, true],
            vec![true, true, true, true]
        ]
    };

    game = game.resize(5, 5);

    assert_eq!(5, game.height());
    assert_eq!(5, game.width());
    for i in 0..5 {
        assert!(!game.is_alive(CellLocation::new(i, 4)));
    }
    for i in 0..5 {
        assert!(!game.is_alive(CellLocation::new(4, i)));
    }
}

#[test]
fn adjacent_cells_should_be_returned_in_a_vec() {
    let mut game = Game{
        matrix: vec![
            vec![true, false, false, false],
            vec![true, false, false, false],
            vec![true, false, false, false],
            vec![true, false, false, false]
        ]
    };
    let adj_cells: Vec<CellLocation> = game.adjacent_cells(CellLocation::new(3, 3));

    assert_eq!(9, adj_cells.len());
    
    let expected_cells = vec![
        (2, 2),
        (3, 2),
        (0, 2),
        (2, 3),
        (3, 3),
        (0, 3),
        (2, 0),
        (3, 0),
        (0, 0)
    ].iter().map(|&(x, y)| CellLocation::new(x, y)).collect::<Vec<CellLocation>>();

    assert_eq!(expected_cells, adj_cells);
}

#[test]
fn new_game_should_create_game_of_given_size() {
    let game: Game = Game::new(8, 12);

    assert_eq!(12, game.matrix.len());
    for row in game.matrix {
        assert_eq!(8, row.len());
    }
}

#[test]
fn should_be_able_to_get_and_set_cell_state() {

    let mut game = Game::new(10, 10);

    let cell = CellLocation::new(0, 0);
    assert_eq!(false, game.is_alive(cell));
    
    game.set_state(cell, true);
    assert_eq!(true, game.is_alive(cell));
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
fn game_should_wrap_around_at_edges() {
    let mut game = Game{
        matrix: vec![
            vec![true, false, false, false],
            vec![true, false, false, false],
            vec![false, false, false, false],
            vec![true, false, false, false]
        ]
    };
    game = game.update();
    assert!(game.is_alive(CellLocation::new(0, 0)));
    assert!(game.is_alive(CellLocation::new(1, 0)));
    assert!(game.is_alive(CellLocation::new(3, 0)));
    assert!(!game.is_alive(CellLocation::new(0, 1)));
    assert!(!game.is_alive(CellLocation::new(0, 3)));
}

#[test]
fn game_should_keep_cells_with_two_or_three_live_neighbors() {
    let mut game = Game{
        matrix: vec![
            vec![true, false, false, false],
            vec![true, true, false, false],
            vec![false, true, false, false],
            vec![false, false, false, false]
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

    game.locations().iter().map(|cell| assert!(!game.is_alive(*cell)));

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

