fn main() {
    let x = add(1, 2);
    println!("x = {}", x);

    let y = abs(-5);
    println!("y = {}", y)
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn abs(a: i32) -> i32 {
    if a < 0 {
        return -a;
    }

    a
}
