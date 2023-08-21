# iterator-assertions

[![paypal](https://img.shields.io/badge/Support_my_work-PayPal-green.svg)](https://www.paypal.com/donate/?hosted_button_id=E648MA54L53J6)

Assert something about an iterator in place.

```rust
let vector = vec![1, 2, 3]
    .assert(|i| i.is_empty().not())
    .assert(|i| i.len() < 5);
```
