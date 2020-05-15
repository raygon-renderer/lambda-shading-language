#![allow(unused)]

use pest::Parser;

use sll_parser::{ast::parse, Grammar};

fn main() {
    let test = r##"
struct Test {
    a: i32,
    b: [f32; 4],
    c: *mut Vector3
}

const HELLO: str = r#"
Hello, World!
Testing
"#;

fn other(_x: f32, Test { a: y, b: Test {a, b, c}, c }: Test) {
    let i: (i32, i32, i32, i32) = (1, 2, 3, 4);
    let (a, mut b, c, d): (i32, i32, i32, i32) = i;

    // do something
}

fn main() {
    let v: Vector3 = Vector3(1, 2, 3);

    let x: Test = Test {
        a: (0 as i32) as u32,
        b: [1, 2, 3, 4],
        c: &v
    };

    let b: i32 = x.a;
    let c: f32 = -*x.c;
}

fn add(x: i32, y: i32) -> i32 {
    let z: i32 = x + y;

    let h: i32 = if z < 2 {
        y = 0;
        z + 2
    } else if y < 0 {
        z += 1;

        0
    } else {
        1
    };

    for i in 0..4 {
        z += i;
    }

    while z < 10 {
        z += 1;
    }

    other(z + 5);

    let = true;

    let x = t.0[0].t[5].y[1+4];


    z + 1
}
"##;

    let test2 = r##"
    const TEST: *const u32 = 1 + 0xF1;

    unsafe fn test(mut x: i32, Test {x: mut i, k}: (u32,)) -> i32 {
        let mut x: i32 = 10;

        x.0[0-1].x().x += 10;

        x.test(x+1).0;
    }
    "##;

    //println!("{:#?}", Grammar::parse(Rule::file, test2));

    println!("{:#?}", parse(test2).unwrap());
}
