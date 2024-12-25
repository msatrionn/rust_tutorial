use std::{array, fmt::format, result};

fn main() {
    println!("Hello, world!");
}

// unit test
#[test]
fn hello_test(){
    println!("hello test")
}

#[test]
fn hello_test_variable(){
    let name = "Satriyo";
    print!("Hello {}", name)
}

#[test]
fn test_mutable(){
    let mut name: &str = "satriyotest";
    name = "satri";
    println!("hello mas {}", name)
}

#[test]
fn shadowing(){
    let name: &str = "satriyo";
    println!("cek {} ", name);

    let name: i32 = 2;
    println!("cek {} ", name);
}

#[test]
fn comment(){
    //ini komentar
    println!("hello") //ini
}

#[test]
fn explicit(){
    //ini komentar
    let age: i32 = -32;

    // gabisa karena u atau usigned hantya bisa positif
    // let age: u32 = -32;
    println!("age {}", age) //ini
}
#[test]
fn number_conversion(){
    let a: i8 = 10;
    println!("{} ", a);
    let b:i32 = a as i32;
    println!("{} ", b);

    let d:i64 = 1000000000;
    let e:i16=d as i16;
    println!("{}",e)
}
#[test]
fn numeric_operator(){
    let a: i32 = 10;
    let b=10;
    let c = a*b;
    let d = a/b;
    println!("{}",c);
    println!("{}",d);
}
#[test]
fn augmented_assignment(){
    let mut a =10;
    a+=10;
    println!("{}",a)
}
#[test]
fn boolean(){
    let a = true;
    let b = false;
    println!("{} {}",a, b);
}

#[test]
fn char_type(){
    // char pake '' cuman bisa 1 char
    // kalo string pake ""
    let char1 = "ass";
    let char2 = 'b';

    println!("{} {}", char1, char2)
}

#[test]
fn tuple(){
    let mut data = (1.5,"cek",3,-4,'a', true);
    println!("{:?}", data);
    println!("{}",data.0);
    println!("{}",data.1);
    println!("{}",data.2);

    // let (a,_,c,_,_,_) = data;
    // println!("{} {}test",a, c);
    data.0 =11.0;
    println!("{:?}", data);
}

// tuple kosong

fn unit(){
    println!("Hello");
}

#[test]
fn test_unit(){
    let result = unit();
    println!("{:?}", result);

    let test = ();
    println!("{:?}", test);
}

#[test]
fn array(){
    let array:[i32;4]   =[1,2,2,4];
    println!("{:?}", array)
}

#[test]
fn array_access(){
    let mut array:[i32;4]   =[1,2,2,4];
    array[2] = 3;
    println!("{}",array[2]);
    // usize/u18 gamungkin negatif
    let length_arr = array.len();
    println!("total {}", length_arr)
}
#[test]
fn two_dimensional_array(){
    let array: [[i32;4];3] = [
        [1,2,2,4],
        [3,4,2,1],
        [5,6,44,4]
    ];
    // println!("{:?}", array);
    println!("{}", array[0][3]);
    println!("{:?}", array[1]);
    // println!("{} {}", MINIMUM_DATA, MAXIMUM);
}

const MAXIMUM: i32 = 42;
#[test]
fn constant(){
    // harus uppercase
    // harus explisti gabisa diisi variabel
    // kalau mau 2 kata pakai_
    const MINIMUM_DATA: i32 = 400*4;
    println!("{} {}", MINIMUM_DATA, MAXIMUM);
}

#[test]
fn scope() {
    let satriyo = 1;
    {
        println!("{}", satriyo);
        let noto = 4;
        println!("{}", noto);
    }
    println!("{}",satriyo);
}

// kalau ukuran tidak jelas masuk heap
// kalau ukuran pasti masujk ke stack

#[test]
fn stack_heap(){
    function_a();
    function_b();
}

fn function_a(){
    let a=10;
    let b=String::from("Satriyo");
    println!("{} {}", a, b)
}

fn function_b(){
    let a=10;
    let b=String::from("Noto");
    println!("{} {}", a, b)
}

#[test]
fn string(){
    let name = " Muhammad Satriyo ";
    let trim= name.trim();
    
    println!("{}", name);
    println!("{}", trim);

    // &str/stringslice tidak dapat dirubah
}

#[test]
fn string_type(){
    let mut name = String::from("Muhammad");
    name.push_str("Satriyo");
    println!("{}",name);

    let budi = name.replace("Satriyo", "goro");
    println!("{}",name);
    println!("{}",budi);
}

#[test]
fn ownership_rule(){
    let a = 10;
    {
        let b = 20;
        println!("{}",b);
    }
    println!("{}",a);
}

// copy hanya bisa di stack di heap gabuisa contoh heap pake type data String

#[test]
fn data_copy(){
    // copy data aja dari a ke b bukan replace
    let a = 10;
    let mut b = a;
    println!("{}",b);
    b = 20;
    println!("{}",a);
    println!("{}",b);
}
#[test]
fn ownership_movement(){
    let name = String::from("Satriyo");
    let name2 = name;
    println!("{}",name2);
    // diheap gabisa diakses lagi karena udah masuk name2 kayak dipindah
    // println!("{}",name);
}

#[test]
fn clone() {
    let name1= String::from("Satriyo");
    let name2= name1.clone();
    // let name3= name1;
    // let name4= name2;
    // let name5= name2;
    // println!("{}", name2);
    println!("{}", name2);
    // println!("{}", name3);
    // println!("{}", name1);
}

#[test]
fn if_expresion() {
    let value = 7;
    if value >=8 {
        println!("good");
    } else if value>=6 {
        println!("not bad");
    }else{
        println!("bad");
    }
}

#[test]
fn let_if_expresion() {
    let value = 7;
    let result:&str;
    if value >=8 {
        result = "good";
    } else if value>=6 {
        result = "not bad";
    }else{
        result = "bad";
    }
    println!("{}", result)
}
#[test]
fn let_if_expresion2() {
    let value = 7;
    let result:&str = if value >=8 {
        "good"
    } else if value>=6 {
        "not bad"
    }else{
        "bad"
    };
    println!("{}", result)
}

#[test]
fn loop_expression() {
    let mut counter=0;
    loop {
        counter += 1;
        if counter > 10{
            break;
        } else if counter%2 == 0{
            continue;
        }
        // if counter % 2 == 0{
        //     continue;
        // } else if counter > 10{
        //     break;
        // }
        println!("Counter {}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 12 {
            break counter * 2;
        }
    };
    println!("{}",result);
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if number >10 {
                break 'outer;
            }
            println!("{} x {} = {}", number, i, number*i);
            i += 1;
            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter < 10 {
        if counter % 2 == 0 {
            println!("{}",counter);
        }
        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array = ["a", "b", "c"];

    let mut index = 0;
    while index < array.len() {
        println!("index : {}, value: {}", index, array[index]);
        index += 1;
    }
}

#[test]
fn array_iteration_for() {
    let array = ["a", "b", "c"];
    for value in array{
        println!("value: {}", value);
    }
}

// range exclusive
#[test]
fn range_exclusive() {
  
    // jarak start dan finish
    let range = 0..5;
    print!("start : {}", range.start);
    print!("finish : {}", range.end);
    let array = ["a", "b", "c", "d", "e"];
    for i  in range{
        println!("index : {}, value: {}", i, array[i]);
    }

    for i  in 0..5{
        println!("index : {}, value: {}", i, array[i]);
    }
}

#[test]
fn feature() {
    let range = 0..=4;
    print!("start : {}", range.start());
    print!("finish : {}", range.end());
    let array = ["a", "b", "c", "d", "e"];
    for i  in range{
        println!("index : {}, value: {}", i, array[i]);
    }

}

fn say_hello(id: i32) {
    println!("say hello {}", id)
}

fn say_goodbye(first_name:&str, last_name:&str) {
    println!("Goodbye {} {}", first_name, last_name);
}

#[test]
fn test_say_hello() {
    say_hello(2);
    say_goodbye("muhammad", "satriyo");
    say_goodbye("muhammad", "noto");
}

fn factorial_loop(n:i32) -> i32 {
    if n <1 {
        return 0;
    }
    
    let mut result = 1;
    for i in 1..= n {
     result *= i;
    }
    result
}

#[test]
fn test_factorial_loop() {
    let result = factorial_loop(5);
    println!("result {}", result);
    let result = factorial_loop(-10);
    println!("result {}", result);
}

fn print_text(value: String, times:u32){
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }
    print_text(value, times - 1);
}

#[test]
fn test_print_text() {
    // print text kayak berapa kali gitu 
    print_text(String::from("Satriyo"), 5);
}

fn factorial_recursive(n:u32) -> u32 {
    if n <= 1 {
        return 1;
    }
    n * factorial_recursive(n-1)
}

#[test]
fn test_factorial_recorsive() {
    let result = factorial_recursive(5);
    println!("result {}", result);
}


fn print_number(number:i32) {
    println!("{}", number);
}

fn hi(name:String){
    println!("Hello {}!", name);
}

#[test]
fn test_hi() {
    let number = 10;
    print_number(number);
    println!("{}", number);

    let name = String::from("Satriyo");
    hi(name);

    // gabisa karena name udah masuk ke function  heap
    // println!("{}", name)
}

fn full_name(first_name:&str, last_name:&str) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name=String::from("Satriyo");
    let last_name=String::from("Noto");
    let new_fullname = full_name(&first_name, &last_name);
    println!("{}", new_fullname);
}

fn new_fullname(first_name:String, last_name:String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);
    (first_name, last_name, full_name)
}

#[test]
fn test_new_fullname(){
    let first_name = String::from("Satriyo");
    let last_name = String::from("Noto");
    let (a, b, c) = new_fullname(first_name, last_name);
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}