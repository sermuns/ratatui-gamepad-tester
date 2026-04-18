use gilrs::Gilrs;
use ratatui::{
    DefaultTerminal,
    crossterm::{
        self,
        event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    },
    prelude::*,
    symbols::Marker,
    widgets::{
        Block, Clear, Padding, Paragraph,
        canvas::{Canvas, Circle, Line as CLine, Rectangle},
    },
};
use std::{io, time::Duration};
use tui_widgets::big_text::{BigText, PixelSize};

use crate::gamepad::Gamepad;

pub struct App {
    gilrs: Gilrs,
    running: bool,
    gamepad: Gamepad,
    show_debug_info: bool,
    force_feedback: bool,
}

impl App {
    pub fn new() -> Self {
        let gamepad = Gamepad::default();

        Self {
            gilrs: Gilrs::new().unwrap(),
            running: true,
            gamepad,
            show_debug_info: false,
            force_feedback: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|terminal| self.draw(terminal))?;
            self.handle_crossterm_events()?;
            self.handle_gamepad_events();

            if self.force_feedback {}

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
        let Some(gilrs::Event { id, event, .. }) = self.gilrs.next_event() else {
            return;
        };

        self.gamepad.id = Some(id);

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
            KeyCode::Char('f') => self.force_feedback = !self.force_feedback,
            #[cfg(debug_assertions)]
            KeyCode::Char('k') => self.gamepad.enter_konami_code(),
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

        const GAMEPAD_AREA_ASPECT_RATIO: f32 = 4.8; // source: it was revealed to me in a dream
        let mut gamepad_area = block.inner(area);

        let target_height = (gamepad_area.width as f32 / GAMEPAD_AREA_ASPECT_RATIO) as u16;

        if target_height <= gamepad_area.height {
            gamepad_area.y += (gamepad_area.height - target_height) / 2;
            gamepad_area.height = target_height;
        } else {
            let target_width = (gamepad_area.height as f32 * GAMEPAD_AREA_ASPECT_RATIO) as u16;
            gamepad_area.x += (gamepad_area.width - target_width) / 2;
            gamepad_area.width = target_width;
        }

        Canvas::default()
            .x_bounds([-45., 45.])
            .y_bounds([-13., 25.])
            .marker(Marker::Octant)
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
                    -7.,
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
                    4.,
                    5.,
                    3.,
                    1.5,
                    if self.gamepad.buttons.start {
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
                    if self.gamepad.buttons.mode {
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
                left_joystick.x += JOYSTICK_MOVEMENT * self.gamepad.axises.left_stick_x as f64;
                left_joystick.y += JOYSTICK_MOVEMENT * self.gamepad.axises.left_stick_y as f64;
                left_joystick.radius = JOYSTICK_INNER_RADIUS;
                left_joystick.color = if self.gamepad.buttons.left_thumb {
                    Color::Yellow
                } else if self.gamepad.left_stick_active() {
                    Color::Cyan
                } else {
                    Color::White
                };
                ctx.draw(&left_joystick);
                if self.gamepad.buttons.left_thumb {
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
                right_joystick.x += JOYSTICK_MOVEMENT * self.gamepad.axises.right_stick_x as f64;
                right_joystick.y += JOYSTICK_MOVEMENT * self.gamepad.axises.right_stick_y as f64;
                right_joystick.radius = JOYSTICK_INNER_RADIUS;
                right_joystick.color = if self.gamepad.buttons.right_thumb {
                    Color::Yellow
                } else if self.gamepad.right_stick_active() {
                    Color::Cyan
                } else {
                    Color::White
                };
                ctx.draw(&right_joystick);
                if self.gamepad.buttons.right_thumb {
                    right_joystick.radius = 1.;
                    ctx.draw(&right_joystick);
                }
            })
            .render(gamepad_area, buf);

        if self.gamepad.check_if_konami_code_is_entered() {
            let big_text = BigText::builder()
                .lines(["KONAMI CODE ENTERED!".into()])
                .pixel_size(PixelSize::Sextant)
                .block(Block::bordered().padding(Padding::uniform(1)))
                .style(Style::default().green())
                .centered()
                .build();
            let big_text_area = area.centered_vertically(Constraint::Length(7));
            Clear.render(big_text_area, buf);
            big_text.render(big_text_area, buf);
        }

        block.render(area, buf);
    }
}
