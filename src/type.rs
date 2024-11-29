fn main(){
    let a: i32 = -10;
    let b: u32 = 20;
    println!("32 bit postive: {0}, 32 bit negative:  {1}", a, b);
    let c: f32 = 3.14;
    let d: f64 = -2.1012005;
    println!("32 bit float: {0}, 64 bit float:  {1}", c, d);
    let is_active: bool = true;
    let letter   : char = 'R';
    println!("{0} is {1}", letter, is_active);
    let name = "Vinh";  //chuoi tham chieu (&str)
    let name2 = "NT_Vinh".to_string();
    let name3 = String::from("Nguyen Thanh Vinh");
    println!("My name is {0} \nMy name is {1} \nMy name is {2}", name, name2, name3);


    let mut greet = String::from("Hello");
    let mut greet2 = "Hello2";   
    let mut greet3 = "Hello3".to_string();

    greet3.push('.'); //add char
    greet3.push_str("Vinh dep chai"); //add string
    println!("{}",greet2);

    // kieu du lieu tong hop

    let tup = (21, 9.1, "Vinh");
    let tupple: (i32,f32, String) = (2005, 21.1, "Nguyen Thanh Vinh".to_string());

    let x =tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("x: {}, y: {} ,x: {}", x,y,z);

    // giai nen tuple
    let(a,b,c) = tupple;
    println!("a: {}, b: {}, c: {}", a,b,c);

    let mty = ();
    let empty: () = ();
    
    let arr: [i32; 5] = [1,2,3,4,5];
    println!("{}", arr[0]);
}