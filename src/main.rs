pub mod gramma;

use gramma::ownership;
use gramma::structure;


fn main() {
    ownership::variables_test();
    ownership::ownership();
    ownership::try_match_test();

    let weight: f32 = 1.2;
    let tom = structure::Person::init(String::from("Tom"), 1.1, weight);
    println!("{0}, {1}", tom.weight, weight);               // 因为是简单类型 所以不会被拿走所有权
}