fn main() {
    let x: i8 = -100;
    let y: i8 = 50;

    println!("{} ... {}({} bits)", i8::MIN, i8::MAX, i8::BITS);

    let z: Option<i8> = x.checked_add(y);

    match z {
        Some(n) => println!("{} + {} = {}, {:?}", x, y, n, z),
        None => println!("JAJAJAJAJAJAJAJAJAJJAJAJAJAJA")
    }

    let z = x.checked_add(y).unwrap_or(i8::MAX);
    println!("{} + {} = {}", x, y, z);

    let z = x.checked_add(y).unwrap_or_default();
    println!("{} + {} = {}", x, y, z);

    let z = x.checked_sub(y).unwrap_or_default();
    println!("{} - {} = {}", x, y, z);
}