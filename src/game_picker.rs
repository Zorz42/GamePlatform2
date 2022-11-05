use crate::graphics;
use rand;
use sfml;
use sfml::graphics::{RenderTarget, Shape, Transformable};
use crate::graphics::{GraphicsManager};

struct Tile<'a> {
    rect: sfml::graphics::RectangleShape<'a>,
}

const TILE_SIZE: f32 = 1500.0;
const TILE_SPACING: f32 = 300.0;
const TILE_SIZE_CHANGE: f32 = 200.0;

impl Tile<'_> {
    pub fn new() -> Self {
        let mut result = Tile{
            rect: sfml::graphics::RectangleShape::new(),
        };
        result.rect.set_fill_color(sfml::graphics::Color::rgb(rand::random(), rand::random(), rand::random()));
        result
    }

    pub fn draw(&mut self, offset: f32, graphics_manager: &mut graphics::GraphicsManager, is_selected: bool) {
        if is_selected {
            self.rect.set_position(sfml::system::Vector2f::new(offset - TILE_SIZE_CHANGE / 2.0, 500.0));
            self.rect.set_size(sfml::system::Vector2f::new(TILE_SIZE + TILE_SIZE_CHANGE, TILE_SIZE + TILE_SIZE_CHANGE));
        } else {
            self.rect.set_position(sfml::system::Vector2f::new(offset, 500.0));
            self.rect.set_size(sfml::system::Vector2f::new(TILE_SIZE, TILE_SIZE));
        }
        graphics_manager.window.draw(&self.rect);
    }
}

pub struct GamePickerScene<'a> {
    tiles: Vec<Tile<'a>>,
    tiles_vel: f32,
    tiles_pos: f32,
    tiles_render_pos_must_be: f32,
    tiles_render_pos: f32,
}

impl GamePickerScene<'_> {
    pub fn new() -> GamePickerScene<'static> {
        GamePickerScene{
            tiles: vec![],
            tiles_vel: 0.0,
            tiles_pos: 0.0,
            tiles_render_pos_must_be: 0.0,
            tiles_render_pos: 0.0
        }
    }
}

impl graphics::Scene for GamePickerScene<'_> {
    fn init(&mut self, mut _graphics: &mut GraphicsManager) {
        for _ in 0..10 {
            self.tiles.push(Tile::new());
        }
    }

    fn render(&mut self, mut graphics: &mut GraphicsManager) {
        self.tiles_vel += -graphics.get_axis_rotation_left().0 / 4.0;
        if graphics.get_axis_rotation_left().0 == 0.0 {
            self.tiles_vel = 0.0;
        }
        self.tiles_vel = f32::max(self.tiles_vel, -300.0);
        self.tiles_vel = f32::min(self.tiles_vel, 300.0);

        self.tiles_pos += self.tiles_vel;
        self.tiles_pos = f32::max(self.tiles_pos, (self.tiles.len() - 1) as f32 * -(TILE_SIZE + TILE_SPACING));
        self.tiles_pos = f32::min(self.tiles_pos, 0.0);

        if graphics.get_axis_rotation_left().0 == 0.0 {
            self.tiles_pos = self.tiles_render_pos_must_be;
        } else {
            let offset = if graphics.get_axis_rotation_left().0 > 0.0 {-0.49} else {0.49};
            self.tiles_render_pos_must_be = (self.tiles_pos / (TILE_SIZE + TILE_SPACING) + offset).round() * (TILE_SIZE + TILE_SPACING);
        }

        self.tiles_render_pos += (self.tiles_render_pos_must_be - self.tiles_render_pos) / 7.0;

        let mut curr_x = self.tiles_render_pos + sfml::window::VideoMode::desktop_mode().width as f32 / 2.0 - TILE_SIZE / 2.0;

        let selected_tile = (-self.tiles_render_pos_must_be / (TILE_SIZE + TILE_SPACING)) as u32;
        for i in 0..self.tiles.len() {
            self.tiles[i].draw(curr_x, &mut graphics, i as u32 == selected_tile);
            curr_x += TILE_SIZE + TILE_SPACING;
        }
    }

    fn on_event(&mut self, graphics: &mut GraphicsManager, event: sfml::window::Event) {
        match event {
            sfml::window::Event::KeyPressed {code: sfml::window::Key::Escape, ..} => graphics.window.close(),
            _ => {}
        }
    }
}