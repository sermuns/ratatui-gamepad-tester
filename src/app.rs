use gilrs::Gilrs;
use ratatui::{
    DefaultTerminal,
    crossterm::{
        self,
        event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    },
    prelude::*,
    symbols::Marker,
    text::ToLine,
    widgets::{
        Block, Clear, Padding, Paragraph,
        canvas::{self, Canvas, Circle, Line as CLine, Rectangle},
    },
};
use std::{
    io,
    time::{Duration, Instant},
};
use tui_widgets::big_text::{BigText, PixelSize};

use crate::gamepad::Gamepad;

pub struct App {
    gilrs: Gilrs,
    running: bool,
    gamepad: Gamepad,
    show_debug_info: bool,
    force_feedback: bool,
}

const GAMEPAD_INACTIVITY_TIMEOUT: Duration = Duration::from_secs(5);

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
        const REDRAW_INTERVAL: Duration = Duration::from_millis(5); // source: it was revealed to me in a dream
        let mut draw_instant: Instant = Instant::now();

        loop {
            self.handle_crossterm_events()?;
            self.handle_gamepad_events();

            // TODO:
            if self.force_feedback {}

            if draw_instant.elapsed() >= REDRAW_INTERVAL {
                terminal.draw(|terminal| self.draw(terminal))?;
                draw_instant = Instant::now();
            }

            if !self.running {
                break Ok(());
            }
        }
    }

    fn handle_crossterm_events(&mut self) -> io::Result<()> {
        use crossterm::event::Event as CrosstermEvent;

        if !crossterm::event::poll(Duration::from_millis(1))? {
            if self.gamepad.id.is_some()
                && self
                    .gamepad
                    .last_seen
                    .is_some_and(|last_seen| last_seen.elapsed() > GAMEPAD_INACTIVITY_TIMEOUT)
            {
                self.gamepad.id = None;
            }
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
        self.gamepad.last_seen = Some(Instant::now());

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
            .title_bottom(" quit: 'q', debug info: 'd' ".to_line().centered())
            .padding(Padding::horizontal(2));

        let details_lines = if let Some(id) = self.gamepad.id {
            let gamepad = self.gilrs.gamepad(id);
            vec![
                Line::from(vec!["Name: ".bold(), gamepad.name().to_string().into()]),
                Line::from(vec![
                    "Power Info: ".bold(),
                    format!("{:?}", gamepad.power_info()).into(),
                ]),
                Line::from(vec![
                    "Is Connected: ".bold(),
                    gamepad.is_connected().to_string().into(),
                ]),
            ]
        } else if self.gamepad.last_seen.is_some() {
            vec![
                format!(
                    "No gamepad input detected in the last {} seconds.",
                    GAMEPAD_INACTIVITY_TIMEOUT.as_secs()
                )
                .into(),
            ]
        } else {
            vec!["No gamepad detected.".into()]
        };

        let [top_area, details_area] = block.inner(area).layout(&Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(details_lines.len() as u16),
        ]));

        Paragraph::new(details_lines).render(details_area, buf);

        if self.show_debug_info {
            Paragraph::new(format!("{:#?}", self.gamepad)).render(top_area, buf);
        } else {
            let mut gamepad_area = top_area.inner(Margin::new(10, 5));
            const GAMEPAD_AREA_ASPECT_RATIO: f32 = 3.5; // source: it was revealed to me in a dream
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
                .x_bounds([-30., 30.])
                .y_bounds([-10., 23.])
                .marker(Marker::Octant)
                .paint(|ctx| self.gamepad.paint_to_canvas(ctx))
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
        }

        block.render(area, buf);
    }
}
