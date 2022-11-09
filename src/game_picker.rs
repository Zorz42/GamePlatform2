use crate::graphics;
use rand;
use sfml;
use crate::game_manager;
use sfml::graphics::{RenderTarget, Shape, Transformable};

struct Tile {
    icon: sfml::SfBox<sfml::graphics::Texture>,
    title: sfml::graphics::RenderTexture,
    icon_x: f32,
    icon_y: f32,
    icon_w: f32,
    icon_h: f32,
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
            icon_x: 0.0,
            icon_y: 0.0,
            icon_w: 0.0,
            icon_h: 0.0,
        }
    }

    pub fn draw(&mut self, offset: f32, graphics_manager: &mut graphics::GraphicsManager, is_selected: bool, is_running: bool, is_out_of_focus: bool) {
        let mut icon_sprite = sfml::graphics::Sprite::new();
        icon_sprite.set_texture(&*self.icon, false);

        let target_x;
        let target_y;
        let target_w;
        let target_h;
        if is_running {
            target_x = 0.0;
            target_y = 0.0;
            target_w = sfml::window::VideoMode::desktop_mode().width as f32;
            target_h = sfml::window::VideoMode::desktop_mode().height as f32;
        } else if is_selected {
            target_x = offset - get_tile_size_change() / 2.0;
            target_y = get_tile_offset();
            target_w = get_tile_size() + get_tile_size_change();
            target_h = get_tile_size() + get_tile_size_change();
        } else if is_out_of_focus {
            target_x = offset + get_tile_size_change() / 2.0;
            target_y = get_tile_offset() + get_tile_size_change() / 2.0;
            target_w = get_tile_size() - get_tile_size_change();
            target_h = get_tile_size() - get_tile_size_change();
        } else {
            target_x = offset;
            target_y = get_tile_offset();
            target_w = get_tile_size();
            target_h = get_tile_size();
        }

        self.icon_x += (target_x - self.icon_x) / 8.0;
        self.icon_y += (target_y - self.icon_y) / 8.0;
        self.icon_w += (target_w - self.icon_w) / 8.0;
        self.icon_h += (target_h - self.icon_h) / 8.0;

        icon_sprite.set_position(sfml::system::Vector2f::new(self.icon_x, self.icon_y));
        let size_x = self.icon_w / self.icon.size().x as f32;
        let size_y = self.icon_h / self.icon.size().y as f32;
        icon_sprite.set_scale(sfml::system::Vector2f::new(size_x, size_y));
        graphics_manager.window.draw(&icon_sprite);

        let mut text_sprite = sfml::graphics::Sprite::new();
        text_sprite.set_texture(self.title.texture(), false);
        text_sprite.set_position(sfml::system::Vector2f::new(self.icon_x, self.icon_y + self.icon_h));
        //text_sprite.set_scale(sfml::system::Vector2f::new(1.0 + self.select_progress, 1.0 + self.select_progress));
        graphics_manager.window.draw(&text_sprite);
    }
}

pub struct GamePickerScene {
    tiles: Vec<Tile>,
    tiles_vel: f32,
    tiles_pos: f32,
    tiles_render_pos: f32,
    selected_tile: u32,
    games: game_manager::GameManager,
    running_thread: Option<std::thread::JoinHandle<()>>,
    running_game_index: i32,
}

impl GamePickerScene {
    pub fn new() -> GamePickerScene {
        GamePickerScene{
            tiles: vec![],
            tiles_vel: 0.0,
            tiles_pos: 0.0,
            tiles_render_pos: 0.0,
            selected_tile: 0,
            games: game_manager::GameManager::new(),
            running_thread: None,
            running_game_index: -1,
        }
    }
}

impl graphics::Scene for GamePickerScene {
    fn init(&mut self, graphics: &mut graphics::GraphicsManager) {
        self.games.init();
        for game in &self.games.games {
            let texture = sfml::graphics::Texture::from_file(&*game.icon_path).unwrap();
            self.tiles.push(Tile::new(graphics,game.name.clone(), texture))
        }
    }

    fn render(&mut self, mut graphics: &mut graphics::GraphicsManager) {
        let game_running = !self.running_thread.is_none();

        if let Some(thread) = &self.running_thread {
            if thread.is_finished() {
                self.running_thread = None;
            } else {

            }
            self.tiles_vel = 0.0;
        } else {
            self.tiles_vel += -graphics.get_axis_rotation_left().0 / 4.0;
            if graphics.get_axis_rotation_left().0 == 0.0 {
                self.tiles_vel = 0.0;
            }
        }

        self.tiles_vel = f32::max(self.tiles_vel, -300.0);
        self.tiles_vel = f32::min(self.tiles_vel, 300.0);

        self.tiles_pos += self.tiles_vel;
        self.tiles_pos = f32::max(self.tiles_pos, (self.tiles.len() as f32 - 1.0) * -(get_tile_size() + get_tile_spacing()));
        self.tiles_pos = f32::min(self.tiles_pos, 0.0);

        if graphics.get_axis_rotation_left().0 == 0.0 {
            self.tiles_pos = self.tiles_render_pos;
        } else {
            let offset = if graphics.get_axis_rotation_left().0 > 0.0 { -0.49 } else { 0.49 };
            self.tiles_render_pos = (self.tiles_pos / (get_tile_size() + get_tile_spacing()) + offset).round() * (get_tile_size() + get_tile_spacing());
        }

        let mut curr_x = self.tiles_render_pos + sfml::window::VideoMode::desktop_mode().width as f32 / 2.0 - get_tile_size() / 2.0;

        self.selected_tile = (-self.tiles_render_pos / (get_tile_size() + get_tile_spacing())).round() as u32;
        for i in 0..self.tiles.len() {
            if i as i32 != self.running_game_index || !game_running {
                self.tiles[i].draw(curr_x, &mut graphics, i as u32 == self.selected_tile, false, game_running);
            }
            curr_x += get_tile_size() + get_tile_spacing();
        }

        if game_running {
            self.tiles[self.running_game_index as usize].draw(curr_x, &mut graphics, false, true, false);
        }

        if self.running_game_index != -1 {
            let running_game_tile = &self.tiles[self.running_game_index as usize];
            let transparency = f32::min(running_game_tile.icon_w * 3.0 / sfml::window::VideoMode::desktop_mode().width as f32 - 1.5, 1.0);
            if transparency < 0.0 {
                if !game_running {
                    self.running_game_index = -1;
                }
            } else {
                self.games.render_game(graphics, running_game_tile.icon_x, running_game_tile.icon_y, running_game_tile.icon_w, running_game_tile.icon_h, transparency);
            }
        }
    }

    fn on_event(&mut self, graphics: &mut graphics::GraphicsManager, event: sfml::window::Event) {
        match event {
            sfml::window::Event::KeyPressed {code: sfml::window::Key::Escape, ..} => graphics.window.close(),
            sfml::window::Event::JoystickButtonPressed {button: 1, ..} => {
                let game = self.games.games[self.selected_tile as usize].clone();
                self.running_game_index = self.selected_tile as i32;
                self.running_thread = Some(std::thread::spawn(move|| {
                    game_manager::GameManager::run_game(&game);
                }));
            }
            _ => {}
        }
    }
}