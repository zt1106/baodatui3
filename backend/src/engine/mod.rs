use crate::model::game::Game;
use futures_util::future::BoxFuture;

pub trait GameEngine {
    fn run_game(&self, game: Game) -> BoxFuture<()>;
}

struct GameEngineImpl {}

struct GameEngineTestImpl {}
