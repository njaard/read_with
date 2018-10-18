[![GitHub license](https://img.shields.io/badge/license-BSD-blue.svg)](https://raw.githubusercontent.com/njaard/read_with/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/read_with.svg)](https://crates.io/crates/read_with)

This implements the `Read` trait, calling a function
to generate the data.

See the [API documentation](https://docs.rs/read_with).

# Import Crate

`read_with="0.1"`

# Example

```
let mut output = vec!();
let many_strings = ["one", "two", "three"];
let mut pos = 0;

std::io::copy(
    &mut ReadWith::new(
        ||
        {
            if pos == many_strings.len() { return None; }
            let o = many_strings[pos];
            pos+=1;
            Some(o)
        }
    ),
    &mut output,
).unwrap();
assert_eq!("onetwothree", str::from_utf8(&output).unwrap());
```
