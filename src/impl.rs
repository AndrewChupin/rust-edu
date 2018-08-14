
struct Rect {
    width: i32,
    height: i32
}

impl Rect {

    // instance method
    fn improve(&self, w: i32, h: i32) -> Rect {
        Rect { width: self.height + w, height: self.height + h }
    }

    // instance method
    fn area(&self) -> i64 {
        (self.width + self.height) as i64
    }

    // static method
    fn init(w: i32, h: i32) -> Rect {
        Rect { width: w, height: h }
    }
}

fn execute() {
    let rect = Rect::init(20, 30);
    let result = rect.improve(12, 12).area();
    print!("area is {}", result)
}

struct RectBuilder {
    width: i32,
    height: i32,
}

impl RectBuilder {
    fn new() -> RectBuilder {
        RectBuilder { width: 0, height: 0 }
    }

    fn width(&mut self, width: i32) -> &mut RectBuilder {
        self.width = width;
        self
    }

    fn height(&mut self, height: i32) -> &mut RectBuilder {
        self.height = height;
        self
    }

    fn finalize(&self) -> Rect {
        Rect { width: self.width, height: self.height }
    }
}

fn execute_builder() {
    let c = RectBuilder::new()
        .width(1)
        .height(2)
        .finalize();

    println!("площадь: {}", c.area());
    println!("x: {}", c.width);
    println!("y: {}", c.height);
}