use sfml::graphics::Sprite;
use sfml::SfBox;
use sfml::system::Clock;

pub struct AnimatedSprite<'a> {
    pub sprite : Sprite<'a>,
    pub step: i32,
    pub clock: SfBox<Clock>,
    pub delay : i32,
    pub repeated : bool,
}

impl<'a> AnimatedSprite<'a> {
    pub fn new(sprite : Sprite<'a>, delay : i32, repeated : bool) -> Self {
        AnimatedSprite {
            sprite,
            step : 0,
            clock : Clock::start(),
            delay,
            repeated,
        }
    }
}