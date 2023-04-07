use crate::object::Object;

pub struct Scene {
    objects: Vec<Box<dyn Object>>,
    // camera: Camera,
}
impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
        }
    }
}
struct Camera {
    x_length: f64,
    y_lenth: f64,
}
