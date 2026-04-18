use std::time::Instant;

use gilrs::{Button, GamepadId};
use ratatui::{
    prelude::*,
    widgets::canvas::{self, Circle, Line as CLine, Rectangle},
};

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
    pub id: Option<GamepadId>,
    pub last_seen: Option<Instant>,
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

    pub fn paint_to_canvas(&self, ctx: &mut canvas::Context) {
        const CONTROLLER_TOP_Y: f64 = 12.;
        const CONTROLLER_TOP_X: f64 = 23.;
        const CONTROLLER_BOTTOM_Y: f64 = -5.;
        const CONTROLLER_BOTTOM_X: f64 = 15.;
        const HANDLE_BOTTOM_Y: f64 = -10.;
        const HANDLE_BOTTOM_X: f64 = 30.;

        // top
        ctx.draw(&CLine::new(
            -CONTROLLER_TOP_X,
            CONTROLLER_TOP_Y,
            CONTROLLER_TOP_X,
            CONTROLLER_TOP_Y,
            Color::DarkGray,
        ));
        // bottom
        ctx.draw(&CLine::new(
            -CONTROLLER_BOTTOM_X,
            CONTROLLER_BOTTOM_Y,
            CONTROLLER_BOTTOM_X,
            CONTROLLER_BOTTOM_Y,
            Color::DarkGray,
        ));
        // left handle
        ctx.draw(&CLine::new(
            -CONTROLLER_TOP_X,
            CONTROLLER_TOP_Y,
            -HANDLE_BOTTOM_X,
            HANDLE_BOTTOM_Y,
            Color::DarkGray,
        ));
        ctx.draw(&CLine::new(
            -CONTROLLER_BOTTOM_X,
            CONTROLLER_BOTTOM_Y,
            -HANDLE_BOTTOM_X,
            HANDLE_BOTTOM_Y,
            Color::DarkGray,
        ));
        // right handle
        ctx.draw(&CLine::new(
            CONTROLLER_TOP_X,
            CONTROLLER_TOP_Y,
            HANDLE_BOTTOM_X,
            HANDLE_BOTTOM_Y,
            Color::DarkGray,
        ));
        ctx.draw(&CLine::new(
            CONTROLLER_BOTTOM_X,
            CONTROLLER_BOTTOM_Y,
            HANDLE_BOTTOM_X,
            HANDLE_BOTTOM_Y,
            Color::DarkGray,
        ));

        // shoulder buttons
        // L1
        ctx.draw(&Rectangle::new(
            -CONTROLLER_TOP_X + 2.,
            14.,
            10.,
            2.,
            if self.buttons.left_trigger {
                Color::Yellow
            } else {
                Color::DarkGray
            },
        ));
        ctx.draw(&Rectangle::new(
            -CONTROLLER_TOP_X + 5.,
            18.,
            3.,
            5.,
            if self.buttons.left_trigger2 {
                Color::Yellow
            } else {
                Color::DarkGray
            },
        ));

        // R1
        ctx.draw(&Rectangle::new(
            CONTROLLER_TOP_X - 2. - 10.,
            14.,
            10.,
            2.,
            if self.buttons.right_trigger {
                Color::Yellow
            } else {
                Color::DarkGray
            },
        ));
        ctx.draw(&Rectangle::new(
            CONTROLLER_TOP_X - 5. - 3.,
            18.,
            3.,
            5.,
            if self.buttons.right_trigger2 {
                Color::Yellow
            } else {
                Color::DarkGray
            },
        ));

        // d-pad
        ctx.draw(&Circle::new(
            -17.5,
            8.5,
            1.1,
            if self.buttons.d_pad_up {
                Color::Yellow
            } else {
                Color::DarkGray
            },
        ));
        ctx.draw(&Circle::new(
            -15.,
            6.,
            1.1,
            if self.buttons.d_pad_right {
                Color::Yellow
            } else {
                Color::DarkGray
            },
        ));
        ctx.draw(&Circle::new(
            -20.,
            6.,
            1.1,
            if self.buttons.d_pad_left {
                Color::Yellow
            } else {
                Color::DarkGray
            },
        ));
        ctx.draw(&Circle::new(
            -17.5,
            3.5,
            1.1,
            if self.buttons.d_pad_down {
                Color::Yellow
            } else {
                Color::DarkGray
            },
        ));

        // action pad
        ctx.draw(&Circle::new(
            17.5,
            8.5,
            1.1,
            if self.buttons.north {
                Color::Yellow
            } else {
                Color::DarkGray
            },
        ));
        ctx.draw(&Circle::new(
            15.,
            6.,
            1.1,
            if self.buttons.west {
                Color::Yellow
            } else {
                Color::DarkGray
            },
        ));
        ctx.draw(&Circle::new(
            20.,
            6.,
            1.1,
            if self.buttons.east {
                Color::Yellow
            } else {
                Color::DarkGray
            },
        ));
        ctx.draw(&Circle::new(
            17.5,
            3.5,
            1.1,
            if self.buttons.south {
                Color::Yellow
            } else {
                Color::DarkGray
            },
        ));

        // start/select
        ctx.draw(&Rectangle::new(
            -7.,
            5.,
            3.,
            1.5,
            if self.buttons.select {
                Color::Yellow
            } else {
                Color::DarkGray
            },
        ));
        ctx.draw(&Rectangle::new(
            4.,
            5.,
            3.,
            1.5,
            if self.buttons.start {
                Color::Yellow
            } else {
                Color::DarkGray
            },
        ));

        // mode (center button, ps-button, home button etc)
        ctx.draw(&Circle::new(
            0.,
            4.,
            1.1,
            if self.buttons.mode {
                Color::Yellow
            } else {
                Color::DarkGray
            },
        ));

        const JOYSTICK_OUTER_RADIUS: f64 = 3.;
        const JOYSTICK_MOVEMENT: f64 = 0.5 * JOYSTICK_OUTER_RADIUS;
        const JOYSTICK_INNER_RADIUS: f64 = 0.5 * JOYSTICK_OUTER_RADIUS;
        const JOYSTICK_X: f64 = 10.;
        const JOYSTICK_Y: f64 = 0.;

        let mut left_joystick = Circle::new(
            -JOYSTICK_X,
            JOYSTICK_Y,
            JOYSTICK_OUTER_RADIUS,
            Color::DarkGray,
        );
        ctx.draw(&left_joystick);
        left_joystick.x += JOYSTICK_MOVEMENT * self.axises.left_stick_x as f64;
        left_joystick.y += JOYSTICK_MOVEMENT * self.axises.left_stick_y as f64;
        left_joystick.radius = JOYSTICK_INNER_RADIUS;
        left_joystick.color = if self.buttons.left_thumb {
            Color::Yellow
        } else if self.left_stick_active() {
            Color::Cyan
        } else {
            Color::White
        };
        ctx.draw(&left_joystick);
        if self.buttons.left_thumb {
            left_joystick.radius = 1.;
            ctx.draw(&left_joystick);
        }

        let mut right_joystick = Circle::new(
            JOYSTICK_X,
            JOYSTICK_Y,
            JOYSTICK_OUTER_RADIUS,
            Color::DarkGray,
        );
        ctx.draw(&right_joystick);
        right_joystick.x += JOYSTICK_MOVEMENT * self.axises.right_stick_x as f64;
        right_joystick.y += JOYSTICK_MOVEMENT * self.axises.right_stick_y as f64;
        right_joystick.radius = JOYSTICK_INNER_RADIUS;
        right_joystick.color = if self.buttons.right_thumb {
            Color::Yellow
        } else if self.right_stick_active() {
            Color::Cyan
        } else {
            Color::White
        };
        ctx.draw(&right_joystick);
        if self.buttons.right_thumb {
            right_joystick.radius = 1.;
            ctx.draw(&right_joystick);
        }
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
        self.button_history_index = 0;
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
