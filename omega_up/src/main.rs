

// A. El mayor de dos

// fn main() {
//     let mut input = String::new();
//     while io::stdin().read_line(&mut input).unwrap() > 0 {
//         let mut numbers = input.split_whitespace();

//         let a: i32 = match numbers.next() {
//             Some(number) => number.parse().unwrap(),
//             None => {
//                 continue;
//             }
//         };

//         let b: i32 = match numbers.next() {
//             Some(number) => number.parse().unwrap(),
//             None => {
//                 continue;
//             }
//         };

//         if a > b {
//             println!("{}", a);
//         } else {
//             println!("{}", b);
//         }

//         input.clear();
//     }
// }


// J. Ordenamiento binario
// use std::io::{Read};
// fn main() {
//     let mut buffer = String::new();
//     std::io::stdin().read_to_string(&mut buffer).unwrap();
//     let mut input = buffer.split_ascii_whitespace();

//     let n: u16 = input.next().unwrap().parse().unwrap();

//     let mut v: Vec<(u8, u32)> = Vec::new();

//     for _ in 0..n {
//         let x: u32 = input.next().unwrap().parse().unwrap();
//         v.push((cuenta_bits_uno(x), x));
//     }

//     v.sort();
    
//     for x in v {
//         print!("{} ", x.1);
//     }
// }

// fn cuenta_bits_uno(mut x: u32) -> u8{
//     let mut bits = 0u8;
//     while x != 0 {
//         bits += (x & 1) as u8;
//         x >>= 1;
//     }
//     bits
// }



// E. Número y Pico
use std::io::{Read};
fn main(){
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace();

    let n: u32 = input.next().unwrap().parse().unwrap();

    println!("{}", numero_pico(n));
}

fn numero_pico(n: u32) -> u32 {
    if n == 1 {
        1
    } else {
        n + numero_pico(n / 2)
    }
}