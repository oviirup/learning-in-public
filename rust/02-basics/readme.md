# 🦀 Basics

This section is all about the basics of the Rust programming language. It covers the fundamental concepts and features that you need to know to get started with Rust.

## Variables and Types

In rust the variables are immutable by default, and have a fixed type. Variables are defined with `let` keyword. To make the variable mutable we need to use the `mut` keyword. It is a convention in Rust to use _snake_case_ for variable names.

Rust is a statically typed language, which means that the type of a variable must be known at compile time. The compiler can infer the type of a variable based on the value it is assigned, but we can also explicitly specify the type.

There are two primitive types: _scalar_ (contains integers, float, booleans, etc.) and _compound_ types (arrays, and tuples).

### Numbers

Rust has several types of numbers, including integers and floating-point numbers.

- Signed integers: `i8`, `i16`, `i32`, `i64`, `i128`
- Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`
- Floating-point numbers: `f32`, `f64`

Each number can store $2^{n-1}-1$ for signed integers and $2^n-1$ for unsigned integers, where `n` is the number of bits. The signed integers can store both positive and negative values, while unsigned integers can only store non-negative values.

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

- `len`: Get the length of the string in bytes.
- `is_empty`: Check if the string is empty.
- `contains`: Check if the string contains a substring.
- `replace`: Replace occurrences of a substring with another string.
- `repeat`: Create a new string by repeating the original string a specified number of times.
- `to_uppercase`: Convert the string to uppercase.
- `to_lowercase`: Convert the string to lowercase.
- `trim`: Remove leading and trailing whitespace from the string.
- `push`: Append a single character to a `String`.
- `push_str`: Append a string slice to a `String`.

> [!NOTE]
> `Strings` are UTF-8 encoded, and are stored in the <ins>heap</ins>. The `&str` type is a reference to a string slice, which is stored in the <ins>stack</ins>.

### Arrays and Slices

An array is a fixed-size collection of elements of the _same type_. A slice is a _dynamically sized_ view into a contiguous sequence of elements.

### Tuples

A tuple is a fixed-size collection of values of _different types_. You can use tuples to group together related values.
Just like JavaScript objects, tuples can be destructured to access their individual elements.

## Loops

The loops in Rust are very similar to JavaScript or any other language. Rust has three types of loops:

- `loop`: An infinite loop that can be exited with a `break` statement.
- `while`: A loop that continues as long as a specified condition is true.
- `for`: A loop that iterates over a range or an iterator.

There are some keywords specific to loops: `break` and `continue`. The `break` keyword is used to exit a loop, while the `continue` keyword is used to skip the current iteration and move to the next one.

We can also specify a label for a loop, which allows us to break out of an outer loop from within an inner loop.

## Functions

Functions in Rust are defined using the `fn` keyword, followed by the function name and a list of parameters. The function body is enclosed in curly braces `{}`.

Functions can return a value using the `return` keyword, or by simply omitting the semicolon at the end of the last expression in the function body. We must specify the return type of the function after an arrow `->` in the function signature in case it returns a value.

> [!NOTE]
> In Rust, functions are first-class citizens, which means that they can be assigned to variables, passed as arguments to other functions, and returned from functions.
> Local variables declared within a function (including function parameters) are stored in what is called a <ins>Stack Frame</ins>. When a function is called, a new stack frame is created, and when the function exits, the stack frame is destroyed.

## Structs

It is similar to class in JavaScript, but it is not a class. A Struct is a custom data type that can hold multiple values of different types. It is defined using the `struct` keyword, followed by the struct name and a list of fields. Each field has a name and a type.

We can define a method for a struct using the `impl` keyword. These are functions that are associated with a struct and can access its fields.

> [!NOTE]
> Structs in Rust do not have inheritance like classes in JavaScript, but we can achieve similar functionality using traits and composition.
