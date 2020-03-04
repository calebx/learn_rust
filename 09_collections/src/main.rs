use std::collections::HashMap;

fn main() {
    // classic init vector with <T>
    let v: Vec<u32> = Vec::new();
    println!("blank v is :{:?}", v);

    // Vec! is a normal macro to init an vector with infered type
    let v = vec![1, 2, 3];
    println!("v again is :{:?}", v);

    let mut v: Vec<String> = Vec::new();
    v.push("1".into());
    v.push("3".into());
    v.push("5".into());
    println!("v.push :{:?}", v);

    // read by []
    let third = &v[2];
    println!("third is {:?}", third);
    
    // read by .get()
    let first = v.get(0);
    println!("0 is {:?}", first);

    // loop from vector, iter() will borrow, into_inter() will move
    for s in v.iter() {
        println!("s is {:?}", s);
    }
    println!("can we read again? {:?}", v);
    for s in v.into_iter() {
        println!("s is {:?}", s);
    }
    // so here we can not read v again after into_inter()
    // println!("can we read again? {:?}", v);

    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 30;
    }
    println!("and now v is {:?}", v);
    let mut v = vec!["hi".to_string()];
    for s in &mut v {
        (*s).push_str(" hi");
    }
    println!("after push str {:?}", v);

    // Hash is important everywhere
    let mut scores = HashMap::new();
    scores.insert("hello".to_string(), 1 as u32);
    scores.insert("world".to_string(), 2 as u32); 
    println!("hash is {:?}", scores);

    // by zip
    let keys = vec![String::from("hi"), String::from("hola"), String::from("boy")];
    let vals = vec![1, 2, 3];
    let mut scores: HashMap<String, usize> = keys.into_iter().zip(vals.into_iter()).collect();
    println!("combine hash is {:?}", scores);
    
    scores.insert("good".into(), 100);
    println!("inserted hash is {:?}", scores);
    
    // insert or update
    scores.entry(String::from("girl")).or_insert(101);
    println!("insert or update hash is {:?}", scores);
}
