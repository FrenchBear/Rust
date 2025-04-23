// l49_result_full: Learning Rust
// Complete exploration of result<T, E>
//
// 2025-04-22	PV      First version

#![allow(dead_code, unused)]

use std::num::ParseIntError;

fn main() {
    // Error handling with the `Result` type.

    // [`Result<T, E>`][`Result`] is the type used for returning and propagating errors. It is an enum with the variants,
    // [`Ok(T)`], representing success and containing a value, and [`Err(E)`], representing error and containing an error
    // value.
    // Functions return [`Result`] whenever errors are expected and recoverable. In the `std` crate, [`Result`] is most
    // prominently used for [I/O](../../std/io/index.html).
    //
    // enum Result<T, E> {
    //    Ok(T),
    //    Err(E),
    //
    // A simple function returning [`Result`] might be defined and used like so:

    #[derive(Debug)]
    enum Version {
        Version1,
        Version2,
    }

    fn parse_version(header: &[u8]) -> Result<Version, &'static str> {
        match header.get(0) {
            None => Err("invalid header length"),
            Some(&1) => Ok(Version::Version1),
            Some(&2) => Ok(Version::Version2),
            Some(_) => Err("invalid version"),
        }
    }

    let version = parse_version(&[1, 2, 3, 4]);
    match version {
        Ok(v) => println!("working with version: {v:?}"),
        Err(e) => println!("error parsing header: {e:?}"),
    }

    // Pattern matching on [`Result`]s is clear and straightforward for simple cases, but [`Result`] comes with some
    // convenience methods that make working with it more succinct.

    // The `is_ok` and `is_err` methods do what they say.
    let good_result: Result<i32, i32> = Ok(10);
    let bad_result: Result<i32, i32> = Err(10);
    assert!(good_result.is_ok() && !good_result.is_err());
    assert!(bad_result.is_err() && !bad_result.is_ok());

    // `map` and `map_err` consume the `Result` and produce another.
    let good_result: Result<i32, i32> = good_result.map(|i| i + 1);
    let bad_result: Result<i32, i32> = bad_result.map_err(|i| i - 1);
    assert_eq!(good_result, Ok(11));
    assert_eq!(bad_result, Err(9));

    // Use `and_then` to continue the computation.
    let good_result: Result<bool, i32> = good_result.and_then(|i| Ok(i == 11));
    assert_eq!(good_result, Ok(true));

    // Use `or_else` to handle the error.
    let bad_result: Result<i32, i32> = bad_result.or_else(|i| Ok(i + 20));
    assert_eq!(bad_result, Ok(29));

    // Consume the result and return the contents with `unwrap`.
    let final_awesome_result = good_result.unwrap();
    assert!(final_awesome_result);

    fn multiply_1(first_number_str: &str, second_number_str: &str) -> Result<i32, std::num::ParseIntError> {
        first_number_str
            .parse::<i32>()
            .and_then(|first_number| second_number_str.parse::<i32>().map(|second_number| first_number * second_number))
    }
    assert_eq!(multiply_1("12", "3"), Ok(36));

    fn multiply_2(first_number_str: &str, second_number_str: &str) -> Result<i32, std::num::ParseIntError> {
        Ok(first_number_str.parse::<i32>()? * second_number_str.parse::<i32>()?)
    }
    assert_eq!(multiply_2("12", "3"), Ok(36));

    // # Results must be used
    //
    // A common problem with using return values to indicate errors is that it is easy to ignore the return value, thus
    // failing to handle the error. [`Result`] is annotated with the `#[must_use]` attribute, which will cause the
    // compiler to issue a warning when a Result value is ignored. This makes [`Result`] especially useful with
    // functions that may encounter errors but don't otherwise return a useful value.
    //
    // You might, if you don't want to handle the error, simply assert success with [`expect`]. This will panic if the
    // let mut file = File::create("valuable_data.txt").unwrap();
    // file.write_all(b"important message").expect("failed to write message");
    //
    // You might also simply assert success:
    // let mut file = File::create("valuable_data.txt").unwrap();
    // assert!(file.write_all(b"important message").is_ok());
    //
    // Or propagate the error up the call stack with [`?`]:
    //
    // fn write_message() -> io::Result<()> {
    //     let mut file = File::create("valuable_data.txt")?;
    //     file.write_all(b"important message")?;
    //     Ok(())
    // }

    // # The question mark operator, `?`
    //
    // When writing code that calls many functions that return the [`Result`] type, the error handling can be tedious.
    // The question mark operator, [`?`], hides some of the boilerplate of propagating errors up the call stack.
    //
    // use std::fs::File;
    // use std::io::prelude::*;
    // use std::io;
    //
    // struct Info {
    //     name: String,
    //     age: i32,
    //     rating: i32,
    // }
    //
    // fn write_info(info: &Info) -> io::Result<()> {
    //     let mut file = File::create("my_best_friends.txt")?;
    //     // Early return on error
    //     file.write_all(format!("name: {}\n", info.name).as_bytes())?;
    //     file.write_all(format!("age: {}\n", info.age).as_bytes())?;
    //     file.write_all(format!("rating: {}\n", info.rating).as_bytes())?;
    //     Ok(())
    // }
    //
    // Ending the expression with [`?`] will result in the [`Ok`]'s unwrapped value, unless the result is [`Err`], in
    // which case [`Err`] is returned early from the enclosing function.
    // [`?`] can be used in functions that return [`Result`] because of the early return of [`Err`] that it provides.

    // # Method overview
    // In addition to working with pattern matching, [`Result`] provides a wide variety of different methods.

    // ## Querying the variant
    // The [`is_ok`] and [`is_err`] methods return [`true`] if the [`Result`] is [`Ok`] or [`Err`], respectively.

    let r1: Result<i32, i32> = Ok(10);
    let r2: Result<i32, i32> = Err(0xCAFE);
    assert!(r1.is_ok());
    assert!(r2.is_err());

    // ## Adapters for working with references
    // * [`as_ref`] converts from `&Result<T, E>` to `Result<&T, &E>`
    // * [`as_mut`] converts from `&mut Result<T, E>` to `Result<&mut T, &mut E>`
    // * [`as_deref`] converts from `&Result<T, E>` to `Result<&T::Target, &E>`
    // * [`as_deref_mut`] converts from `&mut Result<T, E>` to `Result<&mut T::Target, &mut E>`

    let r3: &Result<i32, &str> = &Ok(12);
    let o3 = r3.as_ref();
    assert_eq!(o3, Ok(&12));

    let mut r4: &mut Result<(), f64> = &mut Err(1.414);
    let o4 = r4.as_mut();
    assert_eq!(o4, Err(&mut 1.414));

    let r5: Result<String, u32> = Ok("hello".to_string());
    let o5: Result<&str, &u32> = Ok("hello");
    assert_eq!(r5.as_deref(), o5);
    let r6: Result<String, u32> = Err(42);
    let o6: Result<&str, &u32> = Err(&42);
    assert_eq!(r6.as_deref(), o6);

    let mut s7 = "HELLO".to_string();
    let mut r7: Result<String, u32> = Ok("hello".to_string());
    let o7: Result<&mut str, &mut u32> = Ok(&mut s7);
    assert_eq!(
        r7.as_deref_mut().map(|x| {
            x.make_ascii_uppercase();
            x
        }),
        o7
    );
    let mut i7 = 42;
    let mut r8: Result<String, u32> = Err(42);
    let o8: Result<&mut str, &mut u32> = Err(&mut i7);
    assert_eq!(
        r8.as_deref_mut().map(|x| {
            x.make_ascii_uppercase();
            x
        }),
        o8
    );

    // ## Extracting contained values
    //
    // These methods extract the contained value in a [`Result<T, E>`] when it is the [`Ok`] variant.
    // If the [`Result`] is [`Err`]:
    // * [`expect`] panics with a provided custom message
    // * [`unwrap`] panics with a generic message
    // * [`unwrap_or`] returns the provided default value
    // * [`unwrap_or_default`] returns the default value of the type `T` (which must implement the [`Default`] trait)
    // * [`unwrap_or_else`] returns the result of evaluating the provided function
    //
    // The panicking methods [`expect`] and [`unwrap`] require `E` to implement the [`Debug`] trait.

    let r9: Result<i32, &str> = Ok(12);
    let i9 = r9.expect("Not 12???");
    assert_eq!(i9, 12);

    let r10: Result<i32, &str> = Ok(12);
    let i10 = r10.unwrap();
    assert_eq!(i10, 12);

    let r11: Result<i32, &str> = Err("Y'a un os");
    let i11 = r11.unwrap_or(-1);
    assert_eq!(i11, -1);

    let r12: Result<i32, &str> = Err("Y'a un os");
    let i12 = r12.unwrap_or_default();
    assert_eq!(i12, 0);

    let r13: Result<i32, &str> = Err("Y'a un os");
    let i13 = r13.unwrap_or_else(|k| 42);
    assert_eq!(i13, 42);

    // These methods extract the contained value in a [`Result<T, E>`] when it is the [`Err`] variant.
    // They require `T` to implement the [`Debug`] trait. If the [`Result`] is [`Ok`]:
    // * [`expect_err`] panics with a provided custom message
    // * [`unwrap_err`] panics with a generic message

    let r14: Result<i32, &str> = Err("Y'a un os");
    let s14 = r14.expect_err("Normalement y'aurait une erreur");
    assert_eq!(s14, "Y'a un os");

    let r15: Result<i32, &str> = Err("Y'a un os");
    let s15 = r15.unwrap_err();
    assert_eq!(s15, "Y'a un os");

    // ## Transforming contained values
    //
    // These methods transform [`Result`] to [`Option`]:
    // * [`ok`][Result::ok] transforms [`Result<T, E>`] into [`Option<T>`],  mapping [`Ok(v)`] to [`Some(v)`] and [`Err(e)`] to [`None`]
    // * [`err`][Result::err] transforms [`Result<T, E>`] into [`Option<E>`], mapping [`Err(e)`] to [`Some(e)`] and [`Ok(v)`] to [`None`]
    // * [`transpose`] transposes a [`Result`] of an [`Option`] into an [`Option`] of a [`Result`]

    let r16: Result<i32, &str> = Ok(12);
    let o16 = r16.ok();
    assert_eq!(o16, Some(12));

    let r17: Result<i32, &str> = Err("Y'a un os");
    let s17 = r17.err();
    assert_eq!(s17, Some("Y'a un os"));

    let r18: Result<Option<i32>, Option<&str>> = Ok(Some(53));
    let o18 = r18.transpose();
    assert_eq!(o18, Some(Ok(53)));

    // This method transforms the contained value of the [`Ok`] variant:
    // * [`map`] transforms [`Result<T, E>`] into [`Result<U, E>`] by applying the provided function to the contained
    //   value of [`Ok`] and leaving [`Err`] values unchanged

    let r19: Result<i32, &str> = Ok(12);
    let o19 = r19.map(|x| (x * x).to_string());
    assert_eq!(o19, Ok("144".to_string()));

    // This method transforms the contained value of the [`Err`] variant:
    // * [`map_err`] transforms [`Result<T, E>`] into [`Result<T, F>`] by applying the provided function to the
    //   contained value of [`Err`] and leaving [`Ok`] values unchanged
    let r20: Result<i32, &str> = Err("problem description");
    let o20 = r20.map_err(|m| format!("*** Error: {}", m));
    assert_eq!(o20, Err("*** Error: problem description".to_string()));

    // These methods transform a [`Result<T, E>`] into a value of a possibly
    // different type `U`:
    // * [`map_or`] applies the provided function to the contained value of [`Ok`], or returns the provided default
    //   value if the [`Result`] is [`Err`]
    // * [`map_or_else`] applies the provided function to the contained value of [`Ok`], or applies the provided default
    //   fallback function to the contained value of [`Err`]

    let r21: Result<i32, &str> = Ok(12);
    let o21 = r21.map_or(-42, |x| x * 2);
    assert_eq!(o21, 24);

    let r22: Result<i32, char> = Err('?');
    let o22 = r22.map_or_else(|c| c as i32, |x| x * 2);
    assert_eq!(o22, 63);

    // ## Boolean operators
    //
    // These methods treat the [`Result`] as a Boolean value, where [`Ok`] acts like [`true`] and [`Err`] acts like
    // [`false`]. There are two categories of these methods: ones that take a [`Result`] as input, and ones that take a
    // function as input (to be lazily evaluated).
    //
    // The [`and`] and [`or`] methods take another [`Result`] as input, and produce a [`Result`] as output. The [`and`]
    // method can produce a [`Result<U, E>`] value having a different inner type `U` than [`Result<T, E>`]. The [`or`]
    // method can produce a [`Result<T, F>`] value having a different error type `F` than [`Result<T, E>`].
    //
    // | method  | self     | input     | output   |
    // |---------|----------|-----------|----------|
    // | [`and`] | `Err(e)` | (ignored) | `Err(e)` |
    // | [`and`] | `Ok(x)`  | `Err(d)`  | `Err(d)` |
    // | [`and`] | `Ok(x)`  | `Ok(y)`   | `Ok(y)`  |
    // | [`or`]  | `Err(e)` | `Err(d)`  | `Err(d)` |
    // | [`or`]  | `Err(e)` | `Ok(y)`   | `Ok(y)`  |
    // | [`or`]  | `Ok(x)`  | (ignored) | `Ok(x)`  |

    let r22a: Result<i32, char> = Ok(18);
    let r22b: Result<f64, char> = Ok(1.414);
    let o22 = r22a.and(r22b);
    assert_eq!(o22, Ok(1.414));

    let r23a: Result<i32, char> = Err('?');
    let r23b: Result<i32, &str> = Err("Problem");
    let o23 = r23a.or(r23b);
    assert_eq!(o23, Err("Problem"));

    // The [`and_then`] and [`or_else`] methods take a function as input, and only evaluate the function when they need
    // to produce a new value. The [`and_then`] method can produce a [`Result<U, E>`] value having a different inner
    // type `U` than [`Result<T, E>`]. The [`or_else`] method can produce a [`Result<T, F>`] value having a different
    // error type `F` than [`Result<T, E>`].
    //
    // | method       | self     | function input | function result | output   |
    // |--------------|----------|----------------|-----------------|----------|
    // | [`and_then`] | `Err(e)` | (not provided) | (not evaluated) | `Err(e)` |
    // | [`and_then`] | `Ok(x)`  | `x`            | `Err(d)`        | `Err(d)` |
    // | [`and_then`] | `Ok(x)`  | `x`            | `Ok(y)`         | `Ok(y)`  |
    // | [`or_else`]  | `Err(e)` | `e`            | `Err(d)`        | `Err(d)` |
    // | [`or_else`]  | `Err(e)` | `e`            | `Ok(y)`         | `Ok(y)`  |
    // | [`or_else`]  | `Ok(x)`  | (not provided) | (not evaluated) | `Ok(x)`  |

    let r24: Result<i32, char> = Ok(18);
    let o24 = r24.and_then(|x| Ok(2.718));
    assert_eq!(o24, Ok(2.718));

    let r25: Result<i32, char> = Err('Z');
    let o25 = r25.or_else(|x| Err('W'));
    assert_eq!(o25, Err('W'));

    // ## Comparison operators
    //
    // If `T` and `E` both implement [`PartialOrd`] then [`Result<T, E>`] will derive its [`PartialOrd`] implementation.
    // With this order, an [`Ok`] compares as less than any [`Err`], while two [`Ok`] or two [`Err`] compare as their
    // contained values would in `T` or `E` respectively.  If `T` and `E` both also implement [`Ord`], then so does
    // [`Result<T, E>`].

    assert!(Ok(1) < Err(0));

    let r26a: Result<i32, ()> = Ok(0);
    let r26b = Ok(1);
    assert!(r26a < r26b);

    let r27a: Result<(), i32> = Err(0);
    let r27b = Err(1);
    assert!(r27a < r27b);

    // ## Iterating over `Result`
    //
    // A [`Result`] can be iterated over. This can be helpful if you need an iterator that is conditionally empty. The
    // iterator will either produce a single value (when the [`Result`] is [`Ok`]), or produce no values (when the
    // [`Result`] is [`Err`]). For example, [`into_iter`] acts like [`once(v)`] if the [`Result`] is [`Ok(v)`], and like
    // [`empty()`] if the [`Result`] is [`Err`].
    //
    // Iterators over [`Result<T, E>`] come in three types:
    // * [`into_iter`] consumes the [`Result`] and produces the contained value
    // * [`iter`] produces an immutable reference of type `&T` to the contained value
    // * [`iter_mut`] produces a mutable reference of type `&mut T` to the contained value
    //
    // See [Iterating over `Option`] for examples of how this can be useful.

    // You might want to use an iterator chain to do multiple instances of an operation that can fail, but would like to
    // ignore failures while continuing to process the successful results. In this example, we take advantage of the
    // iterable nature of [`Result`] to select only the [`Ok`] values using [`flatten`][Iterator::flatten].

    use std::str::FromStr;
    let mut results = vec![];
    let mut errs = vec![];
    let nums: Vec<_> = ["17", "not a number", "99", "-27", "768"]
        .into_iter()
        .map(u8::from_str)
        // Save clones of the raw `Result` values to inspect
        .inspect(|x| results.push(x.clone()))
        // Challenge: explain how this captures only the `Err` values
        // Ok: my understanding: This inspect process all values, but .err() transform errors in an Option<E>, keeping errors,
        // while Ok results produce None. Then this option is iterable, error values are appended (using .extend) to errs vec,
        // while Null options produced by .err() generate an empty iterable, which does not add anything to errs vec.
        .inspect(|x| errs.extend(x.clone().err()))
        .flatten()
        .collect();
    assert_eq!(errs.len(), 3);
    assert_eq!(nums, [17, 99]);
    println!("results {results:?}");
    println!("errs {errs:?}");
    println!("nums {nums:?}");

    // ## Collecting into `Result`
    //
    // [`Result`] implements the [`FromIterator`][impl-FromIterator] trait, which allows an iterator over [`Result`]
    // values to be collected into a [`Result`] of a collection of each contained value of the original [`Result`]
    // values, or [`Err`] if any of the elements was [`Err`].

    let v = [Ok(2), Ok(4), Err("err!"), Ok(8)];
    let res: Result<Vec<_>, &str> = v.into_iter().collect();
    assert_eq!(res, Err("err!"));

    let v = [Ok(2), Ok(4), Ok(8)];
    let res: Result<Vec<_>, &str> = v.into_iter().collect();
    assert_eq!(res, Ok(vec![2, 4, 8]));

    // [`Result`] also implements the [`Product`][impl-Product] and [`Sum`][impl-Sum] traits, allowing an iterator over
    // [`Result`] values to provide the [`product`][Iterator::product] and [`sum`][Iterator::sum] methods.

    let v = [Err("error!"), Ok(1), Ok(2), Ok(3), Err("foo")];
    let res: Result<i32, &str> = v.into_iter().sum();
    assert_eq!(res, Err("error!"));

    let v = [Ok(1), Ok(2), Ok(21)];
    let res: Result<i32, &str> = v.into_iter().product();
    assert_eq!(res, Ok(42));
}
