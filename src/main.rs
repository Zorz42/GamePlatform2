mod controller;
mod graphics;

use rand;
use sfml;
use sfml::graphics::{RenderTarget, Shape, Transformable};


struct Tile<'a> {
    rect: sfml::graphics::RectangleShape<'a>,
}

const TILE_SIZE: f32 = 1500.0;
const TILE_SPACING: f32 = 300.0;

impl Tile<'_> {
    pub fn new() -> Self {
        let mut result = Tile{
            rect: sfml::graphics::RectangleShape::new(),
        };
        result.rect.set_size(sfml::system::Vector2f::new(TILE_SIZE, TILE_SIZE));
        result.rect.set_fill_color(sfml::graphics::Color::rgb(rand::random(), rand::random(), rand::random()));
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

    let mut tiles = vec![];
    for _ in 0..10 {
        tiles.push(Tile::new());
    }
    let mut tiles_vel = 0.0;
    let mut tiles_pos = 0.0;
    let mut tiles_render_pos_must_be = 0.0;
    let mut tiles_render_pos = 0.0;

    while graphics_manager.window.is_open() {
        while let Some(e) = graphics_manager.window.poll_event() {
            match e {
                sfml::window::Event::KeyPressed {code: sfml::window::Key::Escape, ..} => graphics_manager.window.close(),

                _ => {}
            }
        }

        controller_manager.update(&mut graphics_manager);

        graphics_manager.window.clear(sfml::graphics::Color::BLACK);

        tiles_vel += -controller_manager.get_axis_rotation_left().0 / 4.0;
        if controller_manager.get_axis_rotation_left().0 == 0.0 {
            tiles_vel = 0.0;
        }
        tiles_vel = f32::max(tiles_vel, -300.0);
        tiles_vel = f32::min(tiles_vel, 300.0);

        tiles_pos += tiles_vel;
        tiles_pos = f32::max(tiles_pos, (tiles.len() - 1) as f32 * -(TILE_SIZE + TILE_SPACING));
        tiles_pos = f32::min(tiles_pos, 0.0);

        if controller_manager.get_axis_rotation_left().0 == 0.0 {
            tiles_pos = tiles_render_pos_must_be;
        } else {
            let offset = if controller_manager.get_axis_rotation_left().0 > 0.0 {-0.49} else {0.49};
            tiles_render_pos_must_be = (tiles_pos / (TILE_SIZE + TILE_SPACING) + offset).round() * (TILE_SIZE + TILE_SPACING);
        }

        tiles_render_pos += (tiles_render_pos_must_be - tiles_render_pos) / 7.0;

        let mut curr_x = tiles_render_pos + sfml::window::VideoMode::desktop_mode().width as f32 / 2.0 - TILE_SIZE / 2.0;
        for i in 0..tiles.len() {
            tiles[i].draw(curr_x, &mut graphics_manager);
            curr_x += TILE_SIZE + TILE_SPACING;
        }

        graphics_manager.window.display()
    }
}