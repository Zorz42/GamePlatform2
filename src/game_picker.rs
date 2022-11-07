use crate::graphics;
use rand;
use sfml;
use crate::game_manager;
use sfml::graphics::{RenderTarget, Shape, Transformable};

struct Tile {
    icon: sfml::SfBox<sfml::graphics::Texture>,
    title: sfml::graphics::RenderTexture,
    select_progress: f32,
}

fn get_tile_size() -> f32 {
    sfml::window::VideoMode::desktop_mode().height as f32 * 0.5
}

fn get_tile_offset() -> f32 {
    sfml::window::VideoMode::desktop_mode().height as f32 * 0.25
}

fn get_tile_spacing() -> f32 {
    sfml::window::VideoMode::desktop_mode().height as f32 * 0.05
}

fn get_tile_size_change() -> f32 {
    sfml::window::VideoMode::desktop_mode().height as f32 * 0.05
}

impl Tile {
    pub fn new(graphics: &mut graphics::GraphicsManager, name: String, texture: sfml::SfBox<sfml::graphics::Texture>) -> Self {
        Tile{
            icon: texture,
            title: graphics.create_text(name.as_str(), 64, sfml::graphics::Color::WHITE),
            select_progress: 0.0,
        }
    }

    pub fn draw(&mut self, offset: f32, graphics_manager: &mut graphics::GraphicsManager, is_selected: bool) {
        let mut icon_sprite = sfml::graphics::Sprite::new();
        icon_sprite.set_texture(&*self.icon, false);

        let select_progress_must_be = if is_selected {1.0} else {0.0};
        self.select_progress += (select_progress_must_be - self.select_progress) / 2.0;

        icon_sprite.set_position(sfml::system::Vector2f::new(offset - self.select_progress * get_tile_size_change() / 2.0, get_tile_offset()));
        let size_x = (get_tile_size() + self.select_progress * get_tile_size_change()) / self.icon.size().x as f32;
        let size_y = (get_tile_size() + self.select_progress * get_tile_size_change()) / self.icon.size().y as f32;
        icon_sprite.set_scale(sfml::system::Vector2f::new(size_x, size_y));
        graphics_manager.window.draw(&icon_sprite);

        let mut text_sprite = sfml::graphics::Sprite::new();
        text_sprite.set_texture(self.title.texture(), false);
        text_sprite.set_position(sfml::system::Vector2f::new(offset + get_tile_size() / 2.0 - self.title.size().x as f32 / 2.0, get_tile_offset() + get_tile_size() + self.select_progress * get_tile_size_change() + get_tile_spacing()));
        text_sprite.set_scale(sfml::system::Vector2f::new(1.0 + self.select_progress, 1.0 + self.select_progress));
        graphics_manager.window.draw(&text_sprite);
    }
}

pub struct GamePickerScene {
    tiles: Vec<Tile>,
    tiles_vel: f32,
    tiles_pos: f32,
    tiles_render_pos_must_be: f32,
    tiles_render_pos: f32,
    selected_tile: u32,
    games: game_manager::GameManager,
}

impl GamePickerScene {
    pub fn new() -> GamePickerScene {
        GamePickerScene{
            tiles: vec![],
            tiles_vel: 0.0,
            tiles_pos: 0.0,
            tiles_render_pos_must_be: 0.0,
            tiles_render_pos: 0.0,
            selected_tile: 0,
            games: game_manager::GameManager::new(),
        }
    }
}

impl graphics::Scene for GamePickerScene {
    fn init(&mut self, mut graphics: &mut graphics::GraphicsManager) {
        self.games.init();
        for game in &self.games.games {
            let texture = sfml::graphics::Texture::from_file(&*game.icon_path).unwrap();
            self.tiles.push(Tile::new(graphics,game.name.clone(), texture))
        }
    }

    fn render(&mut self, mut graphics: &mut graphics::GraphicsManager) {
        self.tiles_vel += -graphics.get_axis_rotation_left().0 / 4.0;
        if graphics.get_axis_rotation_left().0 == 0.0 {
            self.tiles_vel = 0.0;
        }
        self.tiles_vel = f32::max(self.tiles_vel, -300.0);
        self.tiles_vel = f32::min(self.tiles_vel, 300.0);

        self.tiles_pos += self.tiles_vel;
        self.tiles_pos = f32::max(self.tiles_pos, (self.tiles.len() as f32 - 1.0) * -(get_tile_size() + get_tile_spacing()));
        self.tiles_pos = f32::min(self.tiles_pos, 0.0);

        if graphics.get_axis_rotation_left().0 == 0.0 {
            self.tiles_pos = self.tiles_render_pos_must_be;
        } else {
            let offset = if graphics.get_axis_rotation_left().0 > 0.0 {-0.49} else {0.49};
            self.tiles_render_pos_must_be = (self.tiles_pos / (get_tile_size() + get_tile_spacing()) + offset).round() * (get_tile_size() + get_tile_spacing());
        }

        self.tiles_render_pos += (self.tiles_render_pos_must_be - self.tiles_render_pos) / 7.0;

        let mut curr_x = self.tiles_render_pos + sfml::window::VideoMode::desktop_mode().width as f32 / 2.0 - get_tile_size() / 2.0;

        self.selected_tile = (-self.tiles_render_pos_must_be / (get_tile_size() + get_tile_spacing())).round() as u32;
        for i in 0..self.tiles.len() {
            self.tiles[i].draw(curr_x, &mut graphics, i as u32 == self.selected_tile);
            curr_x += get_tile_size() + get_tile_spacing();
        }
    }

    fn on_event(&mut self, graphics: &mut graphics::GraphicsManager, event: sfml::window::Event) {
        match event {
            sfml::window::Event::KeyPressed {code: sfml::window::Key::Escape, ..} => graphics.window.close(),
            sfml::window::Event::JoystickButtonPressed {button: 1, ..} => {
                let game = self.games.games[self.selected_tile as usize].clone();
                std::thread::spawn(move|| {
                    game_manager::GameManager::run_game(&game);
                });
            }
            _ => {}
        }
    }
}