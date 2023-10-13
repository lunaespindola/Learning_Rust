fn main() {
    let mut v = vec![4, 8, 15, 16, 23, 42];
    v = add_one(v);
    println!("v: {:?}", v);
    mul_two(&mut v);
    println!("v: {:?}", v);
}

fn add_one(mut a: Vec<i32>) -> Vec<i32> {
    for i in 0..a.len() {
        a[i] += 1;
    }
    a
}

fn mul_two(a: &mut Vec<i32>) {
    for i in 0..a.len() {
        a[i] *= 2;
    }
    (*a).push(108);
}