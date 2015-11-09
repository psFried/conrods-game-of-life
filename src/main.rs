extern crate conrod;
extern crate piston_window;

use std::path::{Path, PathBuf};

use self::glutin_window::GlutinWindow;
use self::opengl_graphics::{GlGraphics, OpenGL};
use self::opengl_graphics::glyph_cache::GlyphCache;
use self::piston::event_loop::{Events, EventLoop};
use self::piston::input::{RenderEvent};
use self::piston::window::{WindowSettings, Size};

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
    let event_iter = window.events().ups(3).max_fps(1);

    for event in event_iter {
        println!("{:?}");

    }

}
