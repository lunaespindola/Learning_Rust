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
/*
El premio que recibió Norman por ganar el juego de tiro al pato fue un fajo de Feriapesos, 
los cuales son billetes que pueden utilizarse para pagar o reducir el precio de diversos 
alimentos en la sección de comida. Cada establecimiento que acepta Feriapesos los cambia siempre a razón de 10 a 1, es decir, 
10 Feriapesos equivale a un peso real y sólo se podrán cambiar en múltiplos de 10, 
con la finalidad de evitar los centavos. Por ejemplo, 15 Feriapesos equivalen solo a 1 peso, sin embargo 
20 Feriapesos se cambian por 2 pesos.

A Norman se le antojó un buñuelo que cuesta x pesos y quiere utilizar la mayor cantidad de Feriapesos para pagarlo, 
sin embargo, debido a la restricción en el cambio de los Feriapesos, no sabe bien cuál es la cantidad que aún debe pagar. 
¿Podrías de nueva cuenta ayudar a Norman?

Entrada
La primera línea se compone de un entero  X que representa el precio del buñuelo y la siguiente contiene un entero 
F que simboliza la cantidad de Feriapesos que Norman posee. Suponer que 0 <= X, F<=10^6

Salida
Imprimir el símbolo $ seguido de un entero que representa la cantidad de dinero que aún debe pagar Norman para recibir su buñuelo.

Ejemplo de entrada
8
29

Ejemplo de salida
$6

Ejemplo de entrada 2
1
10

Ejemplo de salida 2
$0
*/
use std::io::Read;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace();

    let x: u32 = input.next().unwrap().parse().unwrap();
    let f: u32 = input.next().unwrap().parse().unwrap();

    
}

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
/*
In one of his many trips, Mr. Ed boarded an airplane where the captain talked like… well, like he was completely drunk. “I would like to greet a very special person here, which has been part of our lives for so much time. His wife says from the control tower that she loves you” – the captain said. Of course, Mr. Ed was really scared, how can alcoholic pilots flight with so many people on their hands? But that was not the worst part, our friend noticed that these drunken pilots like to race between them!
By getting close to the captain’s cabin, Mr. Ed could hear another pilot (drunk, as expected) discussing with the captain by radio. Both of them shared information about how fast they were travelling and how far they were to the nearest airport. “If I arrive earlier to the airport, you will owe me a beer” – the captain bragged, then the airplane started to move abruptly.
Of course Mr. Ed survived, if not, he could not tell us this story. But weirdly, you are wondering who won the race between the pilots and their average arrival time, so you asked the velocity and distance to the airport of both planes. Assume that the planes maintained their velocity even when landing.
Input
The input will contain several test cases. The only line of each test case contains 4 space-separated integers v1, d1 , v2, d2
(1 <= v1,d1,v2,d2 <= 10^9): the velocity and distance to the airport of the plane Mr. Ed and the captain were and the velocity and distance to the airport of the plane the captain was competing with. Velocities are expressed in miles per hour and distances in miles.
The last test case is followed by a single line containing 4 zeroes.

Output
Print 2 lines for each test case. In the first one, you should print "You owe me a beer!" if the captain won the race or "No beer for the captain." if the other airplane won the race.
You can safely assume there will be no draws in any test case.

In the second line, print "Avg. arrival time:" followed by the average arrival time (in hours) of both airplanes expressed as a simplified fraction of the form 
x/y, being  x and y integers. If the fraction has an integer result, print the result of the division. See format below for more details.

Example
Input:
2 4 1 3
1 3 2 4
4 7 4 9
0 0 0 0

Output:
Case #1: You owe me a beer!
Avg. arrival time: 5/2
Case #2: No beer for the captain.
Avg. arrival time: 5/2
Case #3: You owe me a beer!
Avg. arrival time: 2

*/

// use std::io::{self};

// fn main() {
//     let mut input = String::new();
//     let mut case = 1;

//     loop {
//         input.clear();
//         io::stdin().read_line(&mut input).expect("Failed to read input");
//         let values: Vec<u64> = input
//             .split_whitespace()
//             .map(|s| s.trim().parse().expect("Failed to parse input"))
//             .collect();

//         if values.len() == 4 && values.iter().all(|&x| x == 0) {
//             break;
//         }

//         let v1 = values[0];
//         let d1 = values[1];
//         let v2 = values[2];
//         let d2 = values[3];

//         let time1 = (d1 as f64) / (v1 as f64);
//         let time2 = (d2 as f64) / (v2 as f64);

//         println!("Case #{}: {}", case, if time1 < time2 { "You owe me a beer!" } else { "No beer for the captain." });

//         let avg_time = (time1 + time2) / 2.0;

//         let avg_time_numerator = (avg_time * 2.0) as u64;
//         let avg_time_denominator = 2;

//         if avg_time_denominator == 1 {
//             println!("Avg. arrival time: {}", avg_time_numerator);
//         } else {
//             if avg_time_numerator % avg_time_denominator == 0 {
//                 println!("Avg. arrival time: {}", avg_time_numerator / avg_time_denominator);
//             } else {
//                 println!("Avg. arrival time: {}/{}", avg_time_numerator, avg_time_denominator);
//             }
//         }

//         case += 1;
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

