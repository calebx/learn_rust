// N.R. not recommanded
const MAX_POINTS: u32 = 1_000;

fn main() {
    variables();
    data_types();
    functions();
    control_flow();
}

fn variables() {
    // * const MUST be annotated
    // * const can be declared everywhere
    // * const MUST be set only to constant expression, NO result of any runtime function
    // * const naming convention is use all UPPERCASE
    println!("MAX_POINTS is {}", MAX_POINTS);

    // variable IS immutable
    // variable should be declared in some scope
    // variable could be shadowed, like declare again (N.R.)
    let i = 1;
    println!("i is {}", i);
    let i = 'x';
    println!("i is {} now, been shadowed", i);

    // mutable variable should be decorated with mut keyword (N.R.)
    let mut j = 1.0;
    println!("j is {}", j);
    j += 0.5;
    println!("j can be muted {}", j);

    // Data types has two data subsets: Scalar & Compound
    // All Variabls MUST know their types at compile time
    // Annoation is required for generic function
    // Or require to sepcify type argument
    let g = "43".parse::<u32>().expect("not a number");
    let f: i32 = "37".parse().expect("not a number");
    println!("g is {}, f is {}", g, f);
}

fn data_types() {
    // Scalar Types
    // Integer: i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
    // Float: f32, f64
    // Boolean: true | false
    // Character: 'a', '你';
    println!("{}, {}, {}, {}", 10 as i32, 3.14 as f32, true, 'z');

    // Compound Types
    // Tuple 
    let tup: (i32, f64, char) = (10, 1.4, 'z');
    println!("{:?}", tup);
    // destructure is cool
    
    let (x, y, z) = tup;
    println!("{:?} {:?} {:?}", x, y, z);
    
    // Array
    // aray length MUST be claimed, since array or tuple lives in memory stack
    let arr: [u8; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
    
    // some shortcut, eg: 9 sevens
    let mut brr = [7; 9];
    println!("{:?}", brr);

    // access and mutate
    brr[3] = 17;
    println!("{:?}", brr);    
}

fn functions() {
    // functions should be defined in same scope or parent scope to caller
    // vice versa
    nested_function();

    fn nested_function() {
        println!("this is nested function");
    }

    // params MUST declared with type
    println!("a + b is {}", plus(1, 2));
    fn plus(a: i32, b: i32) -> i32 {
        a + b // ending expression is the same to return statement
    }

    // statement & expression
    // statement did not return anything, this is a just assign;
    // so we can not code like let a = b = 1;
    let x = 5;

    // expression can return some value
    // call function is an expression, create new scope is express
    let y = {
        let z = x;
        z + 1
    };

    println!("{}, {}", x, y);
}

fn control_flow() {
    let number = 3;

    // if statement MUST provide boolean in condition
    if number < 5 {
        println!("condition was true");
    }
    else {
        println!("condition was false");   
    }

    // rust has no triple operator
    // all ending expression in different scopes should have same type
    let number = if true {
        5
    } else if false {
        6
    }  else {
        7
    };
    println!("number is {}", number); 

    // loop, and loop with break result
    let mut count = 0;
    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };
    println!("result is {}", result); 

    // while, simple, can not break with result
    let arr = [5; 9];
    let mut index = 0;
    while index < 5 {
        index += 1;
        if index == 7 {
            println!("arr[7] is {}", arr[index]);
            break;
        }
    }

    for elem in arr.iter() {
        print!("{} > ", elem);
    }
    print!("\n");
}
