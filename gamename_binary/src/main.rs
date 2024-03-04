use ggez::{event, graphics, Context, GameResult};

struct MainState {
    player: gamename_engine::entity::Player,
}
impl MainState {
    fn new() -> Self {
        MainState {
            player: gamename_engine::entity::Player::new(500.0, 500.0),
        }
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const UPDATE_FPS: u32 = 60;
        // todo!() consume remaining tick and ease it (linear approx.)
        while ctx.time.check_update_time(UPDATE_FPS) {
            self.player.update(1.0 / UPDATE_FPS as f32);
            // todo!()
            // println!("{:?}", ctx.gfx.drawable_size());
            println!("{:?}", self.player.e.view_position());
            println!("{:?}", self.player.view_velocity());
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // todo!(): resolution resolving / abstracted canvas / fullscreen / settings
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        let mut mb = graphics::MeshBuilder::new();
        mb.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 100.0, 100.0),
            graphics::Color::WHITE,
        )?;
        let p_square = graphics::Mesh::from_data(ctx, mb.build());
        self.player.e.drawable = Some(p_square);
        self.player.e.draw(&mut canvas);

        canvas.finish(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        repeated: bool,
    ) -> GameResult {
        use gamename_engine::entity::Moveable;
        use ggez::input::keyboard::KeyCode;

        match input.keycode {
            Some(KeyCode::Escape) => ctx.request_quit(),
            Some(KeyCode::W) => self.player.up(),
            Some(KeyCode::A) => self.player.left(),
            Some(KeyCode::S) => self.player.down(),
            Some(KeyCode::D) => self.player.right(),
            _ => {}
        }

        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
    ) -> GameResult {
        use gamename_engine::entity::Moveable;
        use ggez::input::keyboard::KeyCode;

        match input.keycode {
            Some(KeyCode::W) => self.player.stop_up(),
            Some(KeyCode::A) => self.player.stop_left(),
            Some(KeyCode::S) => self.player.stop_down(),
            Some(KeyCode::D) => self.player.stop_right(),
            _ => {}
        }

        Ok(())
    }
}

fn main() -> GameResult {
    // todo()! resources later

    let cb = gamename_engine::setup::make_cb();
    let (mut ctx, event_loop) = cb.build()?;
    // ctx.gfx.supported_resolutions().for_each(|ps| println!("{ps:?}"));
    println!("{}", ctx.gfx.window().scale_factor());
    let state = MainState::new();
    event::run(ctx, event_loop, state)
}
