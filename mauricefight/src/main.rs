use configuration::resources;
use sfml::SfBox;
use std::{fs::File, borrow::Borrow};
use std::collections::HashMap;
use 
    sfml::{
        graphics::{
            RenderTarget, RenderWindow, Texture,Sprite,IntRect,View,FloatRect,Image,
            Transformable,
        },
        system::Vector2f,
        window::{ContextSettings, Style},
};

mod configuration {
    pub mod configuration;
    pub mod resources;
}
mod arena;
mod engine;
mod gui {
    pub mod menu;
}
mod common;
mod sprites {
    pub  mod animated_sprite;
}
mod fighters {
    pub mod fighter;
}

pub struct ScreenConfiguration {
    pub view_size: Vector2f,
    pub view_center: Vector2f,
    pub ratio: f32,
    pub aa_level: u32,
    pub width: u32,
    pub height: u32,
}

impl ScreenConfiguration {
    pub fn new() -> Self {
        ScreenConfiguration {
            view_size: Vector2f::new(800., 600.),
            view_center: Vector2f::new(400., 300.),
            ratio: 2.7,
            aa_level: 0,
            width: 800,
            height: 600,
        }
    }
}


fn main() {
    // load configuration
    let file = File::open("resources/configuration.json").unwrap();
    let configuration: configuration::configuration::Configuration = serde_json::from_reader(file).unwrap();
    let screen_configuration = ScreenConfiguration::new();
    let textures = resources::load_textures(&configuration);
    let game_resources = resources::GameResources::new(configuration, &textures);

    // load levels sprites
    let background_arena = Image::from_file(&game_resources.configuration.levels[0].sprite.img).unwrap();
    let mut texture = Texture::new().unwrap();
    texture
        .load_from_image(
            &background_arena,
            IntRect::new(
                0,
                0,
                background_arena.size().x.try_into().unwrap(),
                background_arena.size().y.try_into().unwrap(),
            ),
        )
        .unwrap();
    texture.set_smooth(true);
    let mut arena_sprite = Sprite::new();
    arena_sprite.set_texture(&texture, true);

     
    // load modules
    let arena = arena::Arena::new(arena_sprite);
    
    // create window
    let context_settings = ContextSettings {
        antialiasing_level: screen_configuration.aa_level,
        ..Default::default()
    };
    let mut window = RenderWindow::new(
        (screen_configuration.width, screen_configuration.height),
        "Maurice 2D",
        Style::CLOSE,
        &context_settings,
    );
    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);
    let mut view = View::new(screen_configuration.view_center, screen_configuration.view_size);
    view.set_viewport(FloatRect::new(0., 0., screen_configuration.ratio, screen_configuration.ratio));
    window.set_view(&view);

    let mut engine = engine::MauriceFight2dEngine::new(
        window, 
        view, 
        arena,
        &game_resources);

    // game loop
    loop {
        while let Some(event) = engine.window.poll_event() {
            if engine.process_input_event(event, &game_resources) {
                return;
            }
        }
        engine.render_frame(&game_resources);
    }
}
