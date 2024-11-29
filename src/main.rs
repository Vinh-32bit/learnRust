fn main() {
    println!("Hello world");
    println!("{} ages", 10);
    println!("{1}, age: {0}", 19, "Vinh");
    println!("{name}, age: {age}, height: {h}",
            name = "NT Vinh",
            age = 19,
            h = 170);

    //co the format bang cach dung :

    println!("Base 10:                  {}",  100);
    println!("Base 2(binary):           {:b}",100);
    println!("Base 8(octal):            {:o}",100);
    println!("Base 16(hexadecimal):     {:x}",100);

    //chen va can le
    println!("{number:>5}", number=1);
    println!("{number:0>5}", number=1);
    println!("{number:0<5}", number=1);
    //witdh lan *
    println!("{number:*>width$}", number=1, width=6);

    #[allow(dead_code)]
    struct Str(i32);
    let num: f64 = 1.0;
    let wdth: usize = 20;
    println!("{num:>wdth$}");
    }