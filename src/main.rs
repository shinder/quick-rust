fn main() {
    println!("Hello, world!");
    // println!("Hello, 林新德");
    my_for_loop_1();
    my_for_loop_2();
    str_1();
    str_2();
}

fn my_for_loop_1() {
    for n in 1..=3 {
        println!("loop_1: {}", n);
    }
}

fn my_for_loop_2() {
    let arr = [10, 20, 30];
    for i in arr {
        println!("loop_2: {}", i);
    }
}

fn str_1() {
    let mut str1 = "Hello, String";
    println!("{}", str1);
    str1 = "Hello, Rust";
    println!("{}", str1);
}

fn str_2() {
    let mut str1 = String::from("Hello, String2");
    println!("{}", str1);
    str1 = String::from("Hello, Rust2");
    println!("{}", str1);
    let str2 = &mut str1;
    // println!("{}", str1);
    println!("{}", str2);
}
