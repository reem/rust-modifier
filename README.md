# Rust-Modifier

> Chaining APIs for both `self -> Self` and `&mut self` methods.

## Example

```rust
let mut thing = Thing { x: 6 };
thing.set_mut(ModifyX(8));
assert_eq!(thing.x, 8);

let thing = thing.set(ModifyX(9));
assert_eq!(thing.x, 9);
```

## Overview

Rust-modifier allows you to define modifiers just once, then
use them through both `set` and `set_mut`, allowing downstream
users the ability to use whichever API is most convenient.

Additionally, rust-modifier allows users to define their own
modifiers, arbitrarily extending the utility of existing types.

## LICENSE

MIT

