#[derive(Debug)]
enum CarKind {
    Privet(String),
    Company(String),
}
#[derive(Debug)]
struct Car {
    kind: CarKind,
    color: String,
    zero_retouch: bool,
}

fn main() {
    let _privet_car = CarKind::Privet(String::from("sofiane"));
    let car: Car = Car {
        kind: CarKind::Company(String::from("Hermes")),
        color: String::from("Nwar"),
        zero_retouch: true,
    };
    println!(
        "car kind : {:?}  , color {} , zero retouch {} ",
        car.kind, car.color, car.zero_retouch
    )
}
