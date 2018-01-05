use shape::Point;
use rand::{ThreadRng, Rng};

pub fn clamp(t: f32, min: f32, max: f32) -> f32 {
    min.max(max.min(t))
}

pub fn rlerp(rng: &mut ThreadRng, a: f32, b: f32, n_steps: usize) -> Vec<f32> {
    let dt = 1.0 / n_steps as f32;
    return (0..n_steps).map(|i| {
        let mut t = dt * i as f32;
        t += rng.gen_range(-dt/2.0, dt/2.0);
        t = clamp(t, a, b);
        (1.0 - t) * a + t * b
    }).collect();
}

pub fn rlerp_point(rng: &mut ThreadRng, a: &Point, b: &Point, n_steps: usize) -> Vec<Point> {
    return rlerp(rng, 0.0, 1.0, n_steps).iter().map(|t| {
        Point {
            x: (1.0 - t) * a.x + t * b.x,
            y: (1.0 - t) * a.y + t * b.y,
        }
    }).collect();
}
