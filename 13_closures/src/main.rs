struct Cacher<T>
where
    T: Fn(usize) -> usize,
{
    calc: T,
    val: Option<usize>,
}

impl<T> Cacher<T>
where
    T: Fn(usize) -> usize,
{
    fn new(f: T) -> Self {
        Self { calc: f, val: None }
    }

    fn val(mut self, n: usize) -> usize {
        match self.val {
            Some(r) => r,
            None => {
                let rst = (self.calc)(n);
                self.val = Some(rst);
                rst
            }
        }
    }
}

fn main() {
    let f = |x| x + 1;
    let c = Cacher::new(f);
    let c_1 = c.val(1);
    println!("c_1 is: {:?}", c_1);

    iter_test();
}

fn iter_test() {
    let v: Vec<u8> = vec![1, 2, 3, 0, 0, 9];

    for &i in v.iter() {
        println!("v_i is: {:?}", i);
    }
}
