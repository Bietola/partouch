use ggez::nalgebra as na;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{event, graphics, input::keyboard};
use ggez::{Context, GameResult};

const WINDOW_SIZE: f32 = 600.;

struct MainState {
    pov: Point2<f32>,
    zoom_lv: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            zoom_lv: 1.,
            pov: Point2::new(0., 0.),
        };
        Ok(s)
    }
}

fn make_sun(ctx: &mut Context, pos: Point2<f32>) -> GameResult<graphics::Mesh> {
    let mut mb = graphics::MeshBuilder::new();
    for i in -4..4 {
        mb.line(
            &[pos, pos + Vector2::new(200., 50. * i as f32)],
            4.,
            graphics::WHITE,
        )?;
    }
    mb.build(ctx)
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // TODO Update player physics and handle exit

        // Movement
        if keyboard::is_key_pressed(ctx, event::KeyCode::W) {
            self.pov.y += 10. / self.zoom_lv;
        } else if keyboard::is_key_pressed(ctx, event::KeyCode::D) {
            self.pov.x -= 10. / self.zoom_lv;
        } else if keyboard::is_key_pressed(ctx, event::KeyCode::S) {
            self.pov.y -= 10. / self.zoom_lv;
        } else if keyboard::is_key_pressed(ctx, event::KeyCode::A) {
            self.pov.x += 10. / self.zoom_lv;
        }
        // Zoom
        else if keyboard::is_key_pressed(ctx, event::KeyCode::Add) {
            self.zoom_lv += 0.2;
        } else if keyboard::is_key_pressed(ctx, event::KeyCode::Subtract) {
            self.zoom_lv -= 0.2;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let sun = make_sun(ctx, Point2::new(0., 0.))?;
        graphics::draw(ctx, &sun, (Point2::origin(),))?;

        graphics::set_transform(
            ctx,
            graphics::DrawParam::new()
                .dest(self.zoom_lv * self.pov + Vector2::new(WINDOW_SIZE / 2., WINDOW_SIZE / 2.))
                .scale(Vector2::new(self.zoom_lv, self.zoom_lv))
                .to_matrix(),
        );
        graphics::apply_transformations(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("partouch", "dincio")
        .window_setup(ggez::conf::WindowSetup::default().title("Ascension"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_SIZE, WINDOW_SIZE));

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;

    event::run(ctx, event_loop, state)
}
