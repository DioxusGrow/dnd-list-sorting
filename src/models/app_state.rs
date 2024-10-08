use super::item::Card;
use dioxus::prelude::*;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct ApplicationData {
    pub board: Signal<Vec<Signal<Vec<Card>>>>,
}

impl ApplicationData {
    pub fn new(board: Signal<Vec<Signal<Vec<Card>>>>) -> Self {
        Self { board }
    }
}
