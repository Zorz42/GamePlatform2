pub const NO_CONTROLLER: u32 = u32::MAX;
pub struct ControllerManager {
    controller_id: u32
}

impl ControllerManager {
    pub fn new() -> Self {
        ControllerManager{ controller_id: NO_CONTROLLER }
    }

    pub fn update(&mut self) {
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
}
