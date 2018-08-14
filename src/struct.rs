// Struct
struct Point {
    x: i32,
    y: i32, // нельзя поставить mut, стуктуры имеют внутренную неизменяемость
}

fn struct_fun() {
    let mut a = Point { x: 5, y: 6 };
    a.x = 10;

    let b = Point { x: 5, y: 6};
    // error for b.x = 10;
}

// Generic Struct
struct PointRef<'a> {
    x: &'a mut i32,
    y: &'a mut i32,
}

fn get_struct() {
    let mut point = Point { x: 0, y: 0 };

    {
        let r = PointRef { x: &mut point.x, y: &mut point.y };
        *r.x = 5;
        *r.y = 6;
    }

    assert_eq!(5, point.x);
    assert_eq!(6, point.y);
}

// Update struct
struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

fn update_struct() {
    let mut point = Point3d { x: 0, y: 0, z: 0 };
    point = Point3d { y: 1, .. point }; // обновляет y, но оставляет старыми x и z
}

// Couple struct
struct Color(i32, i32, i32);

fn couple_struct() {
    let black = Color(0, 0, 0);
}

// Clone struct
struct Inches(i32);

fn clone_struct() {
    let length = Inches(10);

    let Inches(integer_length) = length;
    println!("Длина в дюймах: {}", integer_length);
}

// Unit struct
struct Electron;

fn unit() {
    let x = Electron;
}