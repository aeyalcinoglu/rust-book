fn main() {
    let tup: (i32, _, char) = (10, 25, 'K');
    let (x, y, z) = tup;
    let t: bool;
    t = tup.2 == z;

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let s = a[1];

    // gives an error, use float literal 5.0
    // let f: f32 = 5;

    println!("Vars: {}, {}, {}, {}, {}", x, y, z, t, s);
}
