use sfml::graphics::{Drawable, RenderTarget, Shape, Transformable};

pub struct GraphicsManager {
    pub window: sfml::graphics::RenderWindow,
    font: sfml::SfBox<sfml::graphics::Font>,
}

const CHARACTER_SIZE: u32 = 64;

impl GraphicsManager {
    pub fn new(title: &str) -> Self {
        let mut gfx_manager = GraphicsManager{
            window: sfml::graphics::RenderWindow::new(
            sfml::window::VideoMode::desktop_mode(),
            title,
            sfml::window::Style::FULLSCREEN,
            &Default::default(),
            ),
            font: unsafe { sfml::graphics::Font::from_memory(std::include_bytes!("../resources/arial.ttf")) }.unwrap(),
        };
        gfx_manager.window.set_vertical_sync_enabled(true);
        gfx_manager
    }

    pub fn create_text(&self, text: &str) -> sfml::graphics::RenderTexture {
        let mut text = sfml::graphics::Text::new(text, &self.font, CHARACTER_SIZE);
        let mut texture = sfml::graphics::RenderTexture::new(text.local_bounds().width as u32 + 5, CHARACTER_SIZE).unwrap();
        texture.clear(sfml::graphics::Color::TRANSPARENT);
        texture.draw(&text);
        texture.display();
        texture
    }
}