use graphics::*;
use opengl_graphics::GlGraphics;

use piston::window::Size;
use crate::geom::Position;


pub mod bullet;
pub mod enemy;
pub mod player;

pub trait GameObject{
    fn collides(&self, other: &dyn GameObject) -> bool{
        let x2 = self.position().x - other.position().x;
        let y2 = self.position().y - other.position().y;

        let sum = x2.powf(2.0) + y2.powf(2.0);

        let r_start = self.radius() - other.radius();
        let r_end = self.radius() + other.radius();

        r_start.powf(2.0) <= sum && sum <= r_end.powf(2.0)
    }

    fn position(&self) -> &Position;
    fn radius(&self) -> f64;

    fn render(&self, ctxt: &Context, gl: &mut GlGraphics);

    fn render_dbg(&self, _: &Context, _: &mut GlGraphics) {}

    fn update(&mut self, _: f64, _:Size){}
}
