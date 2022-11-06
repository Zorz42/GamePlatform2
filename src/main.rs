mod graphics;
mod game_picker;
mod game_manager;


fn main() {
    let mut graphics_manager = graphics::GraphicsManager::new("GamePlatform");
    graphics_manager.update_controllers();

    let main_menu = game_picker::GamePickerScene::new();
    graphics::run_scene( Box::new(main_menu), &mut graphics_manager);
}