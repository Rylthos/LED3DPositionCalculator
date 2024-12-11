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

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
};
use std::thread;

use crate::colour::Colour;
use crate::led_controller::PixelController;

#[derive(Debug)]
pub struct App {
    conn: Arc<RwLock<connection::DDPConnection>>,
    controller: Arc<RwLock<PixelController>>,
    thread_alive: Arc<AtomicBool>,
    transmit_handle: Option<thread::JoinHandle<()>>,
    controller_handle: Option<thread::JoinHandle<()>>,
    update_ms: u64,
    exit: bool,
}

impl App {
    pub fn new(ip: &str, pixel_count: usize, update_ms: u64) -> App {
        let conn = Arc::new(RwLock::new(
            connection::DDPConnection::try_new(
                ip,
                protocol::PixelConfig::default(),
                protocol::ID::Default,
                std::net::UdpSocket::bind("0.0.0.0:4048").unwrap(),
            )
            .unwrap(),
        ));

        let controller = Arc::new(RwLock::new(PixelController::new(pixel_count)));

        App {
            conn,
            controller,
            thread_alive: Arc::new(AtomicBool::new(false)),
            transmit_handle: None,
            controller_handle: None,
            update_ms,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.start_transmit_thread();
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
            KeyCode::Left => {
                let mut controller = self.controller.write().unwrap();
                controller.prev_effect();
            }
            KeyCode::Right => {
                let mut controller = self.controller.write().unwrap();
                controller.next_effect();
            }
            _ => {}
        }
    }

    fn start_transmit_thread(&mut self) {
        self.thread_alive.store(true, Ordering::SeqCst);

        let transmit_alive = self.thread_alive.clone();
        let transmit_controller = self.controller.clone();
        let transmit_conn = self.conn.clone();

        let transmit_ms = self.update_ms;

        self.transmit_handle = Some(thread::spawn(move || {
            while transmit_alive.load(Ordering::SeqCst) {
                {
                    let controller = transmit_controller.read().unwrap();
                    let mut connection = transmit_conn.write().unwrap();

                    controller.transmit(&mut connection);
                }
                thread::sleep(std::time::Duration::from_millis(transmit_ms));
            }
        }));

        let controller_alive = self.thread_alive.clone();
        let controller = self.controller.clone();

        let update_time = self.update_ms;

        self.controller_handle = Some(thread::spawn(move || {
            while controller_alive.load(Ordering::SeqCst) {
                {
                    let mut controller = controller.write().unwrap();

                    controller.update((update_time as f32) / 1000.);
                }

                thread::sleep(std::time::Duration::from_millis(update_time));
            }
        }));
    }

    fn exit(&mut self) {
        self.thread_alive.store(false, Ordering::SeqCst);
        self.transmit_handle
            .take()
            .expect("Called stop on non_running thread")
            .join()
            .expect("Could not join spawned thread");

        self.controller_handle
            .take()
            .expect("Called stop on non_running thread")
            .join()
            .expect("Could not join spawned thread");

        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Previous Effect ".into(),
            "<Left>".blue().bold(),
            " Next Effect ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let controller = self.controller.read().unwrap();
        let cur_eff = controller.get_current_effect();
        let str = cur_eff.to_string();

        let counter_text = Text::from(vec![
            Line::from(vec!["Current Effect:".into()]),
            Line::from(vec![str.into()]),
        ]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
