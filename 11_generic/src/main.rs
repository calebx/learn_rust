use std::fmt::Display;
use std::cmp::PartialOrd;

// why here wre pick Copy instead of Clone:
// as Copy Trait Type as i32, do not need be called explicitly
// like we can do re-assignment here
fn largest<T: Copy + PartialOrd>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let nums = [1, 2, 3, 2, 1];
    println!("largest nums is: {:?}", largest(&nums));

    let floats = [1.1, 1.2, 1.5, 0.0, 0.7];
    println!("largest floats is: {:?}", largest(&floats));

    let chars = "hello".chars().collect::<Vec<_>>();
    println!("largest char is: {}", largest(&chars));

    generic_struct();

    generic_enum();

    generic_impl();
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn generic_struct() {
    let point = Point {
        x: 3.0,
        y: 4,
    };

    println!("point is {:?}", point);
}

#[derive(Debug)]
enum Maybe<T> {
    Just(T),
    Nothing,
}

fn generic_enum() {
    let m = Maybe::Just(3);
    println!("m is {:?}", m);
    let n: Maybe<u32> = Maybe::Nothing;
    println!("n is {:?}", n);
}

#[derive(Debug)]
struct Location<T, U> {
    x: T,
    y: U,
}

impl<T: Copy, U: Display + Copy> Location<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn to_u(&self) -> Location<U, T> {
        Location {
            x: self.y,
            y: self.x,
        }
    }

    fn mixup<V, W: Copy>(&self, other: Location<V, W>) -> Location<T, W> {
        Location {
            x: self.x,
            y: other.y,
        }
    }
}

fn generic_impl() {
    let l = Location { x: 10, y: 3.3 };
    println!("l.x is {}", l.x());

    let o = l.to_u();
    println!("opposite l is {:?}", o);

    let p = o.mixup(Location { x: "hel", y: "yeah" });
    println!("mixup: {:?}", p);
}
