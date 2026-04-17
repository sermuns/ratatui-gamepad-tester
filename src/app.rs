use gilrs::Gilrs;
use ratatui::{
    DefaultTerminal,
    crossterm::{
        self,
        event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    },
    prelude::*,
    widgets::{
        Block, Paragraph,
        canvas::{Canvas, Circle, Line as CLine, Rectangle},
    },
};
use std::{io, time::Duration};

use crate::gamepad::Gamepad;

pub struct App {
    gilrs: Gilrs,
    running: bool,
    gamepad: Gamepad,
    show_debug_info: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            gilrs: Gilrs::new().unwrap(),
            running: true,
            show_debug_info: false,
            gamepad: Default::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|terminal| self.draw(terminal))?;
            self.handle_crossterm_events()?;
            self.handle_gamepad_events();
            if !self.running {
                break Ok(());
            }
        }
    }

    fn handle_crossterm_events(&mut self) -> io::Result<()> {
        use crossterm::event::Event as CrosstermEvent;

        if !crossterm::event::poll(Duration::from_millis(1))? {
            return Ok(());
        }

        match crossterm::event::read()? {
            CrosstermEvent::Key(key_event) => self.handle_key_event(key_event),
            _ => {}
        }

        Ok(())
    }

    fn handle_gamepad_events(&mut self) {
        let Some(gilrs::Event { event, .. }) = self.gilrs.next_event() else {
            return;
        };

        match event {
            gilrs::EventType::ButtonPressed(button, ..) => {
                self.gamepad.set_button_value(button, true)
            }
            gilrs::EventType::ButtonReleased(button, ..) => {
                self.gamepad.set_button_value(button, false)
            }
            gilrs::EventType::AxisChanged(axis, value, ..) => {
                self.gamepad.set_axis_value(axis, value)
            }
            _ => {}
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.kind != KeyEventKind::Press {
            return;
        };
        match key_event.code {
            KeyCode::Char('q') => self.running = false,
            KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.running = false
            }
            KeyCode::Char('d') => self.show_debug_info = !self.show_debug_info,
            _ => {}
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(concat!(" ", env!("CARGO_PKG_NAME"), " ").bold());

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(" quit: 'q', debug info: 'd' ".dim().into_centered_line());

        if self.show_debug_info {
            Paragraph::new(format!("{:#?}", self.gamepad))
                .block(block)
                .render(area, buf);
            return;
        }

        Canvas::default()
            .block(block)
            .x_bounds([-50., 50.])
            .y_bounds([-50., 50.])
            .paint(|ctx| {
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
                    if self.gamepad.buttons.left_trigger {
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
                    if self.gamepad.buttons.left_trigger2 {
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
                    if self.gamepad.buttons.right_trigger {
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
                    if self.gamepad.buttons.right_trigger2 {
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
                    if self.gamepad.buttons.d_pad_up {
                        Color::Yellow
                    } else {
                        Color::DarkGray
                    },
                ));
                ctx.draw(&Circle::new(
                    -15.,
                    6.,
                    1.1,
                    if self.gamepad.buttons.d_pad_right {
                        Color::Yellow
                    } else {
                        Color::DarkGray
                    },
                ));
                ctx.draw(&Circle::new(
                    -20.,
                    6.,
                    1.1,
                    if self.gamepad.buttons.d_pad_left {
                        Color::Yellow
                    } else {
                        Color::DarkGray
                    },
                ));
                ctx.draw(&Circle::new(
                    -17.5,
                    3.5,
                    1.1,
                    if self.gamepad.buttons.d_pad_down {
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
                    if self.gamepad.buttons.north {
                        Color::Yellow
                    } else {
                        Color::DarkGray
                    },
                ));
                ctx.draw(&Circle::new(
                    15.,
                    6.,
                    1.1,
                    if self.gamepad.buttons.west {
                        Color::Yellow
                    } else {
                        Color::DarkGray
                    },
                ));
                ctx.draw(&Circle::new(
                    20.,
                    6.,
                    1.1,
                    if self.gamepad.buttons.east {
                        Color::Yellow
                    } else {
                        Color::DarkGray
                    },
                ));
                ctx.draw(&Circle::new(
                    17.5,
                    3.5,
                    1.1,
                    if self.gamepad.buttons.south {
                        Color::Yellow
                    } else {
                        Color::DarkGray
                    },
                ));

                // start/select
                ctx.draw(&Rectangle::new(
                    -5.,
                    5.,
                    3.,
                    1.5,
                    if self.gamepad.buttons.select {
                        Color::Yellow
                    } else {
                        Color::DarkGray
                    },
                ));
                ctx.draw(&Rectangle::new(
                    2.,
                    5.,
                    3.,
                    1.5,
                    if self.gamepad.buttons.start {
                        Color::Yellow
                    } else {
                        Color::DarkGray
                    },
                ));

                const JOYSTICK_MOVEMENT: f64 = 1.5;

                let mut left_joystick = Circle::new(-10., 0., 3., Color::DarkGray);
                ctx.draw(&left_joystick);
                left_joystick.x += JOYSTICK_MOVEMENT * self.gamepad.axises.left_stick_x as f64;
                left_joystick.y += JOYSTICK_MOVEMENT * self.gamepad.axises.left_stick_y as f64;
                left_joystick.radius = 2.;
                left_joystick.color = if self.gamepad.buttons.left_thumb {
                    Color::Yellow
                } else if self.gamepad.left_stick_active() {
                    Color::Cyan
                } else {
                    Color::White
                };
                ctx.draw(&left_joystick);

                let mut right_joystick = Circle::new(10., 0., 3., Color::DarkGray);
                ctx.draw(&right_joystick);
                right_joystick.x += JOYSTICK_MOVEMENT * self.gamepad.axises.right_stick_x as f64;
                right_joystick.y += JOYSTICK_MOVEMENT * self.gamepad.axises.right_stick_y as f64;
                right_joystick.radius = 2.;
                right_joystick.color = if self.gamepad.buttons.right_thumb {
                    Color::Yellow
                } else if self.gamepad.right_stick_active() {
                    Color::Cyan
                } else {
                    Color::White
                };
                ctx.draw(&right_joystick);
            })
            .render(area, buf);
    }
}
