use sfml::{
    graphics::{
        CircleShape, Color, FloatRect, Font, Image, IntRect, RectangleShape, RenderTarget,
        RenderWindow, Sprite, Text, Texture, Transformable, View,
    },
    system::Vector2f,
    SfBox,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}
pub fn load_texture(
    img: &str,
    size_frame: i32,
    nb_frames: i32,
    smooth: bool,
    repeated: bool) -> Option<SfBox<Texture>> {
    let mut texture = Texture::new()?;
    texture
        .load_from_file(img, IntRect::new(0, 0, size_frame * nb_frames, size_frame))
        .unwrap();
    texture.set_smooth(smooth);
    texture.set_repeated(repeated);
    Option::from(texture)
}

pub fn load_sprite(sprite : &mut Sprite, size : i32) {
    sprite.set_texture_rect(IntRect::new(0, 0, size, size));
    sprite.set_origin(Vector2f::new(size as f32 / 2.,size as f32 / 2.));
}