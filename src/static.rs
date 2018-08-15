static mut N: i32 = 5;
const con: i32 = 5; // inline and need specify type

fn test() {
    unsafe { // sync
        N += 1;
        println!("N: {}", N);
    }
}