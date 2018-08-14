struct Sample<T> {
    x: T
}

fn sample() {
    let x = Sample { x: 32 };
    println!("x {}", x.x);

    let x: Option<i32> = Some(5);
    match x {
        Some(val) => println!("Some {}", val),
        None =>  println!("None"),
    }

    if let val = x {
        println!("Some {}", val)
    } else {
        println!("None")
    }
}

fn gen_func<T>(t: T) {
    // TODO SOMETHING
}