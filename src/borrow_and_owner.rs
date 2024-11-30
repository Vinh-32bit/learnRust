fn main(){
    //ownership
    let s1 = String::from("Hello"); //s1 sở hữu chuỗi
    let s2 = s1; //sở hữu được chuyển từ s1 sang s2

    // println!("{}", s1); // Lỗi: s1 không còn sở hữu giá trị
    println!("{}", s2);

    
    //borrowing

    //immutable borrow
    {
        let s1 = String::from("Hello");
        let s2 = &s1;

        println!("s1: {}, s2: {}", s1,s2);
    }

    //mutable borrow
    {
        let mut s1 = String::from("Hello");
        let s2 = &mut s1;
        s2.push_str(", world!");
        // println!("s1: {}",s1); lỗi vì đã có tham chiếu mut đến s1
        println!("s2: {}",s2);
    }

    {
        let mut value = 10;
        let value_ref = &mut value;
        *value_ref += 5;
        println!("value: {}", value);
    }
    let mut value = 10;
    let value_ref = &mut value;
    *value_ref += 5;
    println!("value: {}", value_ref);
}