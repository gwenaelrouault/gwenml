use crate::configuration::configuration::Configuration;
use sfml::{
    graphics::{Image, IntRect, Sprite, Texture, Transformable},
    SfBox,
};
use std::collections::HashMap;

pub struct GameResources<'a> {
    pub configuration: Configuration,
    pub textures: &'a HashMap<String, SfBox<Texture>>,
    pub sprites: HashMap<String, Sprite<'a>>,
}

impl<'a> GameResources<'a> {
    pub fn new(
        configuration: Configuration,
        textures: &'a HashMap<String, SfBox<Texture>>,
    ) -> Self {
        let nmap: HashMap<String, Sprite> = HashMap::new();
        let mut res = GameResources {
            configuration,
            textures,
            sprites: nmap,
        };
        for (key, texture) in res.textures {
            let mut sprite = Sprite::new();
            sprite.set_texture(&texture, true);
            res.sprites.insert(key.clone(), sprite);
        }
        res
    }

    pub fn get_sprite(&self, name: &str) -> Sprite<'a> {
        self.sprites.get(name).unwrap().clone()
    }
}

pub fn load_textures(configuration: &Configuration) -> HashMap<String, SfBox<Texture>> {
    let mut textures = HashMap::new();
    for character in &configuration.characters {
        let mut texture = Texture::new().unwrap();
        texture
            .load_from_file(
                &character.sprite.img,
                IntRect::new(
                    0,
                    0,
                    character.sprite.size * character.sprite.nb_frames,
                    character.sprite.size,
                ),
            )
            .unwrap();
        texture.set_smooth(true);
        textures.insert(character.name.clone(), texture);
    }
    let mut texture_fonts = Texture::new().unwrap();
    texture_fonts
        .load_from_file(
            &configuration.gui.fonts.sprite.img,
            IntRect::new(
                0,
                0,
                configuration.gui.fonts.sprite.size * configuration.gui.fonts.sprite.nb_frames,
                configuration.gui.fonts.sprite.size,
            ),
        )
        .unwrap();
    textures.insert(String::from("fonts"), texture_fonts);
    let mut texture_cursor = Texture::new().unwrap();
    texture_cursor
        .load_from_file(
            &configuration.gui.cursor.sprite.img,
            IntRect::new(
                0,
                0,
                configuration.gui.cursor.sprite.size * configuration.gui.cursor.sprite.nb_frames,
                configuration.gui.cursor.sprite.size,
            ),
        )
        .unwrap();
    textures.insert(String::from("cursor"), texture_cursor);

    let background_menu = Image::from_file(configuration.textures.sprite.img.as_str()).unwrap();
    let mut texture_menu = Texture::new().unwrap();
    let menu_rect = IntRect::new(
        0,
        0,
        configuration.textures.sprite.size,
        configuration.textures.sprite.size,
    );
    texture_menu
        .load_from_image(&background_menu, menu_rect)
        .unwrap();
    texture_menu.set_smooth(true);
    texture_menu.set_repeated(true);
    textures.insert(String::from("background"), texture_menu);
    textures
}
