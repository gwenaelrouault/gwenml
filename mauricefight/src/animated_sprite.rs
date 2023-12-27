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
use crate::game_common::{Direction, inverse_direction};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AnimationMode {
    OneShot,
    Repeated,
}

pub struct AnimatedSprite<'a> {
    sprite : Sprite<'a>,
    size : i32,
    pub scale : Vector2f,
    pub position : Vector2f,
    pub speed : Vector2f,
    sprite_index : i32,
    step: i32,
    clock: SfBox<Clock>,
    delay : i32,
    mode : AnimationMode,
    started : bool,
    count : i32,
    direction : Direction,
    rect : IntRect,
}

impl<'a> AnimatedSprite<'a> {
    pub fn new(
        sprite : Sprite<'a>,
        size : i32, 
        scale : Vector2f,
        position: Vector2f,
        speed : Vector2f,
        direction : Direction,
        sprite_index : i32, 
        delay : i32, 
        count : i32, 
        mode : AnimationMode) -> Self {
        AnimatedSprite {
            sprite,
            size,
            scale,
            position,
            speed,
            sprite_index,
            step : 0,
            clock : Clock::start(),
            delay,
            mode,
            started : false,
            count,
            direction,
            rect : IntRect::new(sprite_index * size, 0, size, size),
        }
    }

    pub fn is_done(&self) -> bool {
        self.step >= self.count - 1
    }

    pub fn set_animation(&mut self, sprite_index : i32, delay : i32, count : i32, mode : AnimationMode) {
        self.started = false;
        self.sprite_index = sprite_index;
        self.delay = delay;
        self.count = count;
        self.mode = mode;
        self.clock.restart();
    }

    pub fn next_frame(&mut self, window: &mut RenderWindow) -> bool {
        println!("STEP {}", self.step);
        let mut is_closed_current_action = false;
        match self.started {
            true => {
                if self.clock.elapsed_time().as_milliseconds() >= self.delay {
                    if self.is_done() {
                        match self.mode {
                            AnimationMode::Repeated => {
                                self.step = 0;
                            }
                            _ => {
                                is_closed_current_action = true;
                            }
                        }
                    } 
                    else {
                         self.step = self.step + 1;
                    }
                    self.clock.restart();    
                }
            }
            false => {
                self.started = true;
            }
        }
        self.rect.left = (self.sprite_index + self.step) * self.size;
        self.sprite.set_texture_rect(self.rect);
        self.update_position();
        window.draw(&self.sprite);
        is_closed_current_action
    }

    fn update_position(&mut self) {
        self.position.x = self.position.x + self.speed.x;
        self.position.y = self.position.y + self.speed.y;
        self.sprite.set_position(self.position);
        let x_scale = if self.direction == Direction::Left { -1. * self.scale.x } else { self.scale.x };
        self.sprite.set_scale(Vector2f::new(x_scale, self.scale.y));
    }

}