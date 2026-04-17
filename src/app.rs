use gilrs::{Button, Gilrs};
use ratatui::{
    DefaultTerminal,
    crossterm::{
        self,
        event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    },
    prelude::*,
    widgets::{
        Block, Paragraph,
        canvas::{Canvas, Circle, Rectangle},
    },
};
use std::{io, time::Duration};

use crate::gamepad::ActiveGamepad;

pub struct App {
    gilrs: Gilrs,
    running: bool,
    gamepad: ActiveGamepad,
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
                ctx.draw(&Rectangle::new(-20., -5., 40., 12., Color::DarkGray));

                let mut left_joystick = Circle::new(-10., 0., 3., Color::DarkGray);
                ctx.draw(&left_joystick);
                left_joystick.x += 2. * self.gamepad.axises.left_stick_x as f64;
                left_joystick.y += 2. * self.gamepad.axises.left_stick_y as f64;
                left_joystick.radius = 2.;
                left_joystick.color = Color::White;
                ctx.draw(&left_joystick);

                let mut right_joystick = Circle::new(10., 0., 3., Color::DarkGray);
                ctx.draw(&right_joystick);
                right_joystick.x += 2. * self.gamepad.axises.right_stick_x as f64;
                right_joystick.y += 2. * self.gamepad.axises.right_stick_y as f64;
                right_joystick.radius = 2.;
                right_joystick.color = Color::White;
                ctx.draw(&right_joystick);
            })
            .render(area, buf);
    }
}
