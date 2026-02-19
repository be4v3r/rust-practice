/// https://practice.course.rs/compound-types/intro.html
#[test]
fn test611() {
    let _s1: &str = "hello, world";
}
#[test]
fn test612() {
    let s2: Box<str> = "hello, world".into();
    greetings2(&s2);

    fn greetings2(s2: &str) {
        println!("{}", s2);
    }
}
#[test]
fn test613() {
    let mut s3 = String::new();
    s3.push_str("hello, world");
    s3.push('!');

    assert_eq!(s3, "hello, world!");
}
#[test]
fn test614() {
    let mut s4 = String::from("hello");
    s4.push(',');
    s4.push_str(" world");
    s4 += "!";

    println!("{}", s4);
}
#[test]
fn test615() {
    let s5 = String::from("I like dogs");
    let s5_1 = s5.replace("dogs", "cats");

    assert_eq!(s5_1, "I like cats");
}
#[test]
fn test616() {
    let s6_1 = String::from("hello,");
    let s6_2 = String::from("world!");
    let s6_3 = s6_1.clone() + &s6_2;
    assert_eq!(s6_3, "hello,world!");
    println!("{}", s6_1);
}
#[test]
fn test617() {
    let s7 = String::from("hello, world");
    greetings7(s7);

    fn greetings7(s7: String) {
        println!("{}", s7);
    }
}
#[test]
fn test618() {
    let s8 = "hello, world".to_string();
    let _s8_1: &str = &s8;

    let s8b = "hello, world";
    let _s8b_1: &str = s8b;

    let s8c = "hello, world".to_string();
    let _s8c_1: String = s8c;
}
#[test]
fn test619() {
    let byte_escape9 = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape9);

    let unicode_codepoint9 = "\u{211D}";
    let character_name9 = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}", unicode_codepoint9, character_name9);

    let long_string9 = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                        can be escaped too!";
    println!("{}", long_string9);
}
#[test]
fn test6110() {
    let raw_str10 = "Escapes don't work here: ? ℝ";
    assert_eq!(raw_str10, "Escapes don't work here: ? ℝ");

    let quotes10 = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes10);

    let delimiter10 = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter10);

    let long_delimiter10 = r###"Hello, "##""###;
    assert_eq!(long_delimiter10, "Hello, \"##\"");
}
#[test]
fn test6111() {
    let s11_1 = String::from("hi,中国");
    let h11 = &s11_1[0..1];
    assert_eq!(h11, "h");

    let h11_1 = &s11_1[3..6];
    assert_eq!(h11_1, "中");
}
#[test]
fn test6112() {
    for c12 in "你好，世界".chars() {
        println!("{}", c12);
    }
}
#[test]
fn test621() {
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    assert!(arr1.len() == 5);
}
#[test]
fn test622() {
    let _arr2_0 = [1, 2, 3];
    let arr2: [_; 3] = ['a', 'b', 'c'];
    assert!(std::mem::size_of_val(&arr2) == 12);
}
#[test]
fn test623() {
    let list3: [i32; 100] = [1; 100];
    assert!(list3[0] == 1);
    assert!(list3.len() == 100);
}
#[test]
fn test624() {
    let _arr4 = [1, 2, 3];
}
#[test]
fn test625() {
    let arr5 = ['a', 'b', 'c'];
    let ele5 = arr5[0];
    assert!(ele5 == 'a');
}
#[test]
fn test626() {
    let names6 = [String::from("Sunfei"), "Sunface".to_string()];
    let _name0_6 = names6.get(0).unwrap();
    let _name1_6 = &names6[1];
}
#[test]
fn test631() {
    let arr1 = [1, 2, 3];
    let _s1_1: &[i32] = &arr1[0..2];
    let _s1_2: &str = "hello, world";
}
#[test]
fn test632() {
    let arr2: [char; 3] = ['中', '国', '人'];
    let slice2 = &arr2[..2];
    assert!(std::mem::size_of_val(&slice2) == 16);
}
#[test]
fn test633() {
    let arr3: [i32; 5] = [1, 2, 3, 4, 5];
    let slice3: &[i32] = &arr3[1..4];
    assert_eq!(slice3, &[2, 3, 4]);
}
#[test]
fn test634() {
    let s4 = String::from("hello");
    let slice1_4 = &s4[0..2];
    let slice2_4 = &s4[..2];

    assert_eq!(slice1_4, slice2_4);
}
#[test]
fn test635() {
    let s5 = "你好，世界";
    let slice5 = &s5[0..3];
    assert!(slice5 == "你");
}
#[test]
fn test636() {
    let mut s6 = String::from("hello world");
    let letter6 = first_letter6(&s6);
    println!("the first letter is: {}", letter6);
    s6.clear();
    fn first_letter6(s6: &str) -> &str {
        &s6[..1]
    }
}
#[test]
fn test641() {
    let _t0_1: (u8, i16) = (0, -1);
    let _t1_1: (u8, (i16, u32)) = (0, (-1, 1));
    let _t1: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
}
#[test]
fn test642() {
    let t2 = ("i", "am", "sunface");
    assert_eq!(t2.2, "sunface");
}
#[test]
fn test643() {
    let too_long_tuple3 = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple3);
}
#[test]
fn test644() {
    let tup4 = (1, 6.4, "hello");

    let (x4, z4, y4) = tup4;

    assert_eq!(x4, 1);
    assert_eq!(y4, "hello");
    assert_eq!(z4, 6.4);
}
#[test]
fn test645() {
    let (x5, y5, z5);

    (y5, z5, x5) = (1, 2, 3);

    assert_eq!(x5, 3);
    assert_eq!(y5, 1);
    assert_eq!(z5, 2);
}
#[test]
fn test646() {
    let (x6, y6) = sum_multiply6((2, 3));

    assert_eq!(x6, 5);
    assert_eq!(y6, 6);

    fn sum_multiply6(nums6: (i32, i32)) -> (i32, i32) {
        (nums6.0 + nums6.1, nums6.0 * nums6.1)
    }
}
#[test]
fn test651() {
    #[allow(dead_code)]
    struct Person1 {
        name: String,
        age: u8,
        hobby: String,
    }
    let age1 = 30;
    
    let _p1 = Person1 {
        name: String::from("sunface"),
        age: age1,
        hobby: "coding".to_string(),
    };
}
#[test]
fn test652() {
    struct Unit2;
    #[allow(dead_code)]
    trait SomeTrait2 {}
    impl SomeTrait2 for Unit2 {}

    let u2 = Unit2;
    do_something_with_unit2(u2);

    fn do_something_with_unit2(_u: Unit2) {}
}
#[test]
fn test653() {
    #[allow(dead_code)]
    struct Color3(i32, i32, i32);
    struct Point3(i32, i32, i32);
    let v3 = Point3(0, 127, 255);
    check_color3(v3);

    fn check_color3(p3: Point3) {
        let Point3(x3, _, _) = p3;
        assert_eq!(x3, 0);
        assert_eq!(p3.1, 127);
        assert_eq!(p3.2, 255);
    }
}
#[test]
fn test654() {
    struct Person4 {
        name: String,
        age: u8,
    }
    let age4 = 18;
    let mut p4 = Person4 {
        name: String::from("sunface"),
        age: age4,
    };

    p4.age = 30;
    p4.name = String::from("sunfei");
}
#[test]
fn test655() {
    #[allow(dead_code)]
    struct Person5 {
        name: String,
        age: u8,
    }
    #[allow(dead_code)]
    fn build_person5(name5: String, age5: u8) -> Person5 {
        Person5 { age: age5, name: name5 }
    }
}
#[test]
fn test656() {
    #[allow(dead_code)]
    struct User6 {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    let u1_6 = User6 {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };
    let _u2_6 = set_email6(u1_6);

    fn set_email6(u6: User6) -> User6 {
        User6 {
            email: String::from("contact@im.dev"),
            ..u6
        }
    }
}
#[test]
fn test657() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Rectangle7 {
        width: u32,
        height: u32,
    }
    let scale7 = 2;
    let rect1_7 = Rectangle7 {
        width: dbg!(30 * scale7),
        height: 50,
    };

    dbg!(&rect1_7);

    println!("{:?}", rect1_7);
}
#[test]
fn test658() {
    #[derive(Debug)]
    struct File8 {
        name: String,
        data: String,
    }
    let f8 = File8 {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let _name8 = f8.name;

    println!("{}", f8.data);
}
#[test]
#[allow(dead_code)]
fn test661() {
    enum Number1 { Zero, One, Two }
    enum Number2 { Zero = 0, One, Two }
    enum Number3 { Zero = 0, One = 1, Two = 2 }

    assert_eq!(Number1::One as u8, Number2::One as u8);
    assert_eq!(Number2::One as u8, Number3::One as u8);
}
#[test]
fn test662() {
    #[allow(dead_code)]
    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let _msg1_2 = Message2::Move{x: 1, y: 2};
    let _msg2_2 = Message2::Write(String::from("hello, world"));
}
#[test]
fn test663() {
    #[allow(dead_code)]
    enum Message3 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg3 = Message3::Move{x: 1, y: 1};

    if let Message3::Move{x: a3, y: b3} = msg3 {
        assert_eq!(a3, b3);
    } else {
        panic!("NEVER LET THIS RUN！");
    }
}
#[test]
fn test664() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Message4 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msgs4: [Message4; 3] = [
        Message4::Quit,
        Message4::Move { x: 1, y: 3 },
        Message4::ChangeColor(255, 255, 0)
    ];

    for msg4 in msgs4 {
        show_message4(msg4)
    }

    fn show_message4(msg4: Message4) {
        println!("{:?}", msg4);
    }
}
#[test]
fn test665() {
    let five5 = Some(5);
    let six5 = plus_one5(five5);
    let _none5 = plus_one5(None);

    if let Some(n5) = six5 {
        println!("{}", n5);
        return;
    }
    fn plus_one5(x5: Option<i32>) -> Option<i32> {
        match x5 {
            None => None,
            Some(i5) => Some(i5 + 1),
        }
    }
}
#[test]
fn test666() {
    enum List6 {
        Cons(u32, Box<List6>),
        Nil,
    }

    impl List6 {
        fn new() -> List6 {
            List6::Nil
        }

        fn prepend(self, elem6: u32) -> List6 {
            List6::Cons(elem6, Box::new(self))
        }

        fn len(&self) -> u32 {
            match *self {
                List6::Cons(_, ref tail6) => 1 + tail6.len(),
                List6::Nil => 0,
            }
        }

        fn stringify(&self) -> String {
            match *self {
                List6::Cons(head6, ref tail6) => format!("{}, {}", head6, tail6.stringify()),
                List6::Nil => format!("Nil"),
            }
        }
    }

    let mut list6 = List6::new();

    list6 = list6.prepend(1);
    list6 = list6.prepend(2);
    list6 = list6.prepend(3);

    println!("linked list has length: {}", list6.len());
    println!("{}", list6.stringify());
}
