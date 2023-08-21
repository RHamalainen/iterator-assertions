use iterator_assertions::IteratorAssert;
use push_in_place::PushInPlace;

fn main() {
    let mut vector = Vec::new();
    for i in 0..10 {
        println!("{i}");
        vector = vector.push_in_place(i).assert(|i| i.len() < 8);
    }
}
