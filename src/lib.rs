pub fn hello() {
    let x: i64 = 10;
    let y: i64 = 10;
    let z = x + y * 100;
    println!("{z}");
}

pub fn calle() {
    println!("Success!");
    hello();
}
