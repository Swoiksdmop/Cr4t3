pub fn hello() {
    let x:isize = 10;
    let y:isize = 10;
    let z = x + y * 100;
    println!("{z}");
}

pub fn calle() {
    println!("Success!");
    hello();
}

pub fn wow() {
    println!("Great!");
    hello();
}