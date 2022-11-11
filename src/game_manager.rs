use toml;
use serde_derive;
use sfml;
use sfml::graphics::{RenderTarget, Transformable};
use crate::graphics;
use crate::game_instance;

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
    pub game_instance: Option<game_instance::GameInstanceManager>,
}

impl GameManager {
    pub fn new() -> Self {
        GameManager{
            games: vec![],
            game_instance: None,
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

    pub fn start_game_instance(&mut self) {
        self.game_instance = Some(game_instance::GameInstanceManager::new());
        self.game_instance.as_mut().unwrap().init();
    }

    pub fn is_game_instance_running(&self) -> bool {
        !self.game_instance.is_none()
    }

    pub fn end_game_instance(&mut self) {
        self.game_instance.as_mut().unwrap().stop();
        self.game_instance = None;
    }
}