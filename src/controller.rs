pub const NO_CONTROLLER: u32 = u32::MAX;
const AXIS_SHADOW: f32 = 10.0;
pub struct ControllerManager {
    controller_id: u32
}

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

    pub fn is_button_pressed(&self, button: ControllerButton) -> bool {
        sfml::window::joystick::is_button_pressed(self.controller_id, button as u32)
    }

    pub fn get_axis_rotation_left(&self) -> (f32, f32) {
        let axis_x = sfml::window::joystick::axis_position(self.controller_id, sfml::window::joystick::Axis::X);
        let axis_y = sfml::window::joystick::axis_position(self.controller_id, sfml::window::joystick::Axis::Y);

        (
            if axis_x.abs() < AXIS_SHADOW {0.0} else {axis_x},
            if axis_y.abs() < AXIS_SHADOW {0.0} else {axis_y}
        )
    }

    pub fn get_axis_rotation_right(&self) -> (f32, f32) {
        let axis_x = sfml::window::joystick::axis_position(self.controller_id, sfml::window::joystick::Axis::R);
        let axis_y = sfml::window::joystick::axis_position(self.controller_id, sfml::window::joystick::Axis::Z);

        (
            if axis_x.abs() < AXIS_SHADOW {0.0} else {axis_x},
            if axis_y.abs() < AXIS_SHADOW {0.0} else {axis_y}
        )
    }
}
