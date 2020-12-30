![Build, Test](https://github.com/Kelerchian/assign/workflows/Build,%20Test/badge.svg)

# Assign

Mutate instances with declarative flavor!

This module provides macro `assign!` to allow mutating instance with declarative flavor

The motivation of this macro is to enable programmer to document a sequence of mutations instance fields as initialization by writing it in a declarative way. `assign!` macro also allows programmer to skip defining fields that has default value. Such case are used when a dependency is exposing an non-exhaustive struct

## Usage

```rust
#[macro_use]
extern crate assign;

fn main() {
    struct SomeStruct {
        a: u32,
        b: Option<f32>,
        c: String,
    }
    impl SomeStruct {
        fn new() -> SomeStruct {
            SomeStruct {
                a: 1u32,
                b: None,
                c: String::from("old"),
            }
        }
    }

    // In order to treat the mutation of field `a` and `c` as an initialization,
    // Use assign to mutate field in declarative flavor, thus avoiding the risk inserting code
    // between the line that defines a field and the line that defines the other
    // Note that field `b` is skipped
    let instance = assign!(SomeStruct::new(), {
      a: 2u32,
      c: String::from("new"),
    });

    // Equivalent
    let instance2 = {
        let mut item = SomeStruct::new();
        item.a = 2u32;
        item.c = String::from("new");
        item
    };

    assert_eq!(instance.a, instance2.a);
    assert_eq!(&instance.c, &instance2.c);
    assert_eq!(instance.b, instance2.b);
}
```

## License

[MIT](LICENSE)
