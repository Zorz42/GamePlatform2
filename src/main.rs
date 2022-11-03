mod controller;
mod graphics;

use sfml;
use sfml::graphics::{RenderTarget, Transformable};


fn main() {
    let mut graphics_manager = graphics::GraphicsManager::new("GamePlatform");

    let mut controller_manager = controller::ControllerManager::new();
    controller_manager.update(&mut graphics_manager);

    //let mut text = sfml::graphics::Text::new("Controller is connected", &graphics_manager.font, 64);
    //text.set_position(sfml::system::Vector2f::new(50.0, 50.0));


    while graphics_manager.window.is_open() {
        while let Some(e) = graphics_manager.window.poll_event() {
            match e {
                sfml::window::Event::KeyPressed {code: sfml::window::Key::Escape, ..} => graphics_manager.window.close(),

                _ => {}
            }
        }

        controller_manager.update(&mut graphics_manager);

        graphics_manager.window.clear(sfml::graphics::Color::BLACK);
        //graphics_manager.window.draw(&text);

        graphics_manager.window.display()
    }
}