// l46_option_full: Learning Rust
// Complete exploration of Option<T>
//
// 2025-04-20	PV      First version
// 2025-04-21	PV      Added copied() and cloned()

#![allow(dead_code, unused)]
#![allow(
    clippy::bool_assert_comparison,
    clippy::unnecessary_literal_unwrap,
    clippy::unnecessary_lazy_evaluations,
    clippy::bind_instead_of_map
)]

fn main() {
    // Optional values.
    //
    // Type [`Option`] represents an optional value: every [`Option`] is either [`Some`] and contains a value, or [`None`],
    // and does not. [`Option`] types are very common in Rust code, as they have a number of uses:
    // * Initial values
    // * Return values for functions that are not defined over their entire input range (partial functions)
    // * Return value for otherwise reporting simple errors, where [`None`] is returned on error
    // * Optional struct fields
    // * Struct fields that can be loaned or "taken"
    // * Optional function arguments
    // * Nullable pointers
    // * Swapping things out of difficult situations
    //
    // [`Option`]s are commonly paired with pattern matching to query the presence of a value and take action, always
    // accounting for the [`None`] case.

    // # Options and pointers ("nullable" pointers)
    //
    // Rust's pointer types must always point to a valid location; there are no "null" references. Instead, Rust has
    // *optional* pointers, like the optional owned box, [Option]<Box<T>>.
    //
    // The following example uses [`Option`] to create an optional box of [`i32`]. Notice that in order to use the inner
    // [`i32`] value, the `check_optional` function first needs to use pattern matching to determine whether the box has a
    // value (i.e., it is [`Some`] or not [`None`]).
    //
    // ```
    // let optional = None;
    // check_optional(optional);
    //
    // let optional = Some(Box::new(9000));
    // check_optional(optional);
    //
    // fn check_optional(optional: Option<Box<i32>>) {
    //     match optional {
    //         Some(p) => println!("has value {p}"),
    //         None => println!("has no value"),
    //     }
    // }
    // ```

    // # The question mark operator, `?`
    //
    // Similar to the [`Result`] type, when writing code that calls many functions that return the [`Option`] type,
    // handling `Some`/`None` can be tedious. The question mark operator, [`?`], hides some of the boilerplate of
    // propagating values up the call stack.
    //
    // It replaces this:
    // ```
    // fn add_last_numbers(stack: &mut Vec<i32>) -> Option<i32> {
    //     let a = stack.pop();
    //     let b = stack.pop();
    //
    //     match (a, b) {
    //         (Some(x), Some(y)) => Some(x + y),
    //         _ => None,
    //     }
    // }
    //
    // ```
    //
    // With this:
    // ```
    // fn add_last_numbers(stack: &mut Vec<i32>) -> Option<i32> {
    //     Some(stack.pop()? + stack.pop()?)
    // }
    // ```
    //
    // Ending the expression with [`?`] will result in the [`Some`]'s unwrapped value, unless the result is [`None`], in
    // which case [`None`] is returned early from the enclosing function.
    // [`?`] can be used in functions that return [`Option`] because of the early return of [`None`] that it provides.

    // # Method overview
    //
    // In addition to working with pattern matching, [`Option`] provides a wide variety of different methods.
    //
    // ## Querying the variant
    // The [`is_some`] and [`is_none`] methods return [`true`] if the [`Option`] is [`Some`] or [`None`], respectively.

    let o1: Option<i32> = Some(42);
    assert_eq!(o1.is_some(), true);
    assert_eq!(o1.is_none(), false);

    let o2: Option<i32> = None;
    assert_eq!(o2.is_some(), false);
    assert_eq!(o2.is_none(), true);

    // ## Adapters for working with references
    // * [`as_ref`] converts from <code>[&][][Option]\<T></code> to <code>[Option]<[&]T></code>
    // * [`as_mut`] converts from <code>[&mut] [Option]\<T></code> to <code>[Option]<[&mut] T></code>
    // * [`as_deref`] converts from <code>[&][][Option]\<T></code> to <code>[Option]<[&]T::[Target]></code>
    // * [`as_deref_mut`] converts from <code>[&mut] [Option]\<T></code> to <code>[Option]<[&mut] T::[Target]></code>
    // * [`as_pin_ref`] converts from <code>[Pin]<[&][][Option]\<T>></code> to <code>[Option]<[Pin]<[&]T>></code>
    // * [`as_pin_mut`] converts from <code>[Pin]<[&mut] [Option]\<T>></code> to <code>[Option]<[Pin]<[&mut] T>></code>

    let o3: Option<i32> = Some(42);
    let r3: Option<&i32> = o3.as_ref();
    assert_eq!(r3, Some(&42));

    let mut o4: Option<i32> = Some(33);
    let r4: Option<&mut i32> = o4.as_mut();
    assert_eq!(r4, Some(&mut 33));

    let s5 = String::from("Pierre");
    let o5 = &Some(s5);
    let r5 = o5.as_deref();
    assert_eq!(r5, Some("Pierre"));

    let s6 = String::from("Pierre");
    let mut o6 = Some(s6);
    let r6 = o6.as_deref_mut().map(|x| x.to_ascii_uppercase());
    assert_eq!(r6, Some("PIERRE".into()));

    // ## Extracting the contained value
    //
    // These methods extract the contained value in an [`Option<T>`] when it is the [`Some`] variant. If the [`Option`] is [`None`]:
    // * [`expect`] panics with a provided custom message
    // * [`unwrap`] panics with a generic message
    // * [`unwrap_or`] returns the provided default value
    // * [`unwrap_or_default`] returns the default value of the type `T` (which must implement the [`Default`] trait)
    // * [`unwrap_or_else`] returns the result of evaluating the provided function

    let o7 = Some("Hello");
    let r7 = o7.expect("Problem 7");

    let o8 = Some(1.38756);
    let r8 = o8.unwrap();

    let o9: Option<bool> = None;
    let r9 = o9.unwrap_or(false);

    let o10: Option<u8> = None;
    let r10 = o10.unwrap_or_default();

    let o11: Option<f32> = None;
    let r11 = o11.unwrap_or_else(|| 42.0f32);

    // ## Transforming contained values
    //
    // These methods transform [`Option`] to [`Result`]:
    // * [`ok_or`] transforms [`Some(v)`] to [`Ok(v)`], and [`None`] to [`Err(err)`] using the provided default `err` value
    // * [`ok_or_else`] transforms [`Some(v)`] to [`Ok(v)`], and [`None`] to a value of [`Err`] using the provided function
    // * [`transpose`] transposes an [`Option`] of a [`Result`] into a [`Result`] of an [`Option`]

    let o12 = Some(21);
    let r12 = o12.ok_or("Error 12");

    let o13 = Option::<String>::None;
    let r13 = o13.ok_or_else(|| String::from("Error 13"));

    let o14 = Some(Result::<i32, String>::Ok(12));
    let r14 = o14.transpose();

    // These methods transform the [`Some`] variant:
    // * [`filter`] calls the provided predicate function on the contained value `t` if the [`Option`] is [`Some(t)`], and
    //   returns [`Some(t)`] if the function returns `true`; otherwise, returns [`None`]
    // * [`flatten`] removes one level of nesting from an [`Option<Option<T>>`]
    // * [`map`] transforms [`Option<T>`] to [`Option<U>`] by applying the provided function to the contained value of
    //   [`Some`] and leaving [`None`] values unchanged

    let o15a: Option<i32> = Some(42);
    let r15a = o15a.filter(|x| x & 1 == 0); // Filter using predicate: Value None->None, Some(x): if predicate is true -> Some(x) else None
    assert_eq!(r15a, Some(42));
    let o15b: Option<i32> = Some(43);
    let r15b = o15b.filter(|x| x & 1 == 0); // Filter using predicate: Value None->None, Some(x): if predicate is true -> Some(x) else None
    assert_eq!(r15b, None);

    let o16 = Some(Some(true));
    let r16 = o16.flatten();
    assert_eq!(r16, Some(true));

    let o17 = Some(42);
    let r17 = o17.map(|x| x + 1);
    assert_eq!(r17, Some(43));

    // These methods transform [`Option<T>`] to a value of a possibly different type `U`:
    // * [`map_or`] applies the provided function to the contained value of [`Some`], or returns the provided default value
    //   if the [`Option`] is [`None`]
    // * [`map_or_else`] applies the provided function to the contained value of [`Some`], or returns the result of
    //   evaluating the provided fallback function if the [`Option`] is [`None`]

    let o18: Option<i16> = None;
    let r18 = o18.map_or(17833i16, |x| x + 1);
    assert_eq!(r18, 17833i16);

    let o19: Option<char> = Some('k');
    let r19 = o19.map_or_else(|| '?', |c| c.to_ascii_uppercase());
    assert_eq!(r19, 'K');

    // BEWARE: map: Option<T> -> Option<U>  while  map_or also unwraps: Option<T> -> U !!!
    let k = Some(12);
    let l = k.map(|x| x + 1);
    let m = k.map_or(-1, |x| x + 1);

    // These methods combine the [`Some`] variants of two [`Option`] values:
    // * [`zip`] returns [`Some((s, o))`] if `self` is [`Some(s)`] and the provided [`Option`] value is [`Some(o)`];
    //   otherwise, returns [`None`]
    // * [`zip_with`] calls the provided function `f` and returns [`Some(f(s, o))`] if `self` is [`Some(s)`] and the
    //   provided [`Option`] value is [`Some(o)`]; otherwise, returns [`None`]

    let o20a = Some('m');
    let o20b = Some(true);
    let r20 = o20a.zip(o20b);
    assert_eq!(r20, Some(('m', true)));
    let o20c: Option<bool> = None;
    let r20c = o20a.zip(o20c);
    assert_eq!(r20c, None);

    // zip_with is an unstable feature
    // let o21a = Some("Hello".to_string());
    // let o21b = Some("World".to_string());
    // let r21 = o21a.zip_with(o21b, |s1, s2| s1 + " " + s2.as_str());

    // ## Boolean operators
    //
    // These methods treat the [`Option`] as a boolean value, where [`Some`] acts like [`true`] and [`None`] acts like
    // [`false`]. There are two categories of these methods: ones that take an [`Option`] as input, and ones that take a
    // function as input (to be lazily evaluated).
    //
    // The [`and`], [`or`], and [`xor`] methods take another [`Option`] as input, and produce an [`Option`] as output. Only
    // the [`and`] method can produce an [`Option<U>`] value having a different inner type `U` than [`Option<T>`].
    //
    // | method  | self      | input     | output    |
    // |---------|-----------|-----------|-----------|
    // | [`and`] | `None`    | (ignored) | `None`    |
    // | [`and`] | `Some(x)` | `None`    | `None`    |
    // | [`and`] | `Some(x)` | `Some(y)` | `Some(y)` |
    // | [`or`]  | `None`    | `None`    | `None`    |
    // | [`or`]  | `None`    | `Some(y)` | `Some(y)` |
    // | [`or`]  | `Some(x)` | (ignored) | `Some(x)` |
    // | [`xor`] | `None`    | `None`    | `None`    |
    // | [`xor`] | `None`    | `Some(y)` | `Some(y)` |
    // | [`xor`] | `Some(x)` | `None`    | `Some(x)` |
    // | [`xor`] | `Some(x)` | `Some(y)` | `None`    |

    let o22 = Some('a');
    let o22a: Option<char> = None;
    let r22a = o22.and(o22a);
    assert_eq!(r22a, None);
    let o22b = Some('A');
    let r22b = o22.and(o22b);
    assert_eq!(r22b, Some('A'));

    let o23: Option<char> = None;
    let o23b = Some('z');
    let r23 = o23.or(o23b);
    assert_eq!(r23, Some('z'));

    let o24a1 = Some(2);
    let o24a2: Option<u32> = None;
    assert_eq!(o24a1.xor(o24a2), Some(2));
    let o24b1: Option<u32> = None;
    let o24b2 = Some(2);
    assert_eq!(o24b1.xor(o24b2), Some(2));
    let o24c1 = Some(2);
    let o24c2 = Some(2);
    assert_eq!(o24c1.xor(o24c2), None);
    let o24d1: Option<u32> = None;
    let o24d2: Option<u32> = None;
    assert_eq!(o24d1.xor(o24d2), None);

    // The [`and_then`] and [`or_else`] methods take a function as input, and only evaluate the function when they need to
    // produce a new value. Only the [`and_then`] method can produce an [`Option<U>`] value having a different inner type
    // `U` than [`Option<T>`].
    //
    // | method       | self      | function input | function result | output    |
    // |--------------|-----------|----------------|-----------------|-----------|
    // | [`and_then`] | `None`    | (not provided) | (not evaluated) | `None`    |
    // | [`and_then`] | `Some(x)` | `x`            | `None`          | `None`    |
    // | [`and_then`] | `Some(x)` | `x`            | `Some(y)`       | `Some(y)` |
    // | [`or_else`]  | `None`    | (not provided) | `None`          | `None`    |
    // | [`or_else`]  | `None`    | (not provided) | `Some(y)`       | `Some(y)` |
    // | [`or_else`]  | `Some(x)` | (not provided) | (not evaluated) | `Some(x)` |

    let o25 = Some(String::from("Hello"));
    let r25 = o25
        .map(|s| s.to_uppercase().to_string()) // map fn(T) -> U
        .and_then(|st| st.find('E')) // and_then fn(T) -> Some(U)
        .and_then(|x| Some(x - 1));
    assert_eq!(r25, Some(0));

    let s25 = "Hello";
    let o25 = s25.find("z");
    let def = || Some(s25.len());
    let r25 = o25.or_else(def);
    assert_eq!(r25, Some(s25.len()));

    // ## Comparison operators
    // If `T` implements [`PartialOrd`] then [`Option<T>`] will derive its [`PartialOrd`] implementation.  With this order,
    // [`None`] compares as less than any [`Some`], and two [`Some`] compare the same way as their contained values would
    // in `T`.  If `T` also implements [`Ord`], then so does [`Option<T>`].
    assert!(None < Some(0));
    assert!(Some(0) < Some(1));

    // ## Iterating over `Option`
    // An [`Option`] can be iterated over. This can be helpful if you need an iterator that is conditionally empty. The
    // iterator will either produce a single value (when the [`Option`] is [`Some`]), or produce no values (when the
    // [`Option`] is [`None`]). For example, [`into_iter`] acts like [`once(v)`] if the [`Option`] is [`Some(v)`], and like
    // [`empty()`] if the [`Option`] is [`None`].
    //
    // Iterators over [`Option<T>`] come in three types:
    // * [`into_iter`] consumes the [`Option`] and produces the contained value
    // * [`iter`] produces an immutable reference of type `&T` to the contained value
    // * [`iter_mut`] produces a mutable reference of type `&mut T` to the contained value
    //
    // An iterator over [`Option`] can be useful when chaining iterators, for example, to conditionally insert items. (It's
    // not always necessary to explicitly call an iterator constructor: many [`Iterator`] methods that accept other
    // iterators will also accept iterable types that implement [`IntoIterator`], which includes [`Option`].)
    let y26 = Some(42);
    let n26 = None;
    // chain() already calls into_iter(), so we don't have to do so
    let r26a: Vec<i32> = (0..4).chain(y26).chain(4..8).collect();
    assert_eq!(r26a, [0, 1, 2, 3, 42, 4, 5, 6, 7]);
    let r26b: Vec<i32> = (0..4).chain(n26).chain(4..8).collect();
    assert_eq!(r26b, [0, 1, 2, 3, 4, 5, 6, 7]);

    // ## Collecting into `Option`
    //
    // [`Option`] implements the [`FromIterator`][impl-FromIterator] trait, which allows an iterator over [`Option`] values
    // to be collected into an [`Option`] of a collection of each contained value of the original [`Option`] values, or
    // [`None`] if any of the elements was [`None`].

    let v27a = [Some(2), Some(4), None, Some(8)];
    let r27a: Option<Vec<_>> = v27a.into_iter().collect();
    assert_eq!(r27a, None);
    let v27b = [Some(2), Some(4), Some(8)];
    let r27b: Option<Vec<_>> = v27b.into_iter().collect();
    assert_eq!(r27b, Some(vec![2, 4, 8]));

    // [`Option`] also implements the [`Product`][impl-Product] and [`Sum`][impl-Sum] traits, allowing an iterator over
    // [`Option`] values to provide the [`product`][Iterator::product] and [`sum`][Iterator::sum] methods.

    let v28a = [None, Some(1), Some(2), Some(3)];
    let r28a: Option<i32> = v28a.into_iter().sum();
    assert_eq!(r28a, None);
    let v28b = [Some(1), Some(2), Some(21)];
    let r28b: Option<i32> = v28b.into_iter().product();
    assert_eq!(r28b, Some(42));

    // ## Modifying an [`Option`] in-place
    //
    // These methods return a mutable reference to the contained value of an [`Option<T>`]:
    // * [`insert`] inserts a value, dropping any old contents
    // * [`get_or_insert`] gets the current value, inserting a provided default value if it is [`None`]
    // * [`get_or_insert_default`] gets the current value, inserting the default value of type `T` (which must implement
    //   [`Default`]) if it is [`None`]
    // * [`get_or_insert_with`] gets the current value, inserting a default computed by the provided function if it is
    //   [`None`]

    let mut o29 = Some(13);
    o29.insert(31);
    assert_eq!(o29, Some(31));

    let mut o30a = Some(28);
    let r30a = o30a.get_or_insert(66);
    assert_eq!(r30a, &mut 28);
    let mut o30b: Option<i32> = None;
    let r30b = o30b.get_or_insert(66);
    assert_eq!(r30b, &mut 66);

    let mut o31: Option<u8> = None;
    let r31 = o31.get_or_insert_default();
    assert_eq!(r31, &mut 0u8);

    let mut o32 = Some('j');
    let r32 = o32.get_or_insert_with(|| 'Z');
    assert_eq!(r32, &mut 'j');

    // These methods transfer ownership of the contained value of an [`Option`]:
    //
    // * [`take`] takes ownership of the contained value of an [`Option`], if any, replacing the [`Option`] with [`None`]
    // * [`replace`] takes ownership of the contained value of an [`Option`], if any, replacing the [`Option`] with a
    //   [`Some`] containing the provided value

    let mut o33 = Some(2);
    let r33 = o33.take();
    assert_eq!(o33, None);
    assert_eq!(r33, Some(2));

    let mut o34a = Some(2);
    let r34a = o34a.replace(5);
    assert_eq!(o34a, Some(5));
    assert_eq!(r34a, Some(2));
    let mut o34b = None;
    let r34b = o34b.replace(3);
    assert_eq!(o34b, Some(3));
    assert_eq!(r34b, None);

    // Extra:
    // Option Copied transforms an Option<&T> in a Option<T> where T:Copy by copying content
    // Option Cloned transforms an Option<&T> in a Option<T> where T:Copy by cloning content
    let f34 = 53.98;
    let o34 = Some(&34);
    let r34a = o34.copied();
    let r34b = o34.cloned();
}
