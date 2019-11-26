fn main() {
    ownership();
    borrow();
    slice();
}

fn ownership() {
    let i: u32 = 1;
    println!("i is {}", i);

    // assign a variable to other, has 2 cases:
    // a) copy, for those types who implemented Copy trait, as u32
    let j = i;
    println!("i is {}, and j is {}", i, j);

    // b) move, for those types who has no Copy trait, as String
    let s1 = String::from("hello");
    println!("s1 is {}", s1);

    let s2 = s1;
    println!("s2 is {}", s2);
    // so here we can't call s1 anymore, since s1 is moved to s2
    // it is out of scope now once re-assigned
    // ps: anything implemented Drop trait can not has Copy trait

    // call a function is move too
    // here we will lost s2, since it is moved into function scope
    // and be droped there
    take_a_str(s2);

    let s3 = give_a_str();
    println!("s3 is {}", s3);

    fn take_a_str(s: String) {
        println!("took {}", s);
    }

    fn give_a_str() -> String {
        "hi".into()
    }
}

fn borrow() {
    // borrow means call a function with reference as its arguments
    // & ampersand means create a reference to certain variable
    // reference is a certain type which has copy, dereference function
    let s1 = String::from("hi");
    let s1_len = get_length(&s1);
    println!("the length of s1 is {}", s1_len);

    fn get_length(s: &String) -> usize {
        s.len()
    }

    // in single scope reference has rules
    // we can has multiple none mutable reference
    // or single mutable reference
    // or some none mutable reference used before mutable, but not used after none mutable (it is just sugar syntax)
    let mut s2 = String::from("boy");
    append_ex(&mut s2);
    println!("s2 is now {}", s2);

    fn append_ex(s: &mut String) {
        s.push_str("!");
    }
}

fn slice() {
    // slice has no ownership
    // slice reference a contiguous sequence of elements in a collection of heap
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    let s = String::from("hi, caleb");
    let l = first_word(&s);
    println!("first word of s is {}", l);


    fn new_first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let l = new_first_word(&s);
    println!("new first word of s is {}", l);
}
