use sfml;

use sfml::graphics::{RenderTarget, Shape, Transformable};


fn main() {
    let mut window = sfml::graphics::RenderWindow::new(
        sfml::window::VideoMode::desktop_mode(),
        "GamePlatform",
        sfml::window::Style::FULLSCREEN,
        &Default::default(),
    );

    window.set_vertical_sync_enabled(true);

    let mut rect = sfml::graphics::RectangleShape::new();
    rect.set_size(sfml::system::Vector2f::new(50.0, 100.0));
    rect.set_position(sfml::system::Vector2f::new(100.0, 200.0));
    rect.set_fill_color(sfml::graphics::Color::BLUE);

    'main_loop: loop {
        while let Some(e) = window.poll_event() {
            match e {
                sfml::window::Event::Closed => break 'main_loop,
                sfml::window::Event::KeyPressed {code: sfml::window::Key::Escape, ..} => break 'main_loop,

                _ => {}
            }
        }

        window.clear(sfml::graphics::Color::BLACK);
        window.draw(&rect);

        window.display()
    }
}