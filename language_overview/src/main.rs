fn main() {
    hola();
    let n = 34;
    let x = fact(n);
    println!("El factorial de {} es {}", n, x);

    let n = -34;
    let x = sign(n);
    println!("El signo de {} es {}", n, x);

    let n = 0;
    match sign(n) {
        -1 => println!("{} es negativo", n),
        1 => println!("{} es positivo", n),
        _ => println!("{} es cero", n)
    }

}

fn hola() {
    println!("Hola, mundo!");
}

// fn factorial(n: i32) -> i32 {
//     if n == 0 {
//         1
//     } else {
//         n * factorial(n - 1)
//     }
// }

fn fact(n: u8) -> u128 {
    let mut result: u128 = 1;
    // for i in 2..=n {
    //     result *= i as u128;
    // }

    let mut i = 2;
    // while i <= n {
    //     result *= i as u128;
    //     i += 1;
    // }

    loop {
        if i > n {
            break;
        }
        result *= i as u128;
        i += 1;
    }

    result
}

fn sign(n: i32) -> i32 {
    if n > 0 {
        1
    } else if n < 0 {
        -1
    } else {
        0
    }
}