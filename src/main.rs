#[macro_use]
extern crate conrod;
extern crate piston_window;
extern crate opengl_graphics;
extern crate piston;
extern crate input;
extern crate glutin_window;

mod conway;
use conway::{CellLocation, Game};
use std::path::{Path, PathBuf};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event_loop::{self, Events, EventLoop};
use piston::input::{RenderEvent, Event};
use piston::window::{WindowSettings, Size};

use self::conrod::{
    Background,
    Button,
    color,
    Colorable,
    CharacterCache,
    Labelable,
    Sizeable,
    Theme,
    Ui,
    Widget,
    Positionable,
    WidgetId,
    WidgetMatrix,
    Toggle
};

struct GameDimensions {
    game_w: usize,
    game_h: usize,
    matrix_w: f64,
    matrix_h: f64
}

widget_ids!{
    MATRIX,
    PLAY_BUTTON
}

const SQUARE_SIZE: f64 = 15.0;
const PLAY_BUTTON_SPACE: f64 = 90.0;
const MATRIX_PADDING: f64 = 10.0;
const OPENGL: OpenGL = OpenGL::V3_2;

fn main() {

    let theme = Theme::default();

    let initial_win_w: u32 = 400;
    let initial_win_h: u32 = 650;

    let window: GlutinWindow = create_window(initial_win_w , initial_win_h);
    let mut gl = GlGraphics::new(OPENGL);
    let font_path = PathBuf::from("/System/Library/Fonts/Palatino.ttc");
    let glyph_cache: GlyphCache = GlyphCache::new(&font_path).unwrap();
    let ui = &mut Ui::new(glyph_cache, theme);

    let mut game = {
        let initial_matrix_dimensions = get_max_matrix_dimensions(initial_win_w as f64, initial_win_h as f64);
        let initial_game_dimensions = get_game_dimensions(initial_matrix_dimensions.0, initial_matrix_dimensions.1, SQUARE_SIZE);
        Game::new(initial_game_dimensions.game_w, initial_game_dimensions.game_h)
    };

    let mut playing = false;

    for event in window.events().ups(3).max_fps(30) {
        ui.handle_event(&event);

        match event {
            Event::Update(_) if playing => {game = game.update()},
            Event::Render(render_args) => {
                gl.draw(render_args.viewport(), |graphics_context, gl| {
                    draw_ui(ui, &mut game, &mut playing);
                    ui.draw_if_changed(graphics_context, gl);
                });
            },
            _ => {}
        }
    }
}

fn create_window(width: u32, height: u32) -> GlutinWindow {
    WindowSettings::new(
        "Conrod's Game of Life".to_string(),
        Size { width: width, height: height }
    ).opengl(OPENGL)
    .exit_on_esc(true)
    .samples(4)
    .build()
    .unwrap()
}


fn draw_ui<C: CharacterCache>(ui: &mut Ui<C>, game: &mut Game, started: &mut bool) {
    Background::new().rgb(0.8, 0.45, 0.6).set(ui);

    let (max_matrix_w, max_matrix_h) = get_max_matrix_dimensions(ui.win_w, ui.win_h);
    let game_dimensions = get_game_dimensions(max_matrix_w, max_matrix_h, SQUARE_SIZE);
    if game_dimensions.game_w != game.width() || game_dimensions.game_h != game.height() {
        println!("resizing game to x: {}, y: {}, from x: {}, y: {}", game_dimensions.game_w, game_dimensions.game_h, game.width(), game.height());
        *game = game.resize(game_dimensions.game_w, game_dimensions.game_h);
    }

    create_cell_matrix(ui, game, *started, game_dimensions.matrix_w, game_dimensions.matrix_h);
    create_start_button(ui, started);
}

fn create_cell_matrix<C: CharacterCache>(ui: &mut Ui<C>, game: &mut Game, started: bool, matrix_w: f64, matrix_h: f64) {
    let game_w = game.width();
    let game_h = game.height();
    let matrix_pos_y = (ui.win_h - matrix_h) / 2.0 - MATRIX_PADDING;
    WidgetMatrix::new(game_w, game_h)
        .dimensions(matrix_w, matrix_h)
        .xy(0.0, matrix_pos_y)
        .each_widget(|_n: usize, col: usize, row: usize| {
            let alive = game.is_alive(CellLocation::new(col, row));
            let cell_color: color::Color = if alive {
                color::red()
            } else {
                color::grey()
            };

            let game_ref: &mut Game = game;
            Toggle::new(alive)
                .react(move |state: bool| {
                    game_ref.set_state(CellLocation::new(col, row), state);
                }).enabled(!started)
                .color(cell_color)
        }).set(MATRIX, ui);
}

fn get_max_matrix_dimensions(win_w: f64, win_h: f64) -> (f64, f64) {
    let matrix_w = win_w - MATRIX_PADDING;
    let matrix_h = win_h - PLAY_BUTTON_SPACE - MATRIX_PADDING;
    (matrix_w, matrix_h)
}

fn get_game_dimensions(max_matrix_w: f64, max_matrix_h: f64, square_size: f64) -> GameDimensions {
    let game_w: usize = (max_matrix_w / square_size) as usize;
    let game_h: usize = (max_matrix_h / square_size) as usize;

    GameDimensions{
        game_w: game_w,
        game_h: game_h,
        matrix_w: game_w as f64 * square_size,
        matrix_h: game_h as f64 * square_size
    }
}

#[test]
fn get_game_dimensions_should_return_largest_possible_game_for_window_size() {
    let result: GameDimensions = get_game_dimensions(100.0, 150.0, 25.0);
    assert_eq!(4, result.game_w);
    assert_eq!(6, result.game_h);
    
    let result2 = get_game_dimensions(105.3, 224.5, 25.0);
    assert_eq!(4, result2.game_w);
    assert_eq!(8, result2.game_h);
    assert_eq!(100.0, result2.matrix_w);
    assert_eq!(200.0, result2.matrix_h);
}

fn create_start_button<C: CharacterCache>(ui: &mut Ui<C>, started: &mut bool) {
    let (button_label, button_color): (&str, color::Color) = if *started {
        ("Stop", color::red())
    } else {
        ("Start", color::green())
    };

    Button::new()
        .label(button_label)
        .color(button_color)
        .react(|| {
            *started = !*started;
        })
        .down_from(MATRIX, MATRIX_PADDING)
        .align_middle_x()
        .set(PLAY_BUTTON, ui);
}

