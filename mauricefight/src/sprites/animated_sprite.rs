use crate::common::Direction;
use crate::configuration::resources::GameResources;
use sfml::graphics::{IntRect, RenderTarget, RenderWindow, Sprite, Transformable};
use sfml::system::Vector2f;
use sfml::system::{Clock, Vector2};
use sfml::SfBox;

pub struct SpriteInfo<'a> {
    pub sprite: Sprite<'a>,
    pub size: i32,
    pub index: i32,
    pub rect: IntRect,
}

pub struct AnimationInfo {
    pub clock: SfBox<Clock>,
    pub delay: i32,
    pub nb_frames: i32,
    pub step: i32,
    pub started: bool,
}

impl AnimationInfo {
    pub fn reset(&mut self, delay: i32, nb_frames: i32) {
        self.started = false;
        self.delay = delay;
        self.nb_frames = nb_frames;
        self.step = 0;
    }

    pub fn next_frame(&mut self) -> bool {
        let mut is_end_animation = false;
        match self.started {
            true => {
                if self.is_animation_tick() {
                    is_end_animation = self.is_end_sequence();
                    if !is_end_animation {
                        self.next_step();
                    }
                }
            }
            false => {
                self.start();
            }
        }
        is_end_animation
    }

    fn is_animation_tick(&self) -> bool {
        self.clock.elapsed_time().as_milliseconds() >= self.delay
    }

    fn next_step(&mut self) -> i32 {
        self.step = self.step + 1;
        //println!("next step {}", self.step);
        self.clock.restart();
        self.step
    }

    fn start(&mut self) -> i32 {
        self.started = true;
        self.step = 0;
        self.clock.restart();
        self.step
    }

    fn is_end_sequence(&self) -> bool {
        self.step >= self.nb_frames - 1
    }
}

pub struct NavInfo {
    pub position: Vector2f,
    pub speed: Vector2f,
}

pub struct DisplayInfo {
    pub scale: Vector2f,
}

pub struct AnimatedSprite<'a> {
    pub sprite: SpriteInfo<'a>,
    pub display: DisplayInfo,
    pub animation: AnimationInfo,
    pub nav: NavInfo,
}

impl<'a> AnimatedSprite<'a> {
    pub fn new(
        resources: &'a GameResources,
        sprite: &str,
        size: i32,
        scale: f32,
        x: f32,
        y: f32,
        x_speed: f32,
        y_speed: f32,
        direction: Direction,
        sprite_index: i32,
        delay: i32,
        nb_frames: i32,
    ) -> Self {
        let mut _sprite = resources.get_sprite(sprite);
        _sprite.set_origin(Vector2f::new(size as f32 / 2., size as f32 / 2.));
        AnimatedSprite {
            nav: NavInfo {
                position: Vector2f::new(x, y),
                speed: Vector2f::new(x_speed, y_speed),
            },
            sprite: SpriteInfo {
                sprite: _sprite,
                index: sprite_index,
                rect: IntRect::new(sprite_index * size, 0, size, size),
                size,
            },
            animation: AnimationInfo {
                clock: Clock::start(),
                delay,
                nb_frames,
                started: false,
                step: 0,
            },
            display: DisplayInfo {
                scale: Vector2f::new(scale, scale),
            },
        }
    }

    pub fn restart_animation(&mut self) {
        self.animation.step = 0;
    }

    pub fn reset_animation(
        &mut self,
        sprite_index: i32,
        delay: i32,
        nb_frames: i32,
        direction: Direction,
        speed: f32,
    ) {
        self.nav.speed.x = if direction == Direction::Right {
            speed
        } else {
            -1. * speed
        };
        self.sprite.index = sprite_index;
        self.animation.reset(delay, nb_frames);
    }

    pub fn next_frame(&mut self, direction: Direction, window: &mut RenderWindow) -> (bool, i32) {
        //println!(
        //    "STEP {} / INDEX : {} / SIZE : {} / SPEED : {}",
        //    self.animation.step, self.sprite.index, self.sprite.size, self.nav.speed.x
        //);
        let is_closed_current_action = self.animation.next_frame();
        self.sprite.rect.left = (self.sprite.index + self.animation.step) * self.sprite.size;
        //println!(
        //    "POINT {}",
        //    self.sprite.rect.left
        //);
        self.sprite.sprite.set_texture_rect(self.sprite.rect);
        self.update_position(direction);
        window.draw(&self.sprite.sprite);
        (is_closed_current_action, self.animation.step)
    }

    fn update_position(&mut self, direction: Direction) {
        self.nav.position.x = self.nav.position.x + self.nav.speed.x;
        self.nav.position.y = self.nav.position.y + self.nav.speed.y;
        self.sprite.sprite.set_position(self.nav.position);
        let x_scale = if direction == Direction::Left {
            -1. * self.display.scale.x
        } else {
            self.display.scale.x
        };
        self.sprite
            .sprite
            .set_scale(Vector2f::new(x_scale, self.display.scale.y));
    }
}
