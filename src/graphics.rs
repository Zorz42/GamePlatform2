pub struct GraphicsManager {
    pub window: sfml::graphics::RenderWindow,
    pub font: sfml::SfBox<sfml::graphics::Font>,
    should_close: bool,
}

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
            should_close: false,
        };
        gfx_manager.window.set_vertical_sync_enabled(true);
        gfx_manager
    }

    pub fn close(&mut self) {
        self.should_close = true;
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }
}