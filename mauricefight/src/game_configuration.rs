use sfml::system::Vector2f;

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
        Self {
            view_size: Vector2f::new(800., 600.),
            view_center: Vector2f::new(400., 300.),
            ratio: 2.7,
            aa_level: 0,
            width: 800,
            height: 600,
        }
    }
}

pub struct SpriteConfiguration {
    pub nb_frames: i32,
    pub size: i32,
    pub scale: f32,
    pub x_center: f32,
    pub y_center: f32,
}

impl SpriteConfiguration {
    pub fn new() -> Self {
        Self {
            nb_frames: 39,
            size: 100,
            scale: 0.75,
            x_center: 50.,
            y_center: 50.,
        }
    }
}

pub struct GameConfiguration {
    pub sprite: SpriteConfiguration,
    pub screen: ScreenConfiguration,
}

impl GameConfiguration {
    pub fn new() -> Self {
        Self {
            sprite: SpriteConfiguration::new(),
            screen: ScreenConfiguration::new(),
        }
    }
}
