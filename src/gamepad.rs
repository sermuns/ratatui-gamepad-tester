use gilrs::{Button, GamepadId};

const KONAMI_CODE: [Button; 10] = [
    Button::DPadUp,
    Button::DPadUp,
    Button::DPadDown,
    Button::DPadDown,
    Button::DPadLeft,
    Button::DPadRight,
    Button::DPadLeft,
    Button::DPadRight,
    Button::East,
    Button::South,
];

#[derive(Default, Debug)]
pub struct Gamepad {
    pub axises: Axises,
    pub buttons: Buttons,
    button_history: [Button; KONAMI_CODE.len()],
    button_history_index: usize,
}

impl Gamepad {
    const STICK_ACTIVATION_HYPOT: f32 = 0.1;

    pub fn left_stick_active(&self) -> bool {
        self.axises.left_stick_x.hypot(self.axises.left_stick_y) > Self::STICK_ACTIVATION_HYPOT
    }

    pub fn right_stick_active(&self) -> bool {
        self.axises.right_stick_x.hypot(self.axises.right_stick_y) > Self::STICK_ACTIVATION_HYPOT
    }
}

#[derive(Default, Debug)]
pub struct Axises {
    pub left_stick_x: f32,
    pub left_stick_y: f32,
    pub left_z: f32,
    pub right_stick_x: f32,
    pub right_stick_y: f32,
    pub right_z: f32,
    pub d_pad_x: f32,
    pub d_pad_y: f32,
}

#[derive(Default, Debug)]
pub struct Buttons {
    pub south: bool,
    pub east: bool,
    pub north: bool,
    pub west: bool,
    pub c: bool,
    pub z: bool,
    pub left_trigger: bool,
    pub left_trigger2: bool,
    pub right_trigger: bool,
    pub right_trigger2: bool,
    pub select: bool,
    pub start: bool,
    pub mode: bool,
    pub left_thumb: bool,
    pub right_thumb: bool,
    pub d_pad_up: bool,
    pub d_pad_down: bool,
    pub d_pad_left: bool,
    pub d_pad_right: bool,
}

impl Gamepad {
    pub fn check_if_konami_code_is_entered(&self) -> bool {
        // it's a ringbuffer- allow any rotations!
        (0..KONAMI_CODE.len()).all(|i| {
            let wrapped_i = (self.button_history_index + i) % self.button_history.len();
            self.button_history[wrapped_i] == KONAMI_CODE[i]
        })
    }

    #[cfg(debug_assertions)]
    pub fn enter_konami_code(&mut self) {
        self.button_history = KONAMI_CODE;
    }

    pub fn set_button_value(&mut self, button: Button, value: bool) {
        match button {
            Button::South => self.buttons.south = value,
            Button::East => self.buttons.east = value,
            Button::North => self.buttons.north = value,
            Button::West => self.buttons.west = value,
            Button::C => self.buttons.c = value,
            Button::Z => self.buttons.z = value,
            Button::LeftTrigger => self.buttons.left_trigger = value,
            Button::LeftTrigger2 => self.buttons.left_trigger2 = value,
            Button::RightTrigger => self.buttons.right_trigger = value,
            Button::RightTrigger2 => self.buttons.right_trigger2 = value,
            Button::Select => self.buttons.select = value,
            Button::Start => self.buttons.start = value,
            Button::Mode => self.buttons.mode = value,
            Button::LeftThumb => self.buttons.left_thumb = value,
            Button::RightThumb => self.buttons.right_thumb = value,
            Button::DPadUp => self.buttons.d_pad_up = value,
            Button::DPadDown => self.buttons.d_pad_down = value,
            Button::DPadLeft => self.buttons.d_pad_left = value,
            Button::DPadRight => self.buttons.d_pad_right = value,
            Button::Unknown => {}
        }

        if value {
            self.button_history[self.button_history_index] = button;
            self.button_history_index = (self.button_history_index + 1) % self.button_history.len();
        }
    }

    pub fn set_axis_value(&mut self, axis: gilrs::Axis, value: f32) {
        match axis {
            gilrs::Axis::LeftStickX => self.axises.left_stick_x = value,
            gilrs::Axis::LeftStickY => self.axises.left_stick_y = value,
            gilrs::Axis::LeftZ => self.axises.left_z = value,
            gilrs::Axis::RightStickX => self.axises.right_stick_x = value,
            gilrs::Axis::RightStickY => self.axises.right_stick_y = value,
            gilrs::Axis::RightZ => self.axises.right_z = value,
            gilrs::Axis::DPadX => self.axises.d_pad_x = value,
            gilrs::Axis::DPadY => self.axises.d_pad_y = value,
            gilrs::Axis::Unknown => {}
        }
    }
}
