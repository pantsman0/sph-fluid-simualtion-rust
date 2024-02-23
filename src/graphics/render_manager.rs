use crate::fluid_simulation::particle::Particle;
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::RenderArgs;
use std::cmp::max;
use graphics::{color::{BLACK, WHITE}, math::Vec2d, *};

use std::time::{Instant, Duration};
pub struct RenderManager<'r> {
  gl: GlGraphics,
  cc: opengl_graphics::GlyphCache<'r>,
  last_render_time: Instant
}


impl<'a> RenderManager<'a> {

  pub fn new(gl: GlGraphics, cc: GlyphCache<'a>) -> Self {
    RenderManager {
        gl,
        cc,
        last_render_time: Instant::now()
    }
  }


  pub fn render(&mut self, args: &RenderArgs, particles: &Vec<Particle>) {
    self.gl.draw(args.viewport(), |c, gl| {
        clear(BLACK, gl);
        //let start = std::time::Instant::now();
        for particle in particles {
            let color = Self::speed_to_color_gradient(particle.speed());
            ellipse(
                color,
                //[0.0, 0.0, 1.0, 1.0],
                [particle.position.x as f64, particle.position.y as f64, 5.0, 5.0],
                c.transform,
                gl,
            );
        }

        Text::new_color( WHITE, 10).draw_pos(format!("{} fps", 1000 / self.last_render_time.elapsed().as_millis() ).as_str(), [10.0, 10.0], &mut self.cc, &Default::default(), c.transform, gl).unwrap();
        self.last_render_time = Instant::now();
        //println!("ellipse_calls: {:?}, particles: {}", start.elapsed(), particles.len());
    });
  }

  fn speed_to_color_gradient(speed: f32) -> [f32; 4] {
    const MAX_SPEED: f32 = 250.0;
    let ratio: f32 = speed / MAX_SPEED;
    let normalized = (ratio * 256.0 * 4.0) as i32;
    let region = (normalized / 256) as i32;
    let x = normalized % 256;
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    match region {
        3 => {
            r = 1.0;
            g = (max(255 - x, 0) as f32)/ 255.0;
            b = 0.0;
        }
        2 => {
            r = (max(x, 0) as f32) / 255.0;
            g = 1.0;
            b = 0.0;
        }
        1 => {
            r = 0.0;
            g = 1.0;
            b = (max(255 - x, 0) as f32) / 255.0;
        }
        0 => {
            r = 0.0;
            g = (max(x, 0) as f32) / 255.0;
            b = 1.0;
        }
        _ => {}
    }

    [r, g, b, 1.0]
  } 
}