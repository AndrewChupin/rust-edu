use std::fmt::Debug;

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

trait HasArea {
    fn is_invalid(&self) -> bool { self.area() > 23 } // Default method
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl HasArea for i32 {
    fn area(&self) -> f64 {
        println!("это нелепо");

        *self as f64
    }
}

fn bar<T, K>(x: T, y: K)
    where
        T: HasArea,
        K: HasArea + Debug {

    x.clone();
    y.clone();
    println!("{:?}", y);
}

fn some<T: HasArea + Debug>(t: T) {
    // TODO
}

// Convert
trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

impl ConvertTo<i64> for i32 {
    fn convert(&self) -> i64 { *self as i64 }
}

// может быть вызван с T == i32
fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
    x.convert()
}

// может быть вызван с T == i64
fn inverse<T>() -> T
// использует ConvertTo как если бы это было «ConvertTo<i64>»
    where i32: ConvertTo<T> {
    32.convert()
}



// Multi traits
trait Foo {
    fn foo(&self);
}

trait FooBar : Foo {
    fn foobar(&self);
}

struct Baz;

impl Foo for Baz {
    fn foo(&self) { println!("foo"); }
}

impl FooBar for Baz {
    fn foobar(&self) { println!("foobar"); }
}


// Trait DROP
struct Firework {
    strength: i32,
}

impl Drop for Firework {
    fn drop(&mut self) {
        println!("БАБАХ силой {}!!!", self.strength);
    }
}

fn main() {
    let firecracker = Firework { strength: 1 };
    let tnt = Firework { strength: 100 };

    // Вызываются в обратном порядке объявления
    // БАБАХ силой 100!!!
    // БАБАХ силой 1!!!
}