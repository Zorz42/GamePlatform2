mod controller;

use sfml;

use sfml::graphics::{RenderTarget, Transformable};


fn main() {
    let mut controller_manager = controller::ControllerManager::new();

    controller_manager.update();

    let mut window = sfml::graphics::RenderWindow::new(
        sfml::window::VideoMode::desktop_mode(),
        "GamePlatform",
        sfml::window::Style::FULLSCREEN,
        &Default::default(),
    );

    window.set_vertical_sync_enabled(true);

    let font = unsafe { sfml::graphics::Font::from_memory(std::include_bytes!("../resources/arial.ttf")) }.unwrap();

    let mut text1 = sfml::graphics::Text::new("Controller is connected", &font, 64);
    text1.set_position(sfml::system::Vector2f::new(50.0, 50.0));

    let mut text2 = sfml::graphics::Text::new("Controller is not connected", &font, 64);
    text2.set_position(sfml::system::Vector2f::new(50.0, 50.0));

    'main_loop: loop {
        while let Some(e) = window.poll_event() {
            match e {
                sfml::window::Event::Closed => break 'main_loop,
                sfml::window::Event::KeyPressed {code: sfml::window::Key::Escape, ..} => break 'main_loop,

                _ => {}
            }
        }

        controller_manager.update();

        window.clear(sfml::graphics::Color::BLACK);
        if controller_manager.is_controller_connected() {
            window.draw(&text1);
        } else {
            window.draw(&text2);
        }

        window.display()
    }
}