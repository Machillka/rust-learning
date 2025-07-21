pub struct Person {
    pub name: String,
    pub height: f32,
    pub weight: f32
}

impl Person {
    pub fn init(name:String, height: f32, weight: f32) -> Person {
        Person{ name, height, weight }
    }

    pub fn calculate_bmi(&self) -> f32 {
        self.height * self.weight
    }
}