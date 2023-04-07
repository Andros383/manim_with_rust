use super::vector::Vector;

pub struct Circle {
    origin: Vector,
    radius: f64,
}
impl Circle {
    pub fn new(origin: Vector, radius: f64) -> Self {
        Self { origin, radius }
    }
}
impl super::Object for Circle {
    fn contains(&self, point: Vector) -> bool {
        self.origin.distance(&point) < self.radius
    }
}
