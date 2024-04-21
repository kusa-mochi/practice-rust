enum Emotion {
    Anger,
    Happy,
}

struct HappyPersion {
    name: String,
    state: Emotion,
}

trait Emotional {
    fn get_happy(&mut self) -> String;
    fn get_anger(&mut self) -> String;
    fn tell_state(&self) -> String;
}

impl Emotional for HappyPersion {
    fn get_happy(&mut self) -> String {
        format!("{} is always happy :)", self.name)
    }
    
    fn get_anger(&mut self) -> String {
        // 実装予定なし。
        // String型を返さなくても型検査に通るようにする。
        unimplemented!()
    }
    
    fn tell_state(&self) -> String {
        // 実装予定。
        // 一旦、String型を返さなくても型検査に通るようにする。
        todo!()
    }
}

fn main() {
    let mut p = HappyPersion {
        name: String::from("Tayo"),
        state: Emotion::Happy,
    };
    println!("{}", p.get_happy());
}
