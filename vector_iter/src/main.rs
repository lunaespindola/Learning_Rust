fn main() {
    let mut v: Vec<String> = vec![
        "uno".to_string(),
        "dos".to_string(),
        "tres".to_string(),
    ];
    let s1 = "prueba".to_string();
    let s2: String = String::from("otra prueba");
    v.push(s1);
    v.push(s2);

    println!("len v: {}", v.len());
    println!("v: {:?}", v);
    
}
