use player::{InputState, Player};
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

struct SpriteConfiguration {
    nb_frames : i32,
    size : i32,
    scale : f32,
    x_center : f32,
    y_center : f32,
}

fn main() {
    
    let screen_conf = ScreenConfiguration {
        view_size : Vector2f::new(800., 600.),
        view_center : Vector2f::new(400., 300.),
        ratio : 2.7,
        aa_level : 0,
        width : 800,
        height : 600,
    };

    let sprite_conf = SpriteConfiguration {
        nb_frames : 32,
        size : 100,
        scale : 0.75,
        x_center : 50.,
        y_center : 50.
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
    texture.set_smooth(true);
    let mut arena_sprite = Sprite::new();
    arena_sprite.set_texture(&texture, true);
    let arena = arena::Arena::new(arena_sprite);
    let mut texture_player = Texture::new().unwrap();
    let current_player_sprite_rect = IntRect::new(0, 0, sprite_conf.size, sprite_conf.size);
    texture_player
        .load_from_file(
            "resources/spriteHero.png",
            IntRect::new(0, 0, sprite_conf.size * sprite_conf.nb_frames, sprite_conf.size),
        )
        .unwrap();
    texture_player.set_smooth(true);
    let mut player_sprite = Sprite::new();
    player_sprite.set_texture(&texture_player, true);
    player_sprite.set_texture_rect(current_player_sprite_rect);
    player_sprite.set_scale(Vector2f::new(sprite_conf.scale, sprite_conf.scale));
    player_sprite.set_origin(Vector2f::new(sprite_conf.x_center, sprite_conf.y_center));
    
    let player = player::Player {
        position : Vector2f::new(120.,150.),
        speed : Vector2f::new(0.,0.),
        sprite : player_sprite,
        state : player::State::default(),
        input_state : InputState::new(),
        actions : Vec::new(),
        clock : Clock::start(),
        running_action : player::RunAction::Standing,
        running_direction : player::Direction::Right,
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

    let timer = Clock::start();
    let mut engine = engine::MauriceFight2dEngine {
        window,
        view,
        arena,
        player,
        timer,
    };
    loop {
        while let Some(event) = engine.window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                _ => {
                    engine.process_input_event(event);
                }
            }
        }
        engine.render_frame();
    }
}
