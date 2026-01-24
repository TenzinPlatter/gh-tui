use anyhow::Result;
use ratatui::{DefaultTerminal, Frame, widgets::WidgetRef};
use tokio::sync::mpsc;

use crate::{
    api::ApiClient, cache::Cache, config::Config, view::View
};

pub mod cmd;
pub mod events;
pub mod handlers;
pub mod init;
pub mod model;
pub mod msg;
pub mod pane;
pub mod update;
pub mod view;

pub use events::AppEvent;

pub struct App {
    pub model: model::Model,
    pub exit: bool,
    pub receiver: mpsc::UnboundedReceiver<msg::Msg>,
    pub sender: mpsc::UnboundedSender<msg::Msg>,
    pub api_client: ApiClient,
    #[allow(dead_code)]
    pub view: View,
    #[allow(dead_code)]
    pub config: Config,
    #[allow(dead_code)]
    pub cache: Cache,
}

impl App {
    pub async fn main_loop(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events(self.sender.clone(), terminal).await?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        self.view.render_ref(frame.area(), frame.buffer_mut());
    }
}
