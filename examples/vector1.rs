use iterator_assertions::IteratorAssert;

fn main() {
    let _vector = vec![1, 2, 3, 4, 5]
        .assert(|i| i.len() == 5)
        .assert(|i| i.is_empty());
}
