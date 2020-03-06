fn largest<T: Copy + std::cmp::PartialOrd>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn longer_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let nums = [1, 2, 3, 2, 1];
    println!("largest nums is: {:?}", largest(&nums));

    let floats = [1.1, 1.2, 1.5, 0.0, 0.7];
    println!("largest floats is: {:?}", largest(&floats));

    let str_1 = String::from("abcd");
    let str_2 = "xyz!!";
    let longer = longer_str(str_1.as_str(), str_2);
    println!("longer one is {:?}", longer);
}
