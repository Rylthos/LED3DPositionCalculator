use std::io;

use ddp_rs::connection;
use ddp_rs::protocol;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use crate::PixelController;

#[derive(Debug)]
pub struct App {
    conn: Arc<Mutex<connection::DDPConnection>>,
    controller: Arc<Mutex<PixelController>>,
    thread_exit: Arc<Mutex<bool>>,
    transmit_time: u64,
    // controller_handle: thread::JoinHandle<()>,
    exit: bool,
}

impl App {
    pub fn new(ip: &str, pixel_count: usize, transmit_time: u64) -> App {
        App {
            conn: Arc::new(Mutex::new(
                connection::DDPConnection::try_new(
                    ip,
                    protocol::PixelConfig::default(),
                    protocol::ID::Default,
                    std::net::UdpSocket::bind("0.0.0.0:4048").unwrap(),
                )
                .unwrap(),
            )),
            controller: Arc::new(Mutex::new(PixelController::new(pixel_count))),
            thread_exit: Arc::new(Mutex::new(false)),
            transmit_time,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn start_transmit(&mut self) {
        let thread_conn = self.conn.clone();
        let thread_controller = self.controller.clone();
        let thread_exit = self.thread_exit.clone();
        let transmit_time = self.transmit_time;

        thread::spawn(move || loop {
            if let Ok(running) = thread_exit.lock() {
                if !(*running) {
                    break;
                }
            }

            if let Ok(mut controller) = thread_controller.lock() {
                if let Ok(mut conn) = thread_conn.lock() {
                    controller.transmit(&mut conn);
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(transmit_time));
        });
        todo!("Fix this");
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        // self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        // self.counter -= 1;
    }
}

impl Widget for &App {
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

        let value = if let Ok(controller) = self.controller.lock() {
            controller.get_num_pixels()
        } else {
            0
        };

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            value.to_string().yellow(),
            // self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
