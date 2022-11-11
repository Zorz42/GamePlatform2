use sfml;
use sfml::graphics::{RenderTarget, Transformable};
use crate::graphics;
use crate::game_manager;
use laminar;

pub struct GameInstanceManager {
    pub game_window: sfml::graphics::RenderTexture,
    pub socket: laminar::Socket,
}

impl GameInstanceManager {
    pub fn new() -> Self {
        GameInstanceManager{
            game_window: sfml::graphics::RenderTexture::new(sfml::window::VideoMode::desktop_mode().width as u32, sfml::window::VideoMode::desktop_mode().height as u32).unwrap(),
            socket: laminar::Socket::bind("127.0.0.1:65342".parse::<String>().unwrap()).unwrap(),
        }
    }

    pub fn init(&mut self) {
        self.game_window.clear(sfml::graphics::Color::RED);
    }

    pub fn stop(&mut self) {

    }

    pub fn render_game(&mut self, graphics: &mut graphics::GraphicsManager, x: f32, y: f32, w: f32, h: f32, transparency: f32) {
        let mut sprite = sfml::graphics::Sprite::new();
        sprite.set_texture(self.game_window.texture(), false);
        sprite.set_color(sfml::graphics::Color::rgba(255, 255, 255, (255.0 * transparency) as u8));
        sprite.set_position(sfml::system::Vector2f::new(x, y));
        let size_x = w / self.game_window.size().x as f32;
        let size_y = h / self.game_window.size().y as f32;
        sprite.set_scale(sfml::system::Vector2f::new(size_x, size_y));
        graphics.window.draw(&sprite);
    }

    pub fn run_game(game: &game_manager::Game) {
        let mut binding = std::process::Command::new("cargo");
        let command = binding.arg("run");
        command.current_dir(game.game_dir.clone() + "/Game/");
        command.spawn().unwrap().wait().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}