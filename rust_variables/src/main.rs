fn main(){
    // let x = 6;

    // if x != 0 {
    //    println!("X is present in the code");
    // }

    // let condition : bool = true;
    // let number = if condition {5} else {6};
    // println!("The value of number is: {}", number);


    // let mut number = 3;

    // while number != 0 {
    //     println!("{}!", number);

    //     number = number - 1;
    // }

    // println!("LIFTOFF!!!");

    // let a =[10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index = index + 1;
    // }

    // for element in a.iter(){
    //     println!("The value is: {}", element);
    // }


    // for number in (1..4).rev(){
    //     println!("{}!", number);
    // }

//     let celsius: f32 = 12.00;
//     let fahrenheits: f32 = 45.00;

//     let firstcomputation: f32 = celsius_to_fahrenheits(celsius);
//     let secondcomputation: f32 = fahrenheits_to_celsius(fahrenheits);

//     println!("Converting {} to fahrenheits, we have: {}", celsius, firstcomputation);
//     println!("Converting {} to celsius we have: {}", fahrenheits, secondcomputation);
    

// }

// fn celsius_to_fahrenheits(x: f32) -> f32 {
//     x * 32.00
// }

// fn fahrenheits_to_celsius(y: f32) -> f32 {
//     32.00 


    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;  

    // println!("{}", s);

    makes_copy(x);

    println!("{}", x);

}

fn takes_ownership(some_string: String){
    println!("{}", some_string);

}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn fibonacci_sequence(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }

    b
}

