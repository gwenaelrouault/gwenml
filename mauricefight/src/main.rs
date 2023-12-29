use animated_sprite::AnimationMode;
use fighter_common::Fighter;
use menu::Menu;
use std::fs::File;
use 
    sfml::{
        graphics::{
            RenderTarget, RenderWindow, Texture,Sprite,IntRect,View,FloatRect,Image,
            Transformable,
        },
        system::Vector2f,
        window::{ContextSettings, Style}
};

mod configuration;
mod inputs;
mod arena;
mod fighter_common;
mod fighter_maurice;
mod engine;
mod menu;
mod common;
mod animated_sprite;

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
    let configuration: configuration::Configuration = serde_json::from_reader(file).unwrap();
    let screen_configuration = ScreenConfiguration::new();

    // load sprite fonts...
    let texture_fonts = common::load_texture(
        &configuration.gui.fonts.sprite.img.as_str(), 
        configuration.gui.fonts.sprite.size, configuration.gui.fonts.sprite.nb_frames, 
        true, false).unwrap();
    let mut sprite_fonts = Sprite::new();
    sprite_fonts.set_texture(&texture_fonts, true);
    common::load_sprite(&mut sprite_fonts, configuration.gui.fonts.sprite.size);

    // load sprite cursor...
    let texture_cursor = common::load_texture(
        configuration.gui.cursor.sprite.img.as_str(), 
        configuration.gui.cursor.sprite.size, configuration.gui.cursor.sprite.nb_frames,
        true, false).unwrap();
    let mut sprite_cursor = Sprite::new();
    sprite_cursor.set_texture(&texture_cursor, true);
    common::load_sprite(&mut sprite_cursor, configuration.gui.cursor.sprite.size);

    // load textures...
    let background_menu = Image::from_file(configuration.textures.sprite.img.as_str()).unwrap();
    let mut texture_menu = Texture::new().unwrap();
    let menu_rect = IntRect::new(0,0, configuration.textures.sprite.size,configuration.textures.sprite.size);
    texture_menu.load_from_image(&background_menu, menu_rect).unwrap();
    texture_menu.set_smooth(true);
    texture_menu.set_repeated(true);
    let mut menu_sprite = Sprite::new();
    menu_sprite.set_texture_rect(IntRect::new(0,0, screen_configuration.width.try_into().unwrap(), screen_configuration.height.try_into().unwrap()));
    menu_sprite.set_texture(&texture_menu, false);

    // load levels sprites
    let background_arena = Image::from_file(&configuration.levels[0].sprite.img).unwrap();
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

    // load fighters sprites
    let mut texture_player : sfml::SfBox<Texture>  = Texture::new().unwrap();
    let current_player_sprite_rect = IntRect::new(
        0, 
        0, 
        configuration.characters[0].sprite.size, 
        configuration.characters[0].sprite.size);
    texture_player
        .load_from_file(
            &configuration.characters[0].sprite.img,
            IntRect::new(
                0, 
                0, 
                configuration.characters[0].sprite.size * configuration.characters[0].sprite.nb_frames, 
                configuration.characters[0].sprite.size),
        )
        .unwrap();
    texture_player.set_smooth(true);
    let mut player_sprite = Sprite::new();
    player_sprite.set_texture(&texture_player, true);
    player_sprite.set_texture_rect(current_player_sprite_rect);
    player_sprite.set_scale(Vector2f::new(configuration.characters[0].sprite.display.scale, configuration.characters[0].sprite.display.scale));
    player_sprite.set_origin(Vector2f::new(configuration.characters[0].sprite.display.x_origin, configuration.characters[0].sprite.display.y_origin));
    
     
    // load modules
    let menu = Menu::new(menu_sprite, sprite_fonts, sprite_cursor, &configuration);
    let arena = arena::Arena::new(arena_sprite);
    let maurice_fighter = Box::new(fighter_maurice::Maurice::new(
        &configuration.characters[0],
        player_sprite,
        120.0, 
        150.0, 
        "idle",
        AnimationMode::Repeated,
        true,
    ));

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

    // load fighters
    let mut fighters : Vec<Box<dyn Fighter>> = Vec::new();
    fighters.push(maurice_fighter);
    let mut engine = engine::MauriceFight2dEngine::new(window, view, arena, fighters, "maurice", menu, configuration);

    // GAME LOOP
    loop {
        while let Some(event) = engine.window.poll_event() {
            if engine.process_input_event(event) {
                return;
            }
        }
        engine.render_frame();
    }
}
