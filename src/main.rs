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
    MATRIX
}

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

    let mut gl = GlGraphics::new(opengl);
    let font_path = PathBuf::from("/System/Library/Fonts/Palatino.ttc");
    let glyph_cache: GlyphCache = GlyphCache::new(&font_path).unwrap();
    let ui = &mut Ui::new(glyph_cache, theme);
    let event_iter = window.events().ups(1).max_fps(60);

    let mut game = Game::new(20);

    for event in event_iter {
        if let Event::Update(update_args) = event {
            game = game.update();
        }

        ui.handle_event(&event);

        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |graphics_context, gl| {
                draw_ui(ui, &game);
                ui.draw_if_changed(graphics_context, gl);
            });
        }

    }

}

fn draw_ui<C: CharacterCache>(ui: &mut Ui<C>, game: &Game) {
    let game_size = game.size();
    let matrix_size = ui.win_h.min(ui.win_w) - 10.0;
    Background::new().rgb(0.5, 0.5, 0.5).set(ui);

    WidgetMatrix::new(game_size, game_size)
        .dimensions(matrix_size, matrix_size)
        .each_widget(|n: usize, col: usize, row: usize| {
            // println!("col: {}, row: {}, n: {}", col, row, n);
            let alive = false; //game.is_alive(CellLocation::new(row, col));
            //let (r, g, b, a): (f32, f32, f32, f32) = if alive {
            //    (0f32, 0f32, 0f32, 1f32)
            //} else {
            //    (1f32, 1f32, 1f32, 1f32)
            //};
            let (r, g, b, a) = (0.5f32, 0.5f32, 0.5f32, 0.5f32);
            
            Toggle::new(alive)
                .react(|state: bool| { })
                .enabled(false)
                .rgba(r, g, b, a)
                
        }).set(MATRIX, ui);

}







