use self::vector::Vector;

pub mod circle;
pub mod vector;
pub mod complex;

pub trait Object {
    fn contains(&self, v: Vector) -> bool;
}
