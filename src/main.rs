use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};

struct MainState {
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {};
        Ok(s)
    }
}

fn make_sun(ctx: &mut Context, pos: Point2<f32>) -> GameResult<graphics::Mesh> {
    let mut mb = graphics::MeshBuilder::new();
    for i in -4..4 {
        mb.line(&[pos, pos + Vector2::new(200., 50. * i as f32)], 4., graphics::WHITE)?;
    }
    mb.build(ctx)
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update player physics and handle exit

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let sun = make_sun(ctx, Point2::new(200., 200.))?;
        graphics::draw(ctx, &sun, (Point2::new(0., 0.),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("partouch", "dincio");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
