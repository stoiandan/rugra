use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, RectangleShape, Text, Color};
use sfml::graphics::Sprite as SfmlSprite;
use sfml::system::vector2::Vector2f;

use window::Window;
use texture::Texture;

pub struct Sprite<'a> {
    sprite: SfmlSprite<'a>,
}

impl<'a> Sprite<'a> {
    pub fn new() -> Self {
        Sprite {
            sprite: SfmlSprite::new().unwrap(),
        }
    }

    pub fn x(&mut self, x: f32) -> &mut Self {
        let y = self.sprite.get_position().y;
        self.sprite.set_position2f(x, y);
        self
    }

    pub fn y(&mut self, y: f32) -> &mut Self {
        let x = self.sprite.get_position().x;
        self.sprite.set_position2f(x, y);
        self
    }

    pub fn texture(&mut self, texture: &'a Texture) -> &mut Self {
        self.sprite.set_texture(texture.to_sfml_texture(), true);
        self
    }

    pub fn sfml_sprite(&mut self) -> &mut SfmlSprite<'a> {
        &mut self.sprite
    }

    pub fn draw(&mut self, window: &mut Window) {
        window.sfml_window().draw(&mut self.sprite);
    }

    pub fn go(&mut self, x: f32, y: f32) -> &mut Self {
        let pos = self.sprite.get_position();
        self.sprite.set_position(&(pos + Vector2f::new(x, y)));
        self
    }

    pub fn rotate(&mut self, a: f32) -> &mut Self {
        self.sprite.rotate(a);
        self
    }

    pub fn step(&mut self, size: f32) -> &mut Self {
        let angle = size.to_radians();

        self.go(angle.cos() * size, angle.sin() * size);

        self
    }
}
