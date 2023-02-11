fn main(){
    fn match_number(n: i32) {
        match n {
            1 => println!("One!"),
            2|3|4|5 => println!("match 2->5"),
            6..=10 => println!("match 6->10"),
            _ => println!("match 11-> +infinite"),
        }
    }
    for i in 0..=11 {
        print!("{}\t", i);
        match_number(i);
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 6, y: 0 };
    match p { 
        Point{x, y: 0} => println!("On the x axis at {}", x),
        Point{x: 0..=5, y: y@ (10| 20 | 30) } => println!("On the y axis at {}", y),
        Point{x,y} => println!("On neither axis: ({}, {})", x, y),
    }
    enum Message {
        Hello {id: i32},
    }
    let msg = Message::Hello {id: 12};
    match msg {
        Message::Hello {
            id: id@3..=7,
        } => println!("id 值的范围在 [3,7] 之间： {}", id),
        Message::Hello {
            id: newid@ (10 | 11 | 12)
        } => println!("id 值的范围在 [10,12] 之间： {}", newid),
        Message::Hello {id} => println!("Found some other id: {}", id),
    }

    let num = Some(8);
    let split = 5;
    match num {
        Some(x) if x<split => {
            assert!(x < split);
            println!("the x is {}, split is {}", x, split);
        },
        Some(x) => assert!(x >= split),
        None => (),
    }
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);
    match numbers {
        (first,..,last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }

    let mut v = String::from("hello,");
    let r = &mut v;
    match r {
        value => value.push_str(" world!")
    }
    println!("the value is {}", v);
}