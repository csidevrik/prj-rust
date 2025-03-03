use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::{io, time::Duration};

pub struct AgentTui {
    hostname: String,
    architecture: String,
    ipv4_address: String,
    mac_address: String,
    status: String,
}

impl AgentTui {
    pub fn new() -> Self {
        Self {
            hostname: String::new(),
            architecture: String::new(),
            ipv4_address: String::new(),
            mac_address: String::new(),
            status: "Iniciando...".to_string(),
        }
    }

    pub fn update_status(&mut self, status: String) {
        self.status = status;
    }

    pub fn update_info(&mut self, hostname: String, arch: String, ip: String, mac: String) {
        self.hostname = hostname;
        self.architecture = arch;
        self.ipv4_address = ip;
        self.mac_address = mac;
    }

    pub async fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let _res = self.run_app(&mut terminal).await;

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }

    async fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        _ => {}
                    }
                }
            }
        }
    }

    fn ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(10),
                Constraint::Min(0),
            ])
            .split(f.size());

        // Título
        let title = Paragraph::new("DMIG - Agente de Sistema")
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Información del sistema
        let info = vec![
            Line::from(vec![
                Span::raw("Hostname: "),
                Span::styled(&self.hostname, Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::raw("Arquitectura: "),
                Span::styled(&self.architecture, Style::default().fg(Color::Yellow)),
            ]),
            Line::from(vec![
                Span::raw("IPv4: "),
                Span::styled(&self.ipv4_address, Style::default().fg(Color::Blue)),
            ]),
            Line::from(vec![
                Span::raw("MAC: "),
                Span::styled(&self.mac_address, Style::default().fg(Color::Magenta)),
            ]),
        ];

        let system_info = Paragraph::new(info)
            .block(Block::default().title("Información del Sistema").borders(Borders::ALL));
        f.render_widget(system_info, chunks[1]);

        // Estado
        let status = Paragraph::new(self.status.as_str())
            .style(Style::default().fg(Color::White))
            .block(Block::default().title("Estado").borders(Borders::ALL));
        f.render_widget(status, chunks[2]);
    }
} 