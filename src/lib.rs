//! This module provides the `assign!` macro to allow mutating a struct value in
//! a declarative style.
//!
//! It is an alternative to [struct update syntax][] that works with
//! [non-exhaustive][] structs.
//!
//! [struct update syntax]: https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax
//! [non-exhaustive]: https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute
//!
//! It is used as
//!
//! ```
//! # use assign::assign;
//! # struct Struct { field: u8 }
//! # let init_expression = Struct { field: 0 };
//! # let new_field_value = 1;
//! let foo = assign!(init_expression, {
//!     field: new_field_value,
//!     // other_field: new_other_field_value,
//!     // ...
//! });
//! ```
//!
//! For details and examples, see the documentation for the macro itself.
#![no_std]

/// Mutate a struct value in a declarative style.
///
/// # Basic usage
///
/// ```
/// # use assign::assign;
/// #
/// #[non_exhaustive]
/// #[derive(Debug, PartialEq)]
/// struct SomeStruct {
///     a: u32,
///     b: Option<f32>,
///     c: String,
/// }
///
/// impl SomeStruct {
///     fn new() -> Self {
///         // ...
/// #       SomeStruct {
/// #           a: 1u32,
/// #           b: None,
/// #           c: String::from("old"),
/// #       }
///     }
/// }
///
/// let instance1 = assign!(SomeStruct::new(), {
///     a: 2,
///     c: "new".into(),
/// });
///
/// // The same thing using mutation explicitly.
/// // This is what the above expands to.
/// let instance2 = {
///     let mut item = SomeStruct::new();
///     item.a = 2;
///     item.c = "new".into();
///     item
/// };
///
/// // The same thing using struct update syntax (does not work for
/// // non-exhaustive structs defined in external crates).
/// let instance3 = SomeStruct {
///     a: 2,
///     c: "new".into(),
///     ..SomeStruct::new()
/// };
///
/// assert_eq!(instance1, instance2);
/// assert_eq!(instance1, instance3);
/// ```
///
/// # Slightly more realistic example
///
/// ```
/// # struct Arg {}
/// # impl Arg { fn new(_opt: ArgOptions) -> Self { Self {} } }
/// // in awesome_cli_lib
/// #[non_exhaustive]
/// # #[derive(Default)]
/// struct ArgOptions {
///     pub name: String,
///     pub short: Option<String>,
///     pub long: Option<String>,
///     pub help: Option<String>,
///     pub required: bool,
///     pub takes_value: bool,
///     pub multiple: bool,
///     pub default_value: Option<String>,
/// }
///
/// impl ArgOptions {
///     pub fn new(name: String) -> Self {
///         // ...
/// #       Self { name, ..Default::default() }
///     }
/// }
///
/// // your crate
/// use assign::assign;
///
/// let arg = Arg::new(assign!(ArgOptions::new("version".into()), {
///     short: Some("V".into()),
///     long: Some("version".into()),
///     help: Some("prints the version and quits.".into()),
/// }));
/// ```
#[macro_export]
macro_rules! assign {
    ($initial_value:expr, {
        $( $field:ident $( : $value:expr )? ),+ $(,)?
    }) => ({
        let mut item = $initial_value;
        $( $crate::assign!(@assign item $field $( : $value )?); )+
        item
    });
    (@assign $item:ident $field:ident : $value:expr) => {
        $item.$field = $value;
    };
    (@assign $item:ident $field:ident) => {
        $item.$field = $field;
    };
}

#[cfg(test)]
mod tests {
    #[derive(Debug, Default, PartialEq)]
    struct SomeStruct {
        a: u32,
        b: Option<f32>,
        c: Option<u64>,
    }

    #[test]
    fn basic() {
        let res = assign!(SomeStruct::default(), {
            a: 5,
            b: None,
        });

        assert_eq!(
            res,
            SomeStruct {
                a: 5,
                b: None,
                c: None
            }
        );
    }

    #[test]
    fn shorthand() {
        let def = SomeStruct::default();
        let a = 5;
        let res = assign!(def, { a });

        assert_eq!(
            res,
            SomeStruct {
                a: 5,
                b: None,
                c: None
            }
        );
    }

    #[test]
    fn field_expr_inference() {
        let b = 0.0.into();
        let res = assign!(SomeStruct::default(), {
            b,
            c: 1.into(),
        });

        assert_eq!(
            res,
            SomeStruct {
                a: 0,
                b: Some(0.0),
                c: Some(1)
            }
        );
    }

    #[test]
    fn all_fields() {
        let a = 1;
        let b = Some(1.0);

        let res = assign!(SomeStruct::default(), {
            a,
            b,
            c: 1.into(),
        });

        assert_eq!(
            res,
            SomeStruct {
                a: 1,
                b: Some(1.0),
                c: Some(1),
            }
        );
    }
}
