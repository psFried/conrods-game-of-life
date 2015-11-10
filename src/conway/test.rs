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

    let cell = Cell::new(0, 0);
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

    assert_eq!(8, game.count_adjacent_live(Cell::new(1, 1)));
    assert_eq!(3, game.count_adjacent_live(Cell::new(0, 0)));
    assert_eq!(5, game.count_adjacent_live(Cell::new(2, 1)));

    let game2 = Game{
        matrix: vec![
            vec![false, false, true],
            vec![false, true, true],
            vec![true, true, false]
        ]
    };

    assert_eq!(4, game2.count_adjacent_live(Cell::new(1, 1)));
}

