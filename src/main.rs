use std::fmt;
use rand::Rng;

extern crate tch;
use tch::Tensor;

struct Point {
    x: i32,
    y: i32,
}

union MyUnion {
    f1: u32,
    f2: u32,
}

enum Color {
    Red,
    Green,
    Blue,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Blue => write!(f, "Blue"),
        }
    }
}

fn main() {
    println!("Hello, world!");

    let n = 0;
    const MAX_POINTS: u32 = 100_000;

    let x: i32 = 123;
    let y: i64 = x as i64;

    println!("{}, {}", n, MAX_POINTS);
    println!("{}: {}", x, y);

    let p = Point { x: 2, y: 5 };
    println!("struct: {}, {}", p.x, p.y);

    let u = MyUnion { f1: 123 };
    unsafe {
        println!("union: {}, {}", u.f1, u.f2);
    }

    let color = Color::Red;
    println!("enum: {}", color);
    let arr = [Color::Red, Color::Green, Color::Blue];
    // 乱数
    let mut rng = rand::thread_rng();
    let i: usize = rng.gen_range(0..3);
    println!("random: {}", arr[i]);

    let mut name0: &str = "Yamada";
    println!("{}", name0);
    name0 = "Tanaka";
    println!("{}", name0);

    // 文字列を初期化する
    let mut name1 = String::from("Yamada");
    println!("{}", name1);
    // 別の文字列を設定する
    name1 = "Tanaka".to_string();
    // 文字列に追加する
    name1.push_str(" Taro");
    println!("{}", name1);

    println!("{}", &name0[1..2]);
    println!("{}", &name1[3..7]);

    let s = if n == 1 { "OK!" } else { "NG!" };
    println!("{}", s);

    let mut count = 0;
    while count < 3 {
        println!("while loop: {}", count);
        count += 1;
    }

    for i in 0..3 {
        println!("for loop: {}", i);
    }

    println!("torch");
    let t = Tensor::of_slice(&[3, 1, 4, 1, 5]);
    let t = t * 2;
    t.print();
}
