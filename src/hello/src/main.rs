fn main() {
    let result: Result<i32, String> = Ok(123);

    match result {
        Ok(ref code) => println!("ok({})", code),
        Err(ref err) => println!("err({})", err),
    };

    println!("code: {}", result.unwrap_or(-1));
}
