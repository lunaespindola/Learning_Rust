#[derive(Debug)]
enum CardinalPoints {
    North,
    South,
    East,
    West,
}

// enum sex {
//     Male,
//     Female,
//     Other,
// }

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),
}

fn main() {
    let mut direction1 = CardinalPoints::East;
    let mut direction2 = CardinalPoints::North;

    println!("{:?} {:?} ", direction1, direction2);

    direction1 = CardinalPoints::West;
    direction2 = CardinalPoints::South;

    println!("{:?} {:?} ", direction1, direction2);

    let color1 = Color::Blue;
    let color2 = Color::Green;
    let color3 = Color::Red;
    let color4 = Color::RGB(0, 255, 255);

    println!("{:?} {:?} {:?} {:?} ", color1, color2, color3, color4);

    match color4 {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::RGB(x,y,z) => println!("x = {}, y = {}, z = {}", x, y, z)
    }

    let s = "123x";
    // let r = s.parse::<i32>().unwrap();
    let r = s.parse::<i32>().expect("XDDDDDDDDDDDDD");

    // match r {
    //     Ok(n) => println!("n = {}", n),
    //     Err(e) => println!("{:?}", e)
    // }
    
    println!("r = {}", r);

}
