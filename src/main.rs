extern crate conrod;
extern crate piston_window;
extern crate opengl_graphics;
extern crate piston;
extern crate input;
extern crate glutin_window;

mod conway;

use conway::Game;
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
    WidgetId
};

fn main() {
    let opengl = OpenGL::V3_2;
    let window: GlutinWindow = WindowSettings::new(
        "Conrod's Game of Life".to_string(),
        Size { width: 800, height: 500 }
    ).opengl(opengl)
    .exit_on_esc(true)
    .samples(4)
    .build()
    .unwrap();

    let theme = Theme::default();

    // let mut gl = GlGraphics::new(opengl);
    let font_path = PathBuf::from("/System/Library/Fonts/Palatino.ttc");
    let glyph_cache: GlyphCache = GlyphCache::new(&font_path).unwrap();
    let ui = &mut Ui::new(glyph_cache, theme);
    let event_iter = window.events().ups(3).max_fps(60);

    let mut game = Game::new(100);

    for event in event_iter {
        if let Event::Update(update_args) = event {
            game = game.update();
        }

        ui.handle_event(&event);

        draw_ui(ui, &game);

    }

}

fn draw_ui<C: CharacterCache>(ui: &mut Ui<C>, game: &Game) {
    let square_size = get_square_size(ui.win_h, ui.win_w, game.size());

}

fn get_square_size(win_h: f64, win_w: f64, game_size: usize) -> f64 {
    (std::cmp::min(win_h, win_w) - 10.0) / game_size as f64
}








