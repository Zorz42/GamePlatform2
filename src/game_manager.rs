use toml;
use serde_derive;
use sfml;
use sfml::graphics::{RenderTarget, Transformable};
use crate::graphics;

const GAMES_DIR: &str = "Games";

#[derive(serde_derive::Deserialize)]
struct GameConfig {
    name: String,
    icon: String,
}

#[derive(Clone)]
pub struct Game {
    pub name: String,
    pub icon_path: String,
    pub game_dir: String,
}

pub struct GameManager {
    pub games: Vec<Game>,
    pub game_window: sfml::graphics::RenderTexture,
}

impl GameManager {
    pub fn new() -> Self {
        GameManager{
            games: vec![],
            game_window: sfml::graphics::RenderTexture::new(sfml::window::VideoMode::desktop_mode().width as u32, sfml::window::VideoMode::desktop_mode().height as u32).unwrap(),
        }
    }

    pub fn init(&mut self) {
        let games_dir = std::env::current_dir().unwrap().to_str().unwrap().to_owned() + "/" + GAMES_DIR;
        std::fs::create_dir_all(games_dir.clone()).unwrap();

        let paths = std::fs::read_dir(games_dir.clone()).unwrap();
        for path in paths {
            let path2 = path.unwrap().path();
            let name = path2.to_str().unwrap();
            if path2.is_dir() && name.ends_with(".game") {
                self.games.push(Self::read_game(String::from(name)));
            }
        }
    }

    fn read_game_config(filename: String) -> GameConfig {
        let contents = match std::fs::read_to_string(filename.as_str()) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Could not read file `{}`", filename.as_str());
                std::process::exit(1);
            }
        };

        let data: GameConfig = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Unable to load data from `{}`", filename.as_str());
                std::process::exit(1);
            }
        };

        data
    }

    fn read_game(dirname: String) -> Game {
        let config = Self::read_game_config(dirname.clone() + "/config.toml");
        Game{
            name: config.name,
            icon_path: dirname.clone() + "/" + &*config.icon,
            game_dir: dirname.clone(),
        }
    }

    pub fn run_game(game: &Game) {
        let mut binding = std::process::Command::new("cargo");
        let command = binding.arg("run");
        command.current_dir(game.game_dir.clone() + "/Game/");
        command.spawn().unwrap().wait().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    pub fn render_game(&mut self, graphics: &mut graphics::GraphicsManager, x: f32, y: f32, w: f32, h: f32, transparency: f32) {
        self.game_window.clear(sfml::graphics::Color::RED);

        let mut sprite = sfml::graphics::Sprite::new();
        sprite.set_texture(self.game_window.texture(), false);
        sprite.set_color(sfml::graphics::Color::rgba(255, 255, 255, (255.0 * transparency) as u8));
        sprite.set_position(sfml::system::Vector2f::new(x, y));
        let size_x = w / self.game_window.size().x as f32;
        let size_y = h / self.game_window.size().y as f32;
        sprite.set_scale(sfml::system::Vector2f::new(size_x, size_y));
        graphics.window.draw(&sprite);
    }
}