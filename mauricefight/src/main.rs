use sfml::SfBox;
use 
    sfml::{
        audio::{Sound, SoundBuffer, SoundSource},
        graphics::{
            CircleShape, Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape, Text,Texture,Sprite,IntRect,View,FloatRect,Image,
            Transformable,
        },
        system::{Clock, Time, Vector2f},
        window::{ContextSettings, Event,  Key, Style}
};

mod arena;
mod player;
mod engine;

struct ScreenConfiguration {
    view_size : Vector2f,
    view_center : Vector2f,
    ratio : f32,
    aa_level : u32,
    width : u32,
    height : u32,
}

fn main() {
    let mut player = player::Player {
        speed : Vector2f::new(0.1, 0.)
    };
    let screen_conf = ScreenConfiguration {
        view_size : Vector2f::new(800., 600.),
        view_center : Vector2f::new(400., 300.),
        ratio : 2.5,
        aa_level : 0,
        width : 800,
        height : 600,
    };

    let background_arena: Image = Image::from_file("resources/ARENA1.png").unwrap();
    let x_arena = background_arena.size().x;
    let y_arena = background_arena.size().y;
    let mut texture = Texture::new().unwrap();
    texture
        .load_from_image(
            &background_arena,
            IntRect::new(
                0,
                0,
                x_arena.try_into().unwrap(),
                y_arena.try_into().unwrap(),
            ),
        )
        .unwrap();
    let mut arena_sprite = Sprite::new();
    arena_sprite.set_texture(&texture, true);
    let mut arena = arena::Arena::new(arena_sprite);

    let context_settings = ContextSettings {
        antialiasing_level: screen_conf.aa_level,
        ..Default::default()
    };
    let mut window = RenderWindow::new(
        (screen_conf.width, screen_conf.height),
        "Maurice 2D",
        Style::CLOSE,
        &context_settings,
    );
    window.set_vertical_sync_enabled(true);
    let mut view = View::new(screen_conf.view_center, screen_conf.view_size);
    view.set_viewport(FloatRect::new(0., 0., 2.5, 2.5));
    window.set_view(&view);

    let mut engine = engine::MauriceFight2dEngine {
        window,
        view,
        arena,
        player,
    };

    loop {
        while let Some(event) = engine.window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                _ => {}
            }
        }
        engine.render_frame();
    }
}
