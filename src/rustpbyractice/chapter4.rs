/// https://practice.course.rs/basic-types/intro.html
/// to do the rest
#[test]
fn test411() {
    let x1: i32 = 5;
    let mut _y1: i32 = 5;

    _y1 = x1;
    
    let _z1 = 10; // Type of z ? 

    println!("Success!");
}
#[test]
fn test412() {
    let _v2: u16 = 38_u8 as u16;

    println!("Success!");
}
#[test]
fn test413() {
    let x3 = 5;
    assert_eq!("i32".to_string(), type_of(&x3));

    println!("Success!");
}
#[allow(dead_code)]
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
#[test] 
fn test414() {
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 

    println!("Success!");
}
#[test]
fn test415() {
    let v15: u16 = 251_u16 + 8;
    let v25: u16 = u16::checked_add(251, 8).unwrap();
    println!("{},{}",v15,v25);
}
#[test]
fn test416() {
    let v6 = 1024 + 0xff + 0o77 + 0b11111111;
    assert!(v6 == 1597);
    println!("Success!");
}
#[test]
fn test417() {
    let x7 = 1_000.000_1; // ?
    let _y7: f32 = 0.12; // f32
    let _z7 = 0.01_f64; // f64

    assert_eq!(type_of(&x7), "f64".to_string());
    println!("Success!");
}
#[test]
fn test418() {
    assert!(0.1f32+0.2f32==0.3f32);
    println!("Success!");
}
#[test]
fn test419() {
    let mut sum = 0;
    for i9 in -3..2 {
        sum += i9
    }

    assert!(sum == -5);

    for c9 in 'a'..='z' {
        println!("{}",c9 as u8);
    }
}
#[test]
fn test4110() {
    assert_eq!((1..5), std::ops::Range{ start: 1, end: 5 });
    assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5));
}
#[test]
fn test4111() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);
    
    assert!(3 * 50 == 150);

    assert!(9 / 3 == 3); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
#[test]
fn test421() {
    let c11 = 'a';
    assert_eq!(size_of_val(&c11), 4);
    let c21 = '中';
    assert_eq!(size_of_val(&c21), 4);
}
#[test]
fn test422() {
    let c1 = '中';
    print_char(c1);
}
#[allow(dead_code)]
fn print_char(c: char) {
    println!("{}", c);
}
#[test]
fn test423() {
    let _f3: bool = false;

    let t3 = false;
    if !t3 {
        println!("hello, world");
    }
} 
#[test]
fn test424() {
    let f4 = true;
    let t4 = true || false;
    assert_eq!(t4, f4);
}
#[test]
fn test425() {
    let v05: () = ();

    let _v5 = (2, 3);
    assert_eq!(v05, implicitly_ret_unit())
}
#[allow(dead_code)]
fn implicitly_ret_unit() {
    println!("I will return a ()")
}
#[test]
fn test426() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);
}
#[test]
fn test431() {
    let v1 = {
        let mut x1 = 1;
        x1 += 2;
        x1
    };

    assert_eq!(v1, 3);
}
#[test]
fn test432() {
    let v2 = {
        let x2 = 3;
        x2
    };

    assert!(v2 == 3);
}
#[test]
fn test433() {
    let s3 = sum3(1, 2);
    assert_eq!(s3, 3);

    fn sum3(x3: i32, y3: i32) -> i32 {
        x3 + y3
    }
}
#[test]
fn test441() {
    let (x1, y1) = (1, 2);
    let s1 = sum1(x1, y1);

    assert_eq!(s1, 3);

    fn sum1(x1: i32, y1: i32) -> i32 {
        x1 + y1
    }
}
#[test]
fn test442() {
    print2();
    fn print2() -> () {
        println!("hello,world");
    }
}
#[test]
fn test444() {
    println!("Success!");
    #[allow(dead_code)]
    fn get_option4(tp4: u8) -> Option<i32> {
        match tp4 {
            1 => None,
            _ => None,
        }
    }
    #[allow(dead_code)]
    fn never_return_fn4() -> ! {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1))
        }
    }
}