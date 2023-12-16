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
    
    let screen_conf = ScreenConfiguration {
        view_size : Vector2f::new(800., 600.),
        view_center : Vector2f::new(400., 300.),
        ratio : 2.5,
        aa_level : 1,
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

    let mut texture_player = Texture::new().unwrap();
    let mut current_player_sprite_rect = IntRect::new(0, 0, 150, 150);
    texture_player
        .load_from_file(
            "resources/spriteHero1.png",
            IntRect::new(0, 0, 150 * 3, 150),
        )
        .unwrap();
    let mut player_sprite = Sprite::new();
    player_sprite.set_texture(&texture_player, true);
    player_sprite.set_texture_rect(current_player_sprite_rect);
    player_sprite.set_scale(Vector2f::new(0.7, 0.7));
    let mut player = player::Player {
        position : Vector2f::new(120.,100.),
        speed : Vector2f::new(0.1,0.),
        sprite : player_sprite,
    };
    
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
    //window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);
    let mut view = View::new(screen_conf.view_center, screen_conf.view_size);
    view.set_viewport(FloatRect::new(0., 0., screen_conf.ratio, screen_conf.ratio));
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
