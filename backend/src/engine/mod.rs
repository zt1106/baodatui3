use futures_util::future::BoxFuture;
use crate::model::game::Game;

pub trait GameEngine {
    fn run_game(&self, game: Game) -> BoxFuture<()>;
}

struct GameEngineImpl {}

struct GameEngineTestImpl {}