use sfml::graphics::{RenderTarget, Shape, Transformable};

const NO_CONTROLLER: u32 = u32::MAX;
const AXIS_SHADOW: f32 = 50.0;

pub enum ControllerButton {
    Cross = 1,
    Circle = 2,
    Triangle = 3,
    Square = 0,
    Right1 = 5,
    Right2 = 7,
    Left1 = 4,
    Left2 = 6,
    Share = 8,
    Options = 9,
    RightAxis = 11,
    LeftAxis = 10,
    Pad = 13,
}

pub struct GraphicsManager {
    pub window: sfml::graphics::RenderWindow,
    font: sfml::SfBox<sfml::graphics::Font>,
    controller_id: u32,
    no_controller_texture: Option<sfml::graphics::RenderTexture>,
}

impl GraphicsManager {
    pub fn new(title: &str) -> Self {
        let mut graphics = GraphicsManager{
            window: sfml::graphics::RenderWindow::new(
            sfml::window::VideoMode::desktop_mode(),
            title,
            sfml::window::Style::FULLSCREEN,
            &Default::default(),
            ),
            font: unsafe { sfml::graphics::Font::from_memory(include_bytes!("../resources/arial.ttf")) }.unwrap(),
            controller_id: NO_CONTROLLER,
            no_controller_texture: None,
        };
        graphics.window.set_vertical_sync_enabled(true);
        graphics.no_controller_texture = Some(graphics.create_text("Controller is not connected, you can press ESC to exit.", 100, sfml::graphics::Color::WHITE));
        graphics
    }

    pub fn create_text(&self, text: &str, char_size: u32, fill_color: sfml::graphics::Color) -> sfml::graphics::RenderTexture {
        let mut text = sfml::graphics::Text::new(text, &self.font, char_size);
        text.set_fill_color(fill_color);
        let mut texture = sfml::graphics::RenderTexture::new(text.local_bounds().width as u32 + 5, char_size * 2).unwrap();
        texture.clear(sfml::graphics::Color::TRANSPARENT);
        texture.draw(&text);
        texture.display();
        texture
    }

    fn update_controller_id(&mut self) {
        sfml::window::joystick::update();

        self.controller_id = NO_CONTROLLER;
        for id in 0..sfml::window::joystick::COUNT {
            if sfml::window::joystick::is_connected(id) {
                self.controller_id = id;
            }
        }
    }

    pub fn is_controller_connected(&self) -> bool {
        self.controller_id != NO_CONTROLLER
    }

    pub fn is_button_pressed(&self, button: ControllerButton) -> bool {
        sfml::window::joystick::is_button_pressed(self.controller_id, button as u32)
    }

    pub fn get_axis_rotation_left(&self) -> (f32, f32) {
        if !self.is_controller_connected() {
            return (0.0, 0.0);
        }

        let axis_x = sfml::window::joystick::axis_position(self.controller_id, sfml::window::joystick::Axis::X);
        let axis_y = sfml::window::joystick::axis_position(self.controller_id, sfml::window::joystick::Axis::Y);

        (
            if axis_x.abs() < AXIS_SHADOW {0.0} else {axis_x},
            if axis_y.abs() < AXIS_SHADOW {0.0} else {axis_y}
        )
    }

    pub fn get_axis_rotation_right(&self) -> (f32, f32) {
        if !self.is_controller_connected() {
            return (0.0, 0.0);
        }

        let axis_x = sfml::window::joystick::axis_position(self.controller_id, sfml::window::joystick::Axis::R);
        let axis_y = sfml::window::joystick::axis_position(self.controller_id, sfml::window::joystick::Axis::Z);

        (
            if axis_x.abs() < AXIS_SHADOW {0.0} else {axis_x},
            if axis_y.abs() < AXIS_SHADOW {0.0} else {axis_y}
        )
    }

    pub fn update_controllers(&mut self) {
        self.update_controller_id();
        if !self.is_controller_connected() {
            let mut back_rect = sfml::graphics::RectangleShape::new();
            back_rect.set_size(sfml::system::Vector2f::new(sfml::window::VideoMode::desktop_mode().width as f32, sfml::window::VideoMode::desktop_mode().height as f32));
            back_rect.set_fill_color(sfml::graphics::Color::rgba(0, 0, 0, 200));
            self.window.draw(&back_rect);

            let mut text_sprite = sfml::graphics::Sprite::new();
            text_sprite.set_texture(&self.no_controller_texture.as_ref().unwrap().texture(), true);
            text_sprite.set_position(sfml::system::Vector2f::new(100.0, 100.0));
            self.window.draw(&text_sprite);
        }
    }
}

pub trait Scene {
    fn init(&mut self, _graphics: &mut GraphicsManager) {}
    fn render(&mut self, _graphics: &mut GraphicsManager) {}
    fn on_event(&mut self, _graphics: &mut GraphicsManager, _event: sfml::window::Event) {}
    fn stop(&mut self, _graphics: &mut GraphicsManager) {}
}

pub fn run_scene(mut scene: Box<dyn Scene>, graphics: &mut GraphicsManager) {
    scene.init(graphics);

    while graphics.window.is_open() {
        while let Some(event) = graphics.window.poll_event() {
            scene.on_event(graphics, event);
        }

        graphics.window.clear(sfml::graphics::Color::BLACK);

        scene.render(graphics);
        graphics.update_controllers();

        graphics.window.display()
    }

    scene.stop(graphics);
}