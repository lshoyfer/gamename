pub mod setup {
    use ggez::conf;

    pub fn make_cb() -> ggez::ContextBuilder {
        let window_mode = conf::WindowMode {
            resizable: true,
            logical_size: Some(
                ggez::winit::dpi::LogicalSize::new(1280.0, 720.0), // 720p
            ),
            ..Default::default()
        };

        ggez::ContextBuilder::new("gamename", "lshoyfer")
            .window_mode(window_mode)
            .window_setup(conf::WindowSetup::default().title("gamename"))
    }
}

pub mod entity {
    use ggez::glam::Vec2;
    use ggez::graphics::{self, Drawable, Mesh};
    use ggez::mint;

    /// Base struct containing flushing and drawing tools for all Entities
    ///
    /// Revolves around a parent struct changing its position in the update portion of the
    /// eventloop, and then calling [`Entity::update()`] to flush the position
    pub struct Entity<T: Drawable> {
        /// shape, passed to `ggez`'s engine to render
        pub drawable: Option<T>,
        /// position values passed to `flush` for ggez
        position: Vec2,
        /// handed data to be passed to `ggez`'s engine to render
        flush: graphics::DrawParam,
    }

    impl<T: Drawable> Entity<T> {
        // todo!(): proper defaulting
        pub fn new() -> Self {
            Self {
                drawable: None,
                position: Vec2::ZERO,
                flush: graphics::DrawParam::default(),
            }
        }

        /// get potentially new data & prepare it to be flushed to the ggez'
        pub fn update(&mut self) {
            let new_x = self.position.x;
            let new_y = self.position.y;
            let flush_cords = self.get_mut_flush_cords();
            flush_cords.x = new_x;
            flush_cords.y = new_y;
        }

        /// flush Entity's data onto the canvas' buffer
        pub fn draw(&self, canvas: &mut graphics::Canvas) {
            if let Some(drawable) = &self.drawable {
                canvas.draw(drawable, self.flush);
            }
        }

        pub fn view_position(&self) -> Vec2 {
            self.position
        }

        fn get_mut_flush_cords(&mut self) -> &mut mint::Point2<f32> {
            if let graphics::Transform::Values { dest, .. } = &mut self.flush.transform {
                dest
            } else {
                panic!("Cannot calculate destination value for a DrawParam matrix")
            }
        }
    }

    struct BoundF32 {
        value: f32,
        lower: f32,
        upper: f32,
    }

    impl BoundF32 {
        /// returns true if was in bounds, false if wasn't and had to adjust
        fn maintain_bounds(&mut self) -> bool {
            if self.value > self.upper {
                self.value = self.upper;
                false
            } else if self.value < self.lower {
                self.value = self.lower;
                false
            } else {
                true
            }
        }
    }

    pub trait Moveable {
        fn integrate(&mut self, dt: f32);

        fn up(&mut self) {}
        fn down(&mut self) {}
        fn right(&mut self) {}
        fn left(&mut self) {}

        fn stop_up(&mut self) {}
        fn stop_down(&mut self) {}
        fn stop_right(&mut self) {}
        fn stop_left(&mut self) {}
    }

    #[derive(Default)]
    struct MoveStates {
        up: bool,
        down: bool,
        right: bool,
        left: bool,
    }

    pub struct Player {
        pub e: Entity<Mesh>,
        move_states: MoveStates,
        // current velocity
        velocity: Vec2,
        max_velocity: Vec2,
    }

    impl Player {
        pub fn new(accel_x: f32, accel_y: f32) -> Self {
            Self {
                e: Entity::new(),
                move_states: MoveStates::default(),
                velocity: Vec2::ZERO,
                max_velocity: Vec2::new(accel_x, accel_y),
            }
        }

        #[rustfmt::skip]
        pub fn update(&mut self, dt: f32) {

            if let MoveStates { up: true, down: true, .. } = self.move_states { self.velocity = Vec2::ZERO; }
            if let MoveStates { up: true, down: false, .. } = self.move_states { self.velocity.y = -self.max_velocity.y; }
            if let MoveStates { up: false, down: true, .. } = self.move_states { self.velocity.y = self.max_velocity.y; }

            if let MoveStates { left: true, right: true, .. } = self.move_states { self.velocity = Vec2::ZERO; }
            if let MoveStates { left: true, right: false, .. } = self.move_states { self.velocity.x = -self.max_velocity.x; }
            if let MoveStates { left: false, right: true, .. } = self.move_states { self.velocity.x = self.max_velocity.x; }

            if let MoveStates { up: false, down: false, left: false, right: false } = self.move_states { self.velocity = Vec2::ZERO; }


            self.integrate(dt);
            self.e.update();
        }

        pub fn view_velocity(&self) -> Vec2 {
            self.velocity
        }
    }

    impl Moveable for Player {
        fn integrate(&mut self, dt: f32) {
            self.e.position += self.velocity * dt;
        }

        fn up(&mut self) {
            self.move_states.up = true;
        }

        fn down(&mut self) {
            self.move_states.down = true;
        }

        fn right(&mut self) {
            self.move_states.right = true;
        }

        fn left(&mut self) {
            self.move_states.left = true;
        }

        fn stop_up(&mut self) {
            self.move_states.up = false;
        }

        fn stop_down(&mut self) {
            self.move_states.down = false;
        }

        fn stop_right(&mut self) {
            self.move_states.right = false;
        }

        fn stop_left(&mut self) {
            self.move_states.left = false;
        }
    }
}
