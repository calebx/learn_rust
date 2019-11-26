#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let u = User {
        username: String::from("cx"),
        email: String::from("cx@live.com"),
        active: true,
    };

    println!("user u is {:?}", u);
    
    // struct follow same rules of move bind and borrow
    let v = User {
        username: u.username.clone(),
        email: String::from("go@live.com"),
        active: true
    };

    println!("user v is {:?}", v);

    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Point(0, 0, 0);
    println!("blank is {:?}", black);

    #[derive(Debug)]
    struct Nothing();
    let nothing = Nothing();
    println!("nothing is {:?}", nothing);

    example();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn example() {
    let r = Rectangle { width: 16, height: 9 };
    println!("area of r is {}", r.area());
    println!("{:?} is still visiable", r);

    let s = Rectangle::square(10);
    println!("square is {:?}", s);
}
