use self::vector::Vector;

pub mod circle;
pub mod vector;

pub trait Object {
    fn contains(&self, v: Vector) -> bool;
}
