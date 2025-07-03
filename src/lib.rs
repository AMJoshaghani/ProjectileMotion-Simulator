mod utils;

use std::f32::consts::PI;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern {
    // fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Ball {
    x: f64,
    y: f64,
    angle: f64,
    velocity: f64,
    damping_constant: f64,
    stopped: bool,
}

#[wasm_bindgen]
impl Ball {
    pub fn new(x: f64, y: f64, angle: f64, velocity: f64, damping_constant: f64) -> Ball {
        // D/180 = R/pi -> R = D . pi / 180
        Ball { x, y, angle: angle*(PI as f64)/180., velocity, damping_constant, stopped: false }
    }
    pub fn tick(&mut self, dt: f64) {
        // x = (v cos a) t => t = x / (v cos a)
        // y = (v sin a) t - 1/2 gt^2
        // y = x tan a - 1/2 gx^2/(v cos a)

        if self.stopped {
            return;
        }
        // compute components
        let mut vx = self.velocity * self.angle.cos();
        let mut vy = self.velocity * self.angle.sin();

        // apply gravity
        vy -= 9.8 * dt;

        // update position
        self.x += vx * dt;
        self.y += vy * dt;

        // bounce
        if self.y <= 90.0 && vy < 0.0 {
            self.y = 90.0;
            vy = -vy * self.damping_constant;
            vx = vx * self.damping_constant;

            self.velocity = (vx * vx + vy * vy).sqrt();
            self.angle = vy.atan2(vx);

            if self.velocity < 2.0 {
                self.stopped = true;
            }
        } else {
            self.velocity = (vx * vx + vy * vy).sqrt();
            self.angle = vy.atan2(vx);
        }
    }
    pub fn get_x(&self) -> f64 { self.x }
    pub fn get_y(&self) -> f64 { self.y }
    pub fn get_velocity(&self) -> f64 { self.velocity }
    pub fn stopped(&self) -> bool { self.stopped }
}