use game_configuration::GameConfiguration;
use menu::Menu;
use std::collections::VecDeque;
use game_inputs::InputState;
use 
    sfml::{
        graphics::{
            RenderTarget, RenderWindow, Texture,Sprite,IntRect,View,FloatRect,Image,
            Transformable,
        },
        system::{Clock, Vector2f},
        window::{ContextSettings, Event,  Key, Style}
};

mod arena;
mod player;
mod engine;
mod menu;
mod game_configuration;
mod game_events;
mod game_common;
mod game_inputs;
mod animated_sprite;

fn main() {    
    let configuration = GameConfiguration::new();
    let texture_letters = game_common::load_texture(
        "resources/spriteLetters.png", 
        configuration.texture_pack.size_letter, 
        configuration.texture_pack.nb_frames_letters, true, false).unwrap();
    let mut letters_sprite = Sprite::new();
    letters_sprite.set_texture(&texture_letters, true);
    letters_sprite.set_texture_rect(IntRect::new(0, 0, configuration.texture_pack.size_letter, configuration.texture_pack.size_letter));

    let texture_skull = game_common::load_texture(
        "resources/spriteSkull.png", 
        configuration.texture_pack.size_skull, 
        configuration.texture_pack.nb_frames_skull, true, false).unwrap();
    let mut skull_sprite = Sprite::new();
    skull_sprite.set_texture(&texture_skull, true);
    skull_sprite.set_texture_rect(IntRect::new(0, 0, configuration.texture_pack.size_skull, configuration.texture_pack.size_skull));
    skull_sprite.set_scale(Vector2f::new(0.4, 0.4));

    let background_menu = Image::from_file("resources/textures.png").unwrap();
    let mut texture_menu = Texture::new().unwrap();
    let menu_rect = IntRect::new(0,0, configuration.texture_pack.size,configuration.texture_pack.size);
    texture_menu.load_from_image(&background_menu, menu_rect).unwrap();
    texture_menu.set_smooth(true);
    texture_menu.set_repeated(true);
    let mut menu_sprite = Sprite::new();
    menu_sprite.set_texture_rect(IntRect::new(0,0, configuration.screen.width.try_into().unwrap(), configuration.screen.height.try_into().unwrap()));
    menu_sprite.set_texture(&texture_menu, false);
    let menu = Menu::new(menu_sprite, letters_sprite, skull_sprite, &configuration);

    let background_arena = Image::from_file("resources/ARENA1.png").unwrap();
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
    let current_player_sprite_rect = IntRect::new(0, 0, configuration.sprite.size, configuration.sprite.size);
    texture_player
        .load_from_file(
            "resources/spriteHero1.png",
            IntRect::new(0, 0, configuration.sprite.size * configuration.sprite.nb_frames, configuration.sprite.size),
        )
        .unwrap();
    texture_player.set_smooth(true);
    let mut player_sprite = Sprite::new();
    player_sprite.set_texture(&texture_player, true);
    player_sprite.set_texture_rect(current_player_sprite_rect);
    player_sprite.set_scale(Vector2f::new(configuration.sprite.scale, configuration.sprite.scale));
    player_sprite.set_origin(Vector2f::new(configuration.sprite.x_center, configuration.sprite.y_center));
    
    let player = player::Player {
        position : Vector2f::new(120.,150.),
        speed : Vector2f::new(0.,0.),
        sprite : player_sprite,
        state : player::State::default(),
        input_state : InputState::new(),
        actions : VecDeque::new(),
        clock : Clock::start(),
        running_action : game_events::RunAction::Standing,
        running_direction : game_common::Direction::Right,
        ko : true,
    };
    
    let context_settings = ContextSettings {
        antialiasing_level: configuration.screen.aa_level,
        ..Default::default()
    };
    let mut window = RenderWindow::new(
        (configuration.screen.width, configuration.screen.height),
        "Maurice 2D",
        Style::CLOSE,
        &context_settings,
    );
    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);
    let mut view = View::new(configuration.screen.view_center, configuration.screen.view_size);
    view.set_viewport(FloatRect::new(0., 0., configuration.screen.ratio, configuration.screen.ratio));
    window.set_view(&view);

    let mut engine = engine::MauriceFight2dEngine::new(window, view, arena, player, menu, configuration);

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
