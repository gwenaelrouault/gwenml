use sfml::graphics::{
        Sprite, 
        Transformable, 
        RenderWindow, 
        RenderTarget,
        IntRect,
};
use sfml::SfBox;
use sfml::system::Clock;
use sfml::system::Vector2f;
use crate::common::Direction;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AnimationMode {
    OneShot,
    Repeated,
}

pub struct SpriteInfo<'a>  {
    sprite : Sprite<'a>,
    pub size : i32,
    pub index : i32,
    pub rect : IntRect,
}

pub struct AnimationInfo {
    pub clock: SfBox<Clock>,
    pub mode : AnimationMode,
    pub delay : i32,
    pub nb_frames : i32,
    pub step : i32,
    pub started : bool,
}

impl AnimationInfo  {
    pub fn reset(&mut self, delay : i32, nb_frames : i32, mode : AnimationMode) {
        self.started = false;
        self.delay = delay;
        self.nb_frames = nb_frames;
        self.mode = mode;
        self.step = 0;
    }

    pub fn next_frame(&mut self) -> bool {
        let mut is_end_animation = false;
        match self.started {
            true => {
                if self.is_animation_tick(){
                    if self.is_end_sequence() {
                        is_end_animation = self.end_sequence();
                    } 
                    else {
                         self.next_step();
                    }
                }
            }
            false => {
                self.start()
            }
        }
        is_end_animation
    }

    fn restart_clock(&mut self) {
        self.clock.restart();
    }

    fn is_animation_tick(&self) -> bool {
        self.clock.elapsed_time().as_milliseconds() >= self.delay
    }

    fn next_step(&mut self) -> i32 {
        self.step = self.step + 1;
        self.restart_clock();
        self.step       
    }

    fn start(&mut self) {
        self.started = true;
        self.restart_clock();
    }

    fn is_end_sequence(&self) -> bool {
        self.step >= self.nb_frames - 1
    }

    fn end_sequence(&mut self) -> bool {
        let mut is_end_sequence = true;
        match self.mode {
            AnimationMode::Repeated => {
                self.step = 0;
            }
            _ => {
                is_end_sequence = true;
            }
        }
        is_end_sequence
    }
}

pub struct NavInfo {
    pub position : Vector2f,
    pub speed : Vector2f,
    pub direction : Direction,
}

pub struct DisplayInfo {
    pub scale : Vector2f,
}

pub struct AnimatedSprite<'a> {
    sprite : SpriteInfo<'a>,
    display : DisplayInfo,
    animation : AnimationInfo,
    pub nav : NavInfo
}

impl<'a> AnimatedSprite<'a> {
    pub fn new(
        sprite : Sprite<'a>,
        size : i32, 
        scale : f32,
        x : f32,
        y : f32,
        x_speed : f32,
        y_speed : f32,
        direction : Direction,
        sprite_index : i32, 
        delay : i32, 
        nb_frames : i32, 
        mode : AnimationMode) -> Self {
        AnimatedSprite {
            nav : NavInfo {
                position : Vector2f::new(x, y),
                speed : Vector2f::new(x_speed, y_speed),
                direction
            },
            sprite : SpriteInfo {
                sprite,
                index : sprite_index,
                rect : IntRect::new(sprite_index * size, 0, size, size),
                size,
            },
            animation : AnimationInfo {
                clock : Clock::start(),
                delay,
                nb_frames,
                mode,
                started : false,
                step : 0,
            },
            display : DisplayInfo {
                scale : Vector2f::new(scale,scale),
            }
        }
    }

    pub fn start_animation(&mut self, 
        sprite_index : i32, 
        delay : i32, 
        nb_frames : i32, 
        mode : AnimationMode, 
        direction : Direction,
        speed : f32,
        window: &mut RenderWindow) {
        self.nav.speed.x = if direction == Direction::Right {speed} else {-1. * speed};
        self.nav.direction = direction;
        self.sprite.index = sprite_index;
        self.animation.reset(delay, nb_frames, mode);
        self.update_position();
        window.draw(&self.sprite.sprite);
    }

    pub fn next_frame(&mut self, window: &mut RenderWindow) -> bool {
        //println!("STEP {} / INDEX : {} / SIZE : {}", self.animation.step, self.sprite.index, self.sprite.size);
        let is_closed_current_action = self.animation.next_frame();
        self.sprite.rect.left = (self.sprite.index + self.animation.step) * self.sprite.size;
        //println!("RECT LEFT : {}:{}:{}:{}", self.sprite.rect.left, self.sprite.rect.top,
        //    self.sprite.rect.height, self.sprite.rect.width);
        self.sprite.sprite.set_texture_rect(self.sprite.rect);
        self.update_position();
        window.draw(&self.sprite.sprite);
        is_closed_current_action
    }

    fn update_position(&mut self) {
        self.nav.position.x = self.nav.position.x + self.nav.speed.x;
        self.nav.position.y = self.nav.position.y + self.nav.speed.y;
        self.sprite.sprite.set_position(self.nav.position);
        let x_scale = if self.nav.direction == Direction::Left { -1. * self.display.scale.x } else { self.display.scale.x };
        self.sprite.sprite.set_scale(Vector2f::new(x_scale, self.display.scale.y));
    }
}