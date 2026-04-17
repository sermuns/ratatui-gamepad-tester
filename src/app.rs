use gilrs::{Button, Gamepad, Gilrs};
use ratatui::{
    DefaultTerminal,
    crossterm::{
        self,
        event::{KeyCode, KeyEvent, KeyEventKind},
    },
    prelude::*,
    symbols::border,
    widgets::{Block, Paragraph},
};
use std::{io, time::Duration};

// maybe insane to hardcode this..
const MAX_NUM_GAMEPADS: usize = 16;

pub struct App<'a> {
    gilrs: Gilrs,
    gamepads: [Option<Gamepad<'a>>; MAX_NUM_GAMEPADS],
    running: bool,
}

impl App<'_> {
    pub fn new() -> Self {
        Self {
            gilrs: Gilrs::new().unwrap(),
            gamepads: [None; MAX_NUM_GAMEPADS],
            running: true,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        // Iterate over all connected gamepads
        for (id, gamepad) in self.gilrs.gamepads() {
            println!("{} {} is {:?}", id, gamepad.name(), gamepad.power_info());
        }

        loop {
            self.handle_crossterm_events()?;
            self.handle_gamepad_events();
            if !self.running {
                break Ok(());
            }
        }
    }

    fn handle_crossterm_events(&mut self) -> io::Result<()> {
        use crossterm::event::Event as CrosstermEvent;

        if !crossterm::event::poll(Duration::from_millis(10))? {
            return Ok(());
        }

        match crossterm::event::read()? {
            CrosstermEvent::Key(key_event) => self.handle_key_event(key_event),
            _ => {}
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.kind != KeyEventKind::Press {
            return;
        };
        match key_event.code {
            KeyCode::Char('q') => self.running = false,
            _ => {}
        }
    }

    fn handle_gamepad_events(&mut self) {
        let Some(gilrs::Event { event, .. }) = self.gilrs.next_event() else {
            return;
        };

        println!("New event {:?}", event);
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec!["Value: ".into()])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
