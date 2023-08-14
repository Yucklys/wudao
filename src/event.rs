use crate::{action::Action, app::GameResult};
use crossterm::event::{Event as CrosstermEvent, EventStream};
use futures::StreamExt;
use std::time::Duration;
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    task::JoinHandle,
    time::interval,
};

/// Terminal events.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Crossterm events.
    Crossterm(CrosstermEvent),
    /// Action dispatcher.
    Action(Action),
}

/// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    tx: Sender<Event>,
    /// Event receiver channel.
    rx: Receiver<Event>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new() -> Self {
        let (sender, receiver) = channel(100);
        Self {
            tx: sender,
            rx: receiver,
        }
    }

    pub fn start(&self, tick_rate: u64) -> GameResult<()> {
        let tick_rate = Duration::from_millis(tick_rate);
        self.spawn_tick_task(tick_rate);
        self.spawn_crossterm_task();
        Ok(())
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub async fn next(&mut self) -> Option<Event> {
        self.rx.recv().await
    }

    fn spawn_tick_task(&self, tick_rate: Duration) -> JoinHandle<()> {
        let tx = self.tx.clone();
        let mut interval = interval(tick_rate);
        tokio::spawn(async move {
            loop {
                interval.tick().await;
                if tx.send(Event::Tick).await.is_err() {
                    break;
                }
            }
        })
    }

    fn spawn_crossterm_task(&self) -> JoinHandle<()> {
        let tx = self.tx.clone();
        let mut events = EventStream::new();
        tokio::spawn(async move {
            while let Some(Ok(event)) = events.next().await {
                if tx.send(Event::Crossterm(event)).await.is_err() {
                    break;
                }
            }
        })
    }

    pub fn spawn_action_task(&self, actions: Vec<Action>) -> JoinHandle<()> {
        let tx = self.tx.clone();
        tokio::spawn(async move {
            for action in actions {
                if tx.send(Event::Action(action)).await.is_err() {
                    break;
                }
            }
        })
    }
}
