

fn closure() {
    let plus_one = |x: i32| x + 1;

    assert_eq!(2, plus_one(1));
}

fn closure_arg(hello: fn(i32) -> i32) {
    hello(32);
}

fn execute_closure_arg() {
    closure_arg(|x: i32| {
        x + 23
    })
}


fn move_func() {
    let mut num = 5;
    {
        let mut add_num = |x: i32| num += x;
        add_num(5);
    }
    assert_eq!(10, num);


    let mut num = 5;
    {
        let mut add_num = move |x: i32| num += x;
        add_num(5);
    }
    assert_eq!(5, num);
}


fn alternative_fn_closure() {
    fn call_with_one<F>(some_closure: F) -> i32
        where F : Fn(i32) -> i32 {

        some_closure(1)
    }

    let answer = call_with_one(|x| x + 2);

    assert_eq!(3, answer);
}

fn ref_closure() {
    fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
        some_closure(1)
    }

    let answer = call_with_one(&|x| x + 2);

    assert_eq!(3, answer);
}

fn return_fn() {
    fn factory() -> Box<Fn(i32) -> i32> {
        let num = 5;

        Box::new(move |x| x + num)
    }
    let f = factory();

    let answer = f(1);
    assert_eq!(6, answer);
}