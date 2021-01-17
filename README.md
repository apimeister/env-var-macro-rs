# env-var
[![Latest Version](https://img.shields.io/crates/v/env-var.svg)](https://crates.io/crates/env-var)

Macros providing easier access to environment variables in rust.

# License
env-var is dual-licensed under Apache License, Version 2.0 and MIT license.

## Usage

Put this in your `Cargo.toml`:

```text
[dependencies]
env-var = "1"
log = "0.4"
```


## Examples

```rust
#[macro_use]
extern crate env_var;

fn main() {
  // retrieving a optional value
  // var1 either contains the value or an empty string
  let var1 = env_var!(optional "TEST_OPT_1");

  // retrieving a optional value with default
  // var1 either contains the value or "default1"
  let var1 = env_var!(optional "TEST_OPT_1", default: "default1");

  // retrieving a optional value with default
  // if not found, the message will be logged as info message
  // var1 either contains the value or "default1"
  let var1 = env_var!(optional "TEST_OPT_1", default: "default1", msg: "using default");

  // retrieving a required value
  // var1 either contains the value or the programm panics
  let var1 = env_var!(required "TEST_OPT_1");

  // retrieving a required value with default
  // var1 either contains the value or "default1"
  let var1 = env_var!(required "TEST_OPT_1", default: "default1");

  // retrieving a required value with default
  // if not found, the message will be logged as warn message
  // var1 either contains the value or "default1"
  let var1 = env_var!(required "TEST_OPT_1", default: "default1", msg: "using default");

}
```
