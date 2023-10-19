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

// B. Calculando divisores
// use std::io::{Read};
// fn main() {
//     let mut buffer = String::new();
//     std::io::stdin().read_to_string(&mut buffer).unwrap();
//     let mut input = buffer.split_ascii_whitespace();

//     let n: u32 = input.next().unwrap().parse().unwrap();

//     let mut divisores: Vec<u32> = Vec::new();

//     for i in 1..=n {
//         if n % i == 0 {
//             divisores.push(i);
//         }
//     }
//     divisores.reverse();    

//     for d in divisores {
//         println!("{} ", d);
//     }
// }

// C. Los Feriapesos
// use std::io::Read;
// fn main()  {
//     let mut buffer = String::new();
//     std::io::stdin().read_to_string(&mut buffer).unwrap();
//     let mut input = buffer.split_whitespace();

//     let x: i32 = input.next().unwrap().parse().unwrap();
//     let f: i32 = input.next().unwrap().parse().unwrap();


//     if (f/10) >= x {
//         print!("$0")
//     } 
//     else {
//         println!("${}", x - (f / 10));
//     }
// }

// D. Imprimiendo enteros por paridad
// use std::io::{Read};
// fn main() {
//     let mut buffer = String::new();
//     std::io::stdin().read_to_string(&mut buffer).unwrap();
//     let mut input = buffer.split_ascii_whitespace();
//     let n: u16 = input.next().unwrap().parse().unwrap();
//     let mut v: Vec<u32> = Vec::new();
//     for _ in 0..n {
//         let x: u32 = input.next().unwrap().parse().unwrap();
//         v.push(x);
//     }
//     let p: u8 = input.next().unwrap().parse().unwrap();
//     if p == 0 {
//         for x in v {
//             if x % 2 == 0 {
//                 print!("{} ", x);
//             }
//         }
//     } else {
//         for x in v {
//             if x % 2 != 0 {
//                 print!("{} ", x);
//             }
//         }
//     }
// }

// E. Número y Pico
// use std::io::{Read};
// fn main(){
//     let mut buffer = String::new();
//     std::io::stdin().read_to_string(&mut buffer).unwrap();
//     let mut input = buffer.split_ascii_whitespace();
//     let n: u32 = input.next().unwrap().parse().unwrap();
//     println!("{}", numero_pico(n));
// }
// fn numero_pico(n: u32) -> u32 {
//     if n == 1 {
//         1
//     } else {
//         n + numero_pico(n / 2)
//     }
// }

// F. Bit de paridad
// use std::io::{self, Read};

// fn main() {
//     let mut input = String::new();
//     io::stdin().read_to_string(&mut input).expect("Failed to read input");
    
//     let input = input.trim();
    
//     if input.len() != 7 || !input.chars().all(|c| c == '0' || c == '1') {
//         println!("La secuencia de entrada debe contener exactamente 7 bits (0 o 1).");
//         return;
//     }

//     let ones_count = input.chars().filter(|&c| c == '1').count();

//     let parity_bit = if ones_count % 2 == 0 { '0' } else { '1' };

//     println!("{}{}", input, parity_bit);
// }

// G. Perimetro de asteriscos
// use std::io::{self, Read};

// fn main() {
//     let mut buffer = String::new();
//     io::stdin().read_to_string(&mut buffer).expect("Error al leer la entrada.");

//     let mut input = buffer.split_ascii_whitespace();

//     let n: u16 = input.next().unwrap().parse().unwrap();
//     let m: u16 = input.next().unwrap().parse().unwrap();

//     draw_rectangle(n, m);
// }

// fn draw_rectangle(height: u16, width: u16) {
//     for i in 0..height {
//         for j in 0..width {
//             if i == 0 || i == height - 1 || j == 0 || j == width - 1 {
//                 print!("* ");
//             } else {
//                 print!("  ");
//             }
//         }
//         println!();
//     }
// }

// H. Buscando el mayor
// use std::io::{self, Read};

// fn main() {
//     let mut buffer = String::new();
//     io::stdin().read_to_string(&mut buffer).expect("Error al leer la entrada.");

//     let mut input = buffer.split_ascii_whitespace();

//     let n: u16 = input.next().unwrap().parse().unwrap();

//     let mut max = i32::min_value();

//     for _ in 0..n {
//         let x: i32 = input.next().unwrap().parse().unwrap();
//         if x > max {
//             max = x;
//         }
//     }

//     println!("{}", max);
// }

// I. Alcoholic Pilots
// use std::io;
// use std::io::Read;

// fn gcd(mut a: usize, mut b: usize) -> usize {
//     while b != 0 {
//         let temp = b;
//         b = a % b;
//         a = temp;
//     }
//     a
// }

// fn main() {
//     let mut buffer = String::new();
//     io::stdin().read_to_string(&mut buffer).unwrap();
//     let input = buffer.lines();

//     let mut case_number = 1;

//     for line in input {
//         let numbers: Vec<usize> = line
//             .split_whitespace()
//             .map(|x| x.parse().unwrap())
//             .collect();

//         if numbers[0] == 0 && numbers[1] == 0 && numbers[2] == 0 && numbers[3] == 0 {
//             break;
//         }

//         let c = numbers[0] * numbers[3];
//         let o = numbers[1] * numbers[2];
        
//         if c > o {
//             println!("Case #{}: You owe me a beer!", case_number);
//         } else {
//             println!("Case #{}: No beer for the captain.", case_number);
//         }

//         let num = (numbers[0] * numbers[3]) + (numbers[1] * numbers[2]);
//         let den = numbers[0] * numbers[2];
//         let p1 = num * 1;
//         let p2 = den * 2;


//         let gcd = gcd(p1, p2);
//         let num2 = p1 / gcd;
//         let den2 = p2 / gcd;

//         if den2 == 1{
//             println!("Avg. arrival time: {}", num2);
//         } else {
//             println!("Avg. arrival time: {}/{}", num2, den2);
//         }

//         case_number += 1;
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


// K. Desarrollos Binomiales
// use std::io;
// fn main() {
//     let mut input = String::new();

//     io::stdin().read_line(&mut input).expect("Failed to read input");
//     let n: u64 = input.trim().parse().expect("Invalid input");

//     let mut result = String::new();

//     if n == 0 {
//         result.push_str("1");
//         println!("{}", result);
//         return;
//     }
//     else {
//         for i in 0..=n {
//             let coeficient = binomial_coeficient(n, i);
//             if coeficient != 1 {
//                 result.push_str(&format!("{}", coeficient));
//             }
//             if n - i != 0 {
//                 result.push_str(&format!("x"));
//                 if n - i != 1 {
//                     result.push_str(&format!("^{}", n - i));
//                 }
//             }
//             if i != 0 {
//                 result.push_str(&format!("y"));
//                 if i != 1 {
//                     result.push_str(&format!("^{}", i));
//                 }
//             }
//             result.push_str("+");
//         }
    
//         result.pop();
    
//         println!("{}", result);
//     }
// }

// fn binomial_coeficient(n: u64, k: u64) -> u64 {
//     let mut coeficient = 1;
//     for i in 0..k {
//         coeficient *= n - i;
//         coeficient /= i + 1;
//     }
//     coeficient
// }

// L. The Incrementor
// use std::io::{Read};
// fn main(){
//     let mut buffer = String::new();
//     std::io::stdin().read_line(&mut buffer).unwrap();
//     let n_str = buffer.trim();

//     let mut n: Vec<u8> = n_str.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
//     let resultado = incrementor(n);

//     let resultado_str: String = resultado.iter().map(|&d| (d + b'0') as char).collect();

//     println!("{}", resultado_str);
// }


// fn incrementor(mut num: Vec<u8>) -> Vec<u8> {
//     let mut carry = 1;
//     let mut i = num.len() as isize - 1;
    
//     while i >= 0 && carry > 0 {
//         let sum = num[i as usize] + carry;
//         num[i as usize] = sum % 10;
//         carry = sum / 10;
//         i -= 1;
//     }

//     if carry > 0 {
//         num.insert(0, carry);
//     }

//     num
// }

