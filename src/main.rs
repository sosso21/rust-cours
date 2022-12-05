trait HasArea {
    fn area(&self) -> f64;
}

struct Square {
    x: f64,
    y: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.x * self.y
    }
}

fn print_area<T: HasArea>(square: T) {
    let area = square.area();
    println!(" the area is {area} ")
}

fn main() {
    let chess = Square { x: 5.0, y: 5.0 };
    print_area(chess)
}
