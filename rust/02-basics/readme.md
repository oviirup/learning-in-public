# 🦀 Variables and Types

In rust the variables are immutable by default, and have a fixed type. Variables are defined with `let` keyword. To make the variable mutable we need to use the `mut` keyword.

Rust is a statically typed language, which means that the type of a variable must be known at compile time. The compiler can infer the type of a variable based on the value it is assigned, but we can also explicitly specify the type.

### Numbers

Rust has several types of numbers, including integers and floating-point numbers.

- Signed integers: `i8`, `i16`, `i32`, `i64`, `i128`
- Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`
- Floating-point numbers: `f32`, `f64`

Each number can store $2^{n-1}-1$ for signed integers and $2^n$ for unsigned integers, where `n` is the number of bits. The signed integers can store both positive and negative values, while unsigned integers can only store non-negative values.

By default rust uses `i32` for integers and `f64` for floating-point numbers if the type is not specified.

> [!NOTE]
> For large numbers, you can use underscores to improve readability,\
> For example: `let large_number = 1_000_000;`.

### Booleans

You can use the `bool` type to represent a boolean value, which can be either `true` or `false`.

### Characters

The `char` type represents a single Unicode scalar value, which can be a letter, a number, a symbol, or even an emoji.

### Strings

Rust has two main string types:

- `String`: A growable, heap-allocated string.
- `&str`: A string slice, which is a reference to a string.

Both of them have some shared methods, but `String` has additional methods for modifying the string, while `&str` is immutable.

- `length`: Get the length of the string in bytes.
- `is_empty`: Check if the string is empty.
- `contains`: Check if the string contains a substring.
- `replace`: Replace occurrences of a substring with another string.
- `repeat`: Create a new string by repeating the original string a specified number of times.
- `to_uppercase`: Convert the string to uppercase.
- `to_lowercase`: Convert the string to lowercase.
- `trim`: Remove leading and trailing whitespace from the string.
- `push`: Append a single character to a `String`.
- `push_str`: Append a string slice to a `String`.

### Arrays and Slices

An array is a fixed-size collection of elements of the _same type_. A slice is a _dynamically sized_ view into a contiguous sequence of elements.

### Tuples

A tuple is a fixed-size collection of values of _different types_. You can use tuples to group together related values.
Just like JavaScript objects, tuples can be destructured to access their individual elements.
