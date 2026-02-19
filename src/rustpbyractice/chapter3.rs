/// https://practice.course.rs/variables.html
#[test]
fn test31() {
    let x1: i32 = 5; // Uninitialized but used, ERROR !
    let _y1: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x1, 5);
    println!("Success!");
}
#[test]
fn test32() {
    let mut x2 = 1;
    x2 += 2; 
    
    assert_eq!(x2, 3);
    println!("Success!");
}
#[test]
fn test33() {
    let x3: i32 = 10;
    let _y3: i32 = 5;
    {
        println!("Inner scope value of x is {} and value of y is {}", x3, _y3);
    }
    println!("Outer scope value of x is {} and value of y is {}", x3, _y3); 
}
#[test]
fn test34() {
    let _x4 = "hello";
    println!("{}, world", _x4); 
}
#[test]
fn test35() {
    let x5: i32 = 5;
    {
        let x5 = 12;
        assert_eq!(x5, 12);
    }
    assert_eq!(x5, 5);
    let x5 = 42;
    println!("{}", x5); // Prints "42".
}
#[test]
fn test36() {
    let mut _x6: i32 = 1;
    _x6 = 7;
    // Shadowing and re-binding
    let mut _x6 = _x6; 
    _x6 += 3;
    println!("{}",_x6);
    let _y6 = 4;
    // Shadowing
    let y6 = "I can also be bound to text!"; 
    println!("{}", y6);
    println!("Success!");
}
#[test]
fn test37() {
    let x7 = 1;
    println!("{}", x7)
}
#[test]
fn test38() {
    let (x8, y8) = (1, 2);
    let mut x8 = x8;
    x8 += 2;

    assert_eq!(x8, 3);
    assert_eq!(y8, 2);

    println!("Success!");
}
#[test]
fn test39() {
    let (x9, y9);
    (x9,..) = (3, 4);
    [.., y9] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x9,y9], [3, 2]);

    println!("Success!");
}