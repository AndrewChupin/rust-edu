

fn primary() {
    // Call fn
    stared();

    // Get result
    let ret = fun_with_arg(23);
    assert_eq!(ret, 54);
    types()
}


/// # Types
// i8 i16 i32 i64 u8 u16 u32 u64 isize usize f32 f64
fn types() {
    let i = 32;
    let b = true;
    let c = 'c';
    let sym = '💕';
    let f = 1.0;

    // array
    let m = [1, 3, 4];
    // array reference
    let m_tail = &m[2..];

    // str reference
    let str = "string";

    // couples
    let couple = (1,3,4);
    let fix_couple: (i32, &str) = (1, "hello");
    let tuple = fix_couple.1;

    // fn pointer
    let func: fn(i32) -> i32 = fun_with_arg;
}

fn exp() {
    let mut a = 4;
    let mut is_done = false;

    // If
    let new = if a == 4 {
        1
    } else if a == 5 {
        2
    } else {
        3
    };

    // Loop
    loop {
        // Infinity loop
    }

    // While
    while !is_done {
        a += 1;
        is_done = true;
        break;
    }

    // For
    for x in 0..10 {
        println!("New number {}", x)
    }

    for (i,j) in (5..10).enumerate() {
        continue;
        println!("i = {} и j = {}", i, j);
    }

    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; } // продолжает цикл по x
            if y % 2 == 0 { continue 'inner; } // продолжает цикл по y
            println!("x: {}, y: {}", x, y);
        }
    };

    // match
    match new {
        1 | 4 if true => println!("1"),
        2 => println!("2"),
        tie @ 3..7 => println!("{}", tie),
        _ => println!("else"),
    };

    match new {
        ref r => println!("Получили ссылку на {}", r),
    }

    match new {
        ref mut r => println!("Получили ссылку на {}", r),
    }
}

fn stared() {
    // Immutable
    let var: i32 = 12;
    assert_eq!(var, 12);
    println!("var is {}", var);

    // Mutable
    let mut var_mut= 12;
    var_mut = 44;
    assert_eq!(var_mut, 44);
    println!("var_mut is {}", var_mut);

    // String
    let string = "first string";
    assert_eq!(string, "first string");
    println!("string is {}", string);
}


fn fun_with_arg(num: i32) -> i32 {
    num + 31
}

#[allow(dead_code)]
fn diverges() -> ! {
    panic!("Эта функция не возвращает управление!"); // raise or throw
}


// References
///
/// одна или более неизменяемых ссылок (&T) на ресурс;
/// ровно одна изменяемая ссылка (&mut T) на ресурс.
///
fn references() {
    let arr = vec![1, 3, 3];
    let arr = moving(arr); // return value

    let mut arr_mut = vec![1, 3, 3];
    moving_ref(arr_mut);
    let _x = &mut arr_mut; // Для ссылки объекта _ спереди
    let _y = &mut arr_mut; // error here
    let z = &arr_mut; // error here
}

fn window() {
    let mut arr_mut = vec![1, 3, 3];

    for i in &arr_mut {
        println!("New {}", i);
        arr_mut.push(13); // error &arr_mut заимствован в цикле
    }
}

fn lifecycle() {
    let a: &i32;

    {
        let x = 4;
        a = &x;
    }

    println!("Num {}", a); // error [x] is dead
}

fn refs() {
    let x = vec![1,3,4];
    let mut z = vec![1,3,4];

    let _a = &x;
    let _y = &mut z;
}

fn moving(in_arr: Vec<i32>) -> Vec<i32> {
    in_arr
}

fn moving_ref(in_arr: &mut Vec<i32>) {
    in_arr.push(5)
}

// Immutable/Mutable
use std::sync::Arc;
fn clone() {
    let x = Arc::new(5);
    let _y = x.clone(); // immutable ref
}

use std::cell::RefCell;
fn ref_cell() {
    let x = RefCell::new(42);
    let y = x.borrow_mut(); // Mutable ref
    let z = x.borrow_mut(); // panic! more than one mutable ref
}