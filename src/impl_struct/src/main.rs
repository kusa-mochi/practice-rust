struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn say_name(&self) -> &Self {
        println!("name: {}", self.name);
        self
    }

    fn say_age(&self) -> &Self {
        println!("age: {}", self.age);
        self
    }

    fn say_custom(&self, s: String) -> &Self {
        println!("custom: {}", s);
        self
    }
}

fn main() {
    let p = Person{
        name: String::from("Mochi"),
        age: 123,
    };

    p.say_name().say_age().say_custom(String::from("Kusa"));
}
