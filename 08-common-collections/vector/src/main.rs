fn main() {
    let v = vec![1, 2, 3];
    let mut w: Vec<i32> = Vec::new();
    w.push(6);
    w.push(5);
    w.push(4);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // here & is necessary to keep the ownership
    for i in &mut w {
        // dereferencing
        *i += 50;
    }
}
