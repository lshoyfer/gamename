use ggez::{
    event,
    // glam,
    Context,
    GameResult,
};

struct MainState;

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

fn main() -> GameResult {
    // todo()! resources later

    let cb = ggez::ContextBuilder::new("gamename", "lshoyfer");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState;
    event::run(ctx, event_loop, state)
}