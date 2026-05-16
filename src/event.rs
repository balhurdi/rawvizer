use crossterm::event::Event as CrosstermEvent;

use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub enum Event {
    Crossterm(CrosstermEvent),
    App(AppEvent),
}

#[derive(Debug, Clone)]
pub enum AppEvent {
    Quit,
}

pub struct EventHandler {
    sender: mpsc::UnboundedSender<Event>,
    receiver: mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let event_task = EventTask::new(sender.clone());
        tokio::spawn(async { event_task.run().await });

        Self { sender, receiver }
    }

    pub async fn next(&mut self) -> Result<Event> {
        Ok(self.receiver.recv().await.ok_or(Error::NoEvents)?)
    }

    pub fn send(&mut self, app_event: AppEvent) {
        // Ignore the result as the reciever cannot be dropped while this struct still has a
        // reference to it
        let _ = self.sender.send(Event::App(app_event));
    }
}

struct EventTask {
    sender: mpsc::UnboundedSender<Event>,
}

impl EventTask {
    fn new(sender: mpsc::UnboundedSender<Event>) -> Self {
        Self { sender }
    }

    async fn run(self) -> Result<()> {
        let mut reader = crossterm::event::EventStream::new();
        loop {
            let crossterm_event = reader.next().fuse();

            tokio::select! {
                _ = self.sender.closed() => {
                    break;
                }
                Some(Ok(evt)) = crossterm_event => {
                    self.send(Event::Crossterm(evt));
                }
            }
        }

        Ok(())
    }

    fn send(&self, event: Event) {
        let _ = self.sender.send(event);
    }
}
