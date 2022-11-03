mod controller;
mod graphics;

use sfml;
use sfml::graphics::{Drawable, RenderTarget, Shape, Transformable};


struct Tile<'a> {
    rect: sfml::graphics::RectangleShape<'a>,

}

impl Tile<'_> {
    pub fn new() -> Self {
        let mut result = Tile{
            rect: sfml::graphics::RectangleShape::new(),
        };
        result.rect.set_size(sfml::system::Vector2f::new(500.0, 500.0));
        result.rect.set_fill_color(sfml::graphics::Color::BLUE);
        result
    }

    pub fn draw(&mut self, offset: f32, graphics_manager: &mut graphics::GraphicsManager) {
        self.rect.set_position(sfml::system::Vector2f::new(offset, 500.0));
        graphics_manager.window.draw(&self.rect);
    }
}



fn main() {
    let mut graphics_manager = graphics::GraphicsManager::new("GamePlatform");

    let mut controller_manager = controller::ControllerManager::new();
    controller_manager.update(&mut graphics_manager);

    let mut text = graphics_manager.create_text("Controller is connected");
    let mut text_sprite = sfml::graphics::Sprite::new();
    text_sprite.set_texture(&text.texture(), true);
    text_sprite.set_position(sfml::system::Vector2f::new(50.0, 50.0));

    let mut tiles = vec![Tile::new(), Tile::new(), Tile::new()];

    while graphics_manager.window.is_open() {
        while let Some(e) = graphics_manager.window.poll_event() {
            match e {
                sfml::window::Event::KeyPressed {code: sfml::window::Key::Escape, ..} => graphics_manager.window.close(),

                _ => {}
            }
        }

        controller_manager.update(&mut graphics_manager);

        graphics_manager.window.clear(sfml::graphics::Color::BLACK);
        graphics_manager.window.draw(&text_sprite);

        let mut curr_x = 0.0;
        for i in 0..tiles.len() {
            tiles[i].draw(curr_x, &mut graphics_manager);
            curr_x += 50.0;
        }

        graphics_manager.window.display()
    }
}