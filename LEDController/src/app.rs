use std::io;

use ddp_rs::connection;
use ddp_rs::protocol;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph},
    DefaultTerminal, Frame,
};

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
};
use std::thread;
use std::time::{Duration, Instant};

use crate::led_controller::PixelController;

#[derive(PartialEq)]
enum CurrentScreen {
    MainView,
    Exiting,
}

pub struct App {
    conn: Arc<RwLock<connection::DDPConnection>>,
    controller: Arc<RwLock<PixelController>>,
    thread_alive: Arc<AtomicBool>,
    transmit_handle: Option<thread::JoinHandle<()>>,
    controller_handle: Option<thread::JoinHandle<()>>,
    enabled: Arc<AtomicBool>,
    update_ms: u64,
    current_screen: CurrentScreen,
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
            enabled: Arc::new(AtomicBool::new(true)),
            update_ms,
            current_screen: CurrentScreen::MainView,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.start_transmit_thread();
        let tick_rate = Duration::from_millis(self.update_ms);
        let mut last_tick = Instant::now();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                self.handle_events()?;
            }

            last_tick = Instant::now();
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let display = Layout::default()
            .constraints([Constraint::Length(5), Constraint::Min(1)])
            .split(frame.area());

        let header = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(display[0]);

        self.draw_effect(frame, header[0]);
        self.draw_title(frame, header[1]);
        self.draw_brightness(frame, header[2]);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::THICK)
            .style(Style::default());

        frame.render_widget(block, display[1]);

        {
            let controller = self.controller.read().unwrap();
            let current_effect = controller.get_current_effect();

            current_effect.draw(frame, display[1]);
        }

        if self.current_screen == CurrentScreen::Exiting {
            self.draw_exit(frame, display[1]);
        }
    }

    fn draw_effect(&self, frame: &mut Frame, layout: Rect) {
        let controller = self.controller.read().unwrap();
        let current_effect = controller.get_current_effect();
        let current_effect_str = current_effect.to_string();

        let effect_block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::THICK)
            .style(Style::default());

        let effect = Paragraph::new(vec![
            Line::from(Span::styled(
                "Current Effect:",
                Style::default().fg(Color::Green),
            ))
            .centered(),
            Line::from(Span::styled(
                current_effect_str,
                Style::default().fg(Color::White),
            ))
            .centered(),
            Line::from(vec![
                Span::styled("<LEFT>", Style::default().fg(Color::Red)),
                Span::raw("  "),
                Span::styled("<RIGHT>", Style::default().fg(Color::Green)),
            ])
            .centered(),
        ])
        .bold()
        .centered()
        .block(effect_block);

        frame.render_widget(effect, layout);
    }

    fn draw_title(&self, frame: &mut Frame, layout: Rect) {
        let title_block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::THICK)
            .style(Style::default());

        let title = Paragraph::new(vec![
            Line::from(Span::styled(
                " LED Controller ",
                Style::default().fg(Color::Green),
            )),
            Line::from(Span::raw("")),
            if self.enabled.load(Ordering::SeqCst) {
                Line::from(Span::styled(
                    "(e) Enabled ",
                    Style::default().fg(Color::Green),
                ))
                .centered()
            } else {
                Line::from(Span::styled(
                    "(e) Disabled",
                    Style::default().fg(Color::Red),
                ))
                .centered()
            },
        ])
        .bold()
        .centered()
        .block(title_block);

        frame.render_widget(title, layout);
    }

    fn draw_brightness(&self, frame: &mut Frame, layout: Rect) {
        let controller = self.controller.read().unwrap();

        let brightness_block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::THICK)
            .style(Style::default());

        let brightness_text = vec![
            Line::from(Span::styled(
                "Brightness",
                Style::default().fg(Color::White),
            )),
            Line::from(format!("{:.2}", controller.get_brightness())),
            Line::from(vec![
                Span::raw("<"),
                Span::styled("-", Style::default().fg(Color::Red)),
                Span::raw("  "),
                Span::styled("+", Style::default().fg(Color::Green)),
                Span::raw(">"),
            ]),
        ];

        let brightness = Paragraph::new(brightness_text)
            .centered()
            .block(brightness_block);

        frame.render_widget(brightness, layout);
    }

    fn draw_exit(&self, frame: &mut Frame, layout: Rect) {
        let percent_x = 40;
        let percent_y = 30;
        let middle_third = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(layout)[1];

        let center = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(middle_third)[1];

        frame.render_widget(Clear, center);

        let block = Block::default()
            .title(Line::from("y/N").centered())
            .borders(Borders::ALL)
            .style(Style::default());

        let block_text = Text::styled(
            "Are you sure you want to exit?",
            Style::default().fg(Color::White),
        )
        .centered();

        let paragraph = Paragraph::new(block_text).centered().block(block);

        frame.render_widget(paragraph, center);
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
        match self.current_screen {
            CurrentScreen::Exiting => match key_event.code {
                KeyCode::Char('q') | KeyCode::Char('y') | KeyCode::Char('Y') => self.exit(),
                _ => self.current_screen = CurrentScreen::MainView,
            },
            CurrentScreen::MainView => match key_event.code {
                KeyCode::Char('q') => self.current_screen = CurrentScreen::Exiting,
                KeyCode::Left => {
                    let mut controller = self.controller.write().unwrap();
                    controller.prev_effect();
                }
                KeyCode::Right => {
                    let mut controller = self.controller.write().unwrap();
                    controller.next_effect();
                }
                KeyCode::Char('+') | KeyCode::Char('=') => {
                    let mut controller = self.controller.write().unwrap();
                    controller.increase_brightness();
                }
                KeyCode::Char('-') | KeyCode::Char('_') => {
                    let mut controller = self.controller.write().unwrap();
                    controller.decrease_brightness();
                }
                KeyCode::Char('e') => {
                    let new_enabled = !self.enabled.load(Ordering::SeqCst);
                    self.enabled.store(new_enabled, Ordering::SeqCst);
                }
                _ => {
                    self.controller
                        .write()
                        .unwrap()
                        .get_current_effect_mut()
                        .handle_input(key_event);
                }
            },
        }
    }

    fn start_transmit_thread(&mut self) {
        self.thread_alive.store(true, Ordering::SeqCst);

        let transmit_alive = self.thread_alive.clone();
        let transmit_controller = self.controller.clone();
        let transmit_conn = self.conn.clone();
        let transmit_enabled = self.enabled.clone();

        let transmit_ms = self.update_ms;

        self.transmit_handle = Some(thread::spawn(move || {
            let tick_rate = Duration::from_millis(transmit_ms);
            let mut last_tick = Instant::now();
            while transmit_alive.load(Ordering::SeqCst) {
                {
                    if last_tick.elapsed() >= tick_rate {
                        if !transmit_enabled.load(Ordering::SeqCst) {
                            continue;
                        }
                        let controller = transmit_controller.read().unwrap();
                        let mut connection = transmit_conn.write().unwrap();

                        controller.transmit(&mut connection);

                        last_tick = Instant::now();
                    }
                }
                thread::sleep(std::time::Duration::from_millis(transmit_ms));
            }
        }));

        let controller_alive = self.thread_alive.clone();
        let controller = self.controller.clone();
        let update_enabled = self.enabled.clone();
        let update_time = self.update_ms;

        self.controller_handle = Some(thread::spawn(move || {
            let tick_rate = Duration::from_millis(update_time);
            let mut last_tick = Instant::now();

            while controller_alive.load(Ordering::SeqCst) {
                {
                    if last_tick.elapsed() >= tick_rate {
                        if !update_enabled.load(Ordering::SeqCst) {
                            continue;
                        }
                        let mut controller = controller.write().unwrap();

                        controller.update((update_time as f32) / 1000.);

                        last_tick = Instant::now();
                    }
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
