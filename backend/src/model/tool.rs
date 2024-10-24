use crate::model::game::Game;

pub trait Tool {
    fn apply(self, game: &mut Game);
}

pub struct ExchangeLocationCard {}
