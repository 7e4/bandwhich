use ::tui::backend::Backend;
use ::tui::layout::{Alignment, Rect};
use ::tui::style::{Color, Modifier, Style};
use ::tui::terminal::Frame;
use ::tui::widgets::{Paragraph, Text, Widget};

use crate::display::{DisplayBandwidth, UIState};

const SECONDS_IN_DAY: u64 = 86400;

pub struct HeaderDetails<'a> {
    pub state: &'a UIState,
    pub elapsed_time: std::time::Duration,
    pub paused: bool,
}

impl<'a> HeaderDetails<'a> {
    pub fn render(&self, frame: &mut Frame<impl Backend>, rect: Rect) {
        let bandwidth = self.bandwidth_string();
        let elapsed_time = self.elapsed_time_string();
        let print_elapsed_time = bandwidth.len() + elapsed_time.len() + 1 <= rect.width as usize;

        let color = if self.paused {
            Color::Yellow
        } else {
            Color::Green
        };

        if print_elapsed_time {
            self.render_elapsed_time(frame, rect, &color);
        }
        self.render_bandwidth(frame, rect, &color);
    }

    fn render_bandwidth(&self, frame: &mut Frame<impl Backend>, rect: Rect, color: &Color) {
        let bandwidth_text = {
            [Text::styled(
                self.bandwidth_string(),
                Style::default().fg(*color).modifier(Modifier::BOLD),
            )]
        };

        Paragraph::new(bandwidth_text.iter())
            .alignment(Alignment::Left)
            .render(frame, rect);
    }

    fn bandwidth_string(&self) -> String {
        let c_mode = self.state.cumulative_mode;
        format!(
            " Total Up / Down: {} / {}{}",
            DisplayBandwidth {
                bandwidth: self.state.total_bytes_uploaded as f64,
                as_rate: !c_mode,
            },
            DisplayBandwidth {
                bandwidth: self.state.total_bytes_downloaded as f64,
                as_rate: !c_mode,
            },
            if self.paused { " [PAUSED]" } else { "" }
        )
    }

    fn render_elapsed_time(&self, frame: &mut Frame<impl Backend>, rect: Rect, color: &Color) {
        let elapsed_time_text = [Text::styled(
            self.elapsed_time_string(),
            Style::default().fg(*color).modifier(Modifier::BOLD),
        )];
        Paragraph::new(elapsed_time_text.iter())
            .alignment(Alignment::Right)
            .render(frame, rect);
    }

    fn days_string(&self) -> String {
        match self.elapsed_time.as_secs() / SECONDS_IN_DAY {
            0 => "".to_string(),
            1 => "1 day, ".to_string(),
            n => format!("{} days, ", n)
        }
    }

    fn elapsed_time_string(&self) -> String {
        format!(
            "{}{:02}:{:02}:{:02} ",
            self.days_string(),
            (self.elapsed_time.as_secs() % SECONDS_IN_DAY) / 3600,
            (self.elapsed_time.as_secs() % 3600) / 60,
            self.elapsed_time.as_secs() % 60
        )
    }
}
