use toml;
use serde_derive;

const GAMES_DIR: &str = "Games";

#[derive(serde_derive::Deserialize)]
struct GameConfig {
    name: String,
}

pub struct Game {
    pub name: String,
}

pub struct GameManager {
    pub games: Vec<Game>,
}

impl GameManager {
    pub fn new() -> Self {
        GameManager{
            games: vec![],
        }
    }

    pub fn init(&mut self) {
        std::fs::create_dir_all(GAMES_DIR).expect("");

        let paths = std::fs::read_dir("Games/").unwrap();
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
        let config = Self::read_game_config(dirname + "/config.toml");
        Game{
            name: config.name,
        }
    }
}