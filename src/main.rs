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
use piston::event_loop::{Events, EventLoop};
use piston::input::{RenderEvent, Event};
use piston::window::{WindowSettings, Size};

use self::conrod::{
    Background,
    Button,
    color,
    Colorable,
    CharacterCache,
    DropDownList,
    Labelable,
    Label,
    Sizeable,
    Theme,
    Ui,
    Widget,
    NumberDialer,
    Frameable,
    Positionable,
    TextBox,
    WidgetIndex,
    WidgetId,
    WidgetMatrix,
    Toggle
};

widget_ids!{
    MATRIX,
    PLAY_BUTTON
}


fn main() {
    let opengl = OpenGL::V3_2;
    let window: GlutinWindow = WindowSettings::new(
        "Conrod's Game of Life".to_string(),
        Size { width: 500, height: 650 }
    ).opengl(opengl)
    .exit_on_esc(true)
    .samples(4)
    .build()
    .unwrap();

    let theme = Theme::default();

    let mut gl = GlGraphics::new(opengl);
    let font_path = PathBuf::from("/System/Library/Fonts/Palatino.ttc");
    let glyph_cache: GlyphCache = GlyphCache::new(&font_path).unwrap();
    let ui = &mut Ui::new(glyph_cache, theme);
    let event_iter = window.events().ups(1).max_fps(30);

    let mut game = Game::new(40);
    let mut playing = false;

    for event in event_iter {
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

const PLAY_BUTTON_SPACE: f64 = 90.0;
const MATRIX_PADDING: f64 = 10.0;

fn draw_ui<C: CharacterCache>(ui: &mut Ui<C>, game: &mut Game, started: &mut bool) {
    Background::new().rgb(0.8, 0.45, 0.6).set(ui);

    let game_size = game.size();
    let matrix_size = ui.win_w.min(ui.win_h - PLAY_BUTTON_SPACE) - MATRIX_PADDING;
    let matrix_pos_y = (ui.win_h - matrix_size) / 2.0 - MATRIX_PADDING;
    WidgetMatrix::new(game_size, game_size)
        .dimensions(matrix_size, matrix_size)
        .xy(0.0, matrix_pos_y)
        .each_widget(|_n: usize, col: usize, row: usize| {
            let alive = game.is_alive(CellLocation::new(row, col));
            let cell_color: color::Color = if alive {
                color::red()
            } else {
                color::grey()
            };
            
            Toggle::new(alive)
                .react(|state: bool| { 
                    println!("Toggle clicked: row: {}, col: {}", row, col);
                    game.set_state(CellLocation::new(row, col), state);
                }).enabled(!*started)
                .color(cell_color)
                
        }).set(MATRIX, ui);

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







