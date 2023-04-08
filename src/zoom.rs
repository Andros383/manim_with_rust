use super::object::vector::Vector;
use sdl2::rect::Point;
pub struct Setup {
    origin: Vector,
    width: u32,
    // rotation: f64,//Egyértelműen radiánban
}
//Azt hittem az sdl-es pont csak pozitív lehet, mert jobbra lefele megy.
//Na mind1

/// Takes in a sdl2 Point and spits back a Vector
pub fn zoom(input: Point, width: i32, height: i32, goal_setup: Setup) -> Vector {
    let x = input.x;
    let y = input.y;
    let origin = goal_setup.origin;
    let xmax = goal_setup.width as i32;
    // let ymax = goal_setup.height as i32;
    //ymaxnak nem szabad léteznie, mivel ha már tudom milyen széles lesz a kép
    //a width/height hányadosból számolható
    //És ekkor lesz helyes, akkor is, ha a renderelt kép nem 1:1 képarányú

    //Lehet letek nélkül kellene
    //Fordítás pipa
    let x = -x;

    //Ronda típuscastolás
    //X tengely átméretezése
    let x: f64 = x as f64 / width as f64 * xmax as f64;
    //Y tengely átméretezése, picit más, mert nem kell ymax, nincs is
    let y: f64 = y as f64 * xmax as f64 / width as f64;

    //Centerelés
    let x = x - width as f64 / 2.0;
    let y = y - height as f64 / 2.0;

    //Tolás új origóba
    let x = x + origin.x;
    let y = origin.y;

    //TODO forgatás.
    //Valszeg komplex számokkal könnyebb lenne
    //Ezt mondom 0:07-kor mindenféle irónia nékül
    //Nem vagyok normális
    Vector { x, y }
}
