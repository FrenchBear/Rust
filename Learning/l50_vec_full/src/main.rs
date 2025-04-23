// l50_vec_full: Learning Rust
// Complete exploration of Vec<T>
//
// 2025-04-23	PV      First version

#![allow(dead_code, unused)]

use std::vec;

fn main() {
    // A contiguous growable array type with heap-allocated contents, written `Vec<T>`, short for 'vector'.
    // Vectors have *O*(1) indexing, amortized *O*(1) push (to the end) and *O*(1) pop (from the end).
    // Vectors ensure they never allocate more than `isize::MAX` bytes.

    // # Examples
    // You can explicitly create a [`Vec`] with [`Vec::new`]:
    let v: Vec<i32> = Vec::new();

    // ...or by using the [`vec!`] macro:
    let v: Vec<i32> = vec![];
    let v = vec![1, 2, 3, 4, 5];
    let v = vec![0; 10]; // ten zeroes

    // You can [`push`] values onto the end of a vector (which will grow the vector as needed):
    let mut v = vec![1, 2];
    v.push(3);

    // Popping values works in much the same way:
    let mut v = vec![1, 2];
    let two = v.pop();

    // Vectors also support indexing (through the [`Index`] and [`IndexMut`] traits):
    let mut v = vec![1, 2, 3];
    let three = v[2];
    v[1] = v[1] + 5;

    // # Examples
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);

    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);

    vec[0] = 7;
    assert_eq!(vec[0], 7);

    vec.extend([1, 2, 3]);

    for x in &vec {
        println!("{x}");
    }
    assert_eq!(vec, [7, 1, 2, 3]);

    // The [`vec!`] macro is provided for convenient initialization:
    let mut vec1 = vec![1, 2, 3];
    vec1.push(4);
    let vec2 = Vec::from([1, 2, 3, 4]);
    assert_eq!(vec1, vec2);

    // It can also initialize each element of a `Vec<T>` with a given value.
    // This may be more efficient than performing allocation and initialization
    // in separate steps, especially when initializing a vector of zeros:
    let vec = vec![0; 5];
    assert_eq!(vec, [0, 0, 0, 0, 0]);

    // The following is equivalent, but potentially slower:
    let mut vec = Vec::with_capacity(5);
    vec.resize(5, 0);
    assert_eq!(vec, [0, 0, 0, 0, 0]);

    // Use a `Vec<T>` as an efficient stack:
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        // Prints 3, 2, 1
        println!("{top}");
    }

    // # Indexing
    //
    // The `Vec` type allows access to values by index, because it implements the [`Index`] trait. An example will be more explicit:
    let v = vec![0, 2, 4, 6];
    println!("{}", v[1]); // it will display '2'

    // However be careful: if you try to access an index which isn't in the `Vec`, your software will panic! You cannot do this:
    // let v = vec![0, 2, 4, 6];
    // println!("{}", v[6]); // it will panic!

    // Use [`get`] and [`get_mut`] if you want to check whether the index is in the `Vec`.

    // # Slicing
    //
    // A `Vec` can be mutable. On the other hand, slices are read-only objects.
    // To get a [slice][prim@slice], use [`&`]. Example:

    fn read_slice(slice: &[usize]) {
        // ...
    }

    let v = vec![0, 1];
    read_slice(&v);

    // ... and that's all!
    // you can also do it like this:
    let u: &[usize] = &v;
    // or like this:
    let u: &[_] = &v;

    // In Rust, it's more common to pass slices as arguments rather than vectors when you just want to provide read access.
    // The same goes for [`String`] and [`&str`].

    // # Capacity and reallocation
    //
    // The capacity of a vector is the amount of space allocated for any future elements that will be added onto the vector.
    // This is not to be confused with the *length* of a vector, which specifies the number of actual elements within the
    // vector. If a vector's length exceeds its capacity, its capacity will automatically be increased, but its elements
    // will have to be reallocated.
    //
    // For example, a vector with capacity 10 and length 0 would be an empty vector with space for 10 more elements. Pushing
    // 10 or fewer elements onto the vector will not change its capacity or cause reallocation to occur. However, if the
    // vector's length is increased to 11, it will have to reallocate, which can be slow. For this reason, it is recommended
    // to use [`Vec::with_capacity`] whenever possible to specify how big the vector is expected to get.

    // # Guarantees
    //
    // Due to its incredibly fundamental nature, `Vec` makes a lot of guarantees about its design. This ensures that it's as
    // low-overhead as possible in the general case, and can be correctly manipulated in primitive ways by unsafe code. Note
    // that these guarantees refer to an unqualified `Vec<T>`. If additional type parameters are added (e.g., to support
    // custom allocators), overriding their defaults may change the behavior.
    //
    // Most fundamentally, `Vec` is and always will be a (pointer, capacity, length) triplet. No more, no less. The order of
    // these fields is completely unspecified, and you should use the appropriate methods to modify these. The pointer will
    // never be null, so this type is null-pointer-optimized.
    //
    // However, the pointer might not actually point to allocated memory. In particular, if you construct a `Vec` with
    // capacity 0 via [`Vec::new`], [`vec![]`][`vec!`], [`Vec::with_capacity(0)`][`Vec::with_capacity`], or by calling
    // [`shrink_to_fit`] on an empty Vec, it will not allocate memory. Similarly, if you store zero-sized types inside a
    // `Vec`, it will not allocate space for them. *Note that in this case the `Vec` might not report a [`capacity`] of 0*.
    // `Vec` will allocate if and only if <code>[mem::size_of::\<T>]\() * [capacity]\() > 0</code>. In general, `Vec`'s
    // allocation details are very subtle --- if you intend to allocate memory using a `Vec` and use it for something else
    // (either to pass to unsafe code, or to build your own memory-backed collection), be sure to deallocate this memory by
    // using `from_raw_parts` to recover the `Vec` and then dropping it.

    // If a `Vec` *has* allocated memory, then the memory it points to is on the heap (as defined by the allocator Rust is
    // configured to use by default), and its pointer points to [`len`] initialized, contiguous elements in order (what you
    // would see if you coerced it to a slice), followed by <code>[capacity] - [len]</code> logically uninitialized,
    // contiguous elements.
    //
    // A vector containing the elements `'a'` and `'b'` with capacity 4 can be visualized as below. The top part is the
    // `Vec` struct, it contains a pointer to the head of the allocation in the heap, length and capacity. The bottom part
    // is the allocation on the heap, a contiguous memory block.
    //
    //             ptr      len  capacity
    //        +--------+--------+--------+
    //        | 0x0123 |      2 |      4 |
    //        +--------+--------+--------+
    //             |
    //             v
    // Heap   +--------+--------+--------+--------+
    //        |    'a' |    'b' | uninit | uninit |
    //        +--------+--------+--------+--------+
    //
    // - **uninit** represents memory that is not initialized, see [`MaybeUninit`].
    // - Note: the ABI is not stable and `Vec` makes no guarantees about its memory layout (including the order of fields).
    //
    // `Vec` will never perform a "small optimization" where elements are actually stored on the stack for two reasons:
    // * It would make it more difficult for unsafe code to correctly manipulate a `Vec`. The contents of a `Vec` wouldn't
    //   have a stable address if it were only moved, and it would be more difficult to determine if a `Vec` had actually
    //   allocated memory.
    // * It would penalize the general case, incurring an additional branch on every access.

    // `Vec` will never automatically shrink itself, even if completely empty. This ensures no unnecessary allocations or
    // deallocations occur. Emptying a `Vec` and then filling it back up to the same [`len`] should incur no calls to the
    // allocator. If you wish to free up unused memory, use [`shrink_to_fit`] or [`shrink_to`].

    // [`push`] and [`insert`] will never (re)allocate if the reported capacity is sufficient. [`push`] and [`insert`]
    // *will* (re)allocate if <code>[len] == [capacity]</code>. That is, the reported capacity is completely accurate, and
    // can be relied on. It can even be used to manually free the memory allocated by a `Vec` if desired. Bulk insertion
    // methods *may* reallocate, even when not necessary.

    // `Vec` does not guarantee any particular growth strategy when reallocating when full, nor when [`reserve`] is called.
    // The current strategy is basic and it may prove desirable to use a non-constant growth factor. Whatever strategy is
    // used will of course guarantee *O*(1) amortized [`push`].

    // `vec![x; n]`, `vec![a, b, c, d]`, and [`Vec::with_capacity(n)`][`Vec::with_capacity`], will all produce a `Vec` with
    // at least the requested capacity. If <code>[len] == [capacity]</code>, (as is the case for the [`vec!`] macro), then a
    // `Vec<T>` can be converted to and from a [`Box<[T]>`][owned slice] without reallocating or moving the elements.

    // `Vec` will not specifically overwrite any data that is removed from it, but also won't specifically preserve it. Its
    // uninitialized memory is scratch space that it may use however it wants. It will generally just do whatever is most
    // efficient or otherwise easy to implement. Do not rely on removed data to be erased for security purposes. Even if you
    // drop a `Vec`, its buffer may simply be reused by another allocation. Even if you zero a `Vec`'s memory first, that
    // might not actually happen because the optimizer does not consider this a side-effect that must be preserved. There is
    // one case which we will not break, however: using `unsafe` code to write to the excess capacity, and then increasing
    // the length to match, is always valid.

    // Currently, `Vec` does not guarantee the order in which elements are dropped. The order has changed in the past and
    // may change again.

    //////////////////////////////////////////////////////////////////
    // Inherent methods in impl<T> Vec<T>

    // ===============================================================================================
    // pub const fn new() -> Self

    // Constructs a new, empty `Vec<T>`.
    // The vector will not allocate until elements are pushed onto it.
    let v = Vec::<i32>::new();

    // ===============================================================================================
    // pub fn with_capacity(capacity: usize) -> Self
    //
    // Constructs a new, empty `Vec<T>` with at least the specified capacity.
    // The vector will be able to hold at least `capacity` elements without reallocating. This method is allowed to allocate
    // for more elements than `capacity`. If `capacity` is zero, the vector will not allocate.
    // It is important to note that although the returned vector has the minimum *capacity* specified, the vector will have
    // a zero *length*. For an explanation of the difference between length and capacity, see *[Capacity and reallocation]*.
    // If it is important to know the exact allocated capacity of a `Vec`, always use the [`capacity`] method after
    // construction.
    // For `Vec<T>` where `T` is a zero-sized type, there will be no allocation and the capacity will always be `usize::MAX`.

    let mut vec = Vec::with_capacity(10);

    // The vector contains no items, even though it has capacity for more
    assert_eq!(vec.len(), 0);
    assert!(vec.capacity() >= 10);

    // These are all done without reallocating...
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert!(vec.capacity() >= 10);

    // ...but this may make the vector reallocate
    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);

    // A vector of a zero-sized type will always over-allocate, since no
    // allocation is necessary
    let vec_units = Vec::<()>::with_capacity(10);
    assert_eq!(vec_units.capacity(), usize::MAX);

    // ===============================================================================================
    // Creates a `Vec<T>` directly from a pointer, a length, and a capacity.
    //
    // # Safety
    //
    // This is highly unsafe, due to the number of invariants that aren't
    // checked:
    //
    // * `ptr` must have been allocated using the global allocator, such as via
    //   the [`alloc::alloc`] function.
    // * `T` needs to have the same alignment as what `ptr` was allocated with.
    //   (`T` having a less strict alignment is not sufficient, the alignment really
    //   needs to be equal to satisfy the [`dealloc`] requirement that memory must be
    //   allocated and deallocated with the same layout.)
    // * The size of `T` times the `capacity` (ie. the allocated size in bytes) needs
    //   to be the same size as the pointer was allocated with. (Because similar to
    //   alignment, [`dealloc`] must be called with the same layout `size`.)
    // * `length` needs to be less than or equal to `capacity`.
    // * The first `length` values must be properly initialized values of type `T`.
    // * `capacity` needs to be the capacity that the pointer was allocated with.
    // * The allocated size in bytes must be no larger than `isize::MAX`.
    //   See the safety documentation of [`pointer::offset`].
    //
    // These requirements are always upheld by any `ptr` that has been allocated
    // via `Vec<T>`. Other allocation sources are allowed if the invariants are
    // upheld.
    //
    // Violating these may cause problems like corrupting the allocator's
    // internal data structures. For example it is normally **not** safe
    // to build a `Vec<u8>` from a pointer to a C `char` array with length
    // `size_t`, doing so is only safe if the array was initially allocated by
    // a `Vec` or `String`.
    // It's also not safe to build one from a `Vec<u16>` and its length, because
    // the allocator cares about the alignment, and these two types have different
    // alignments. The buffer was allocated with alignment 2 (for `u16`), but after
    // turning it into a `Vec<u8>` it'll be deallocated with alignment 1. To avoid
    // these issues, it is often preferable to do casting/transmuting using
    // [`slice::from_raw_parts`] instead.
    //
    // The ownership of `ptr` is effectively transferred to the
    // `Vec<T>` which may then deallocate, reallocate or change the
    // contents of memory pointed to by the pointer at will. Ensure
    // that nothing else uses the pointer after calling this
    // function.

    use std::mem;
    use std::ptr;

    let v = vec![1, 2, 3];

    // Prevent running `v`'s destructor so we are in complete control of the allocation.
    let mut v = mem::ManuallyDrop::new(v);

    // Pull out the various important pieces of information about `v`
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    unsafe {
        // Overwrite memory with 4, 5, 6
        for i in 0..len {
            ptr::write(p.add(i), 4 + i);
        }

        // Put everything back together into a Vec
        let rebuilt = Vec::from_raw_parts(p, len, cap);
        assert_eq!(rebuilt, [4, 5, 6]);
    }

    // Using memory that was allocated elsewhere:

    use std::alloc::{Layout, alloc};

    fn main() {
        let layout = Layout::array::<u32>(16).expect("overflow cannot happen");

        let vec = unsafe {
            let mem = alloc(layout).cast::<u32>();
            if mem.is_null() {
                return;
            }

            mem.write(1_000_000);

            Vec::from_raw_parts(mem, 1, 16)
        };

        assert_eq!(vec, &[1_000_000]);
        assert_eq!(vec.capacity(), 16);
    }

    //////////////////////////////////////////////////////////////////
    // impl<T: Clone, A: Allocator> Vec<T, A>

    // ===============================================================================================
    // pub fn resize(&mut self, new_len: usize, value: T)

    // Resizes the `Vec` in-place so that `len` is equal to `new_len`.
    //
    // If `new_len` is greater than `len`, the `Vec` is extended by the difference, with each additional slot filled with
    // `value`. If `new_len` is less than `len`, the `Vec` is simply truncated.
    //
    // This method requires `T` to implement [`Clone`], in order to be able to clone the passed value. If you need more
    // flexibility (or want to rely on [`Default`] instead of [`Clone`]), use [`Vec::resize_with`]. If you only need to
    // resize to a smaller size, use [`Vec::truncate`].

    let mut vec = vec!["hello"];
    vec.resize(3, "world");
    assert_eq!(vec, ["hello", "world", "world"]);

    let mut vec = vec!['a', 'b', 'c', 'd'];
    vec.resize(2, '_');
    assert_eq!(vec, ['a', 'b']);

    // ===============================================================================================
    // pub fn extend_from_slice(&mut self, other: &[T])

    // Clones and appends all elements in a slice to the `Vec`.
    //
    // Iterates over the slice `other`, clones each element, and then appends
    // it to this `Vec`. The `other` slice is traversed in-order.
    //
    // Note that this function is the same as [`extend`],
    // except that it also works with slice elements that are Clone but not Copy.
    // If Rust gets specialization this function may be deprecated.
    //
    // # Examples
    //
    // ```
    // let mut vec = vec![1];
    // vec.extend_from_slice(&[2, 3, 4]);
    // assert_eq!(vec, [1, 2, 3, 4]);

    // ===============================================================================================
    // pub fn extend_from_within<R>(&mut self, src: R) where R: RangeBounds<usize>

    // Given a range `src`, clones a slice of elements in that range and appends it to the end.
    // `src` must be a range that can form a valid subslice of the `Vec`.
    //
    // # Panics
    // Panics if starting index is greater than the end index or if the index is greater than the length of the vector.

    let mut characters = vec!['a', 'b', 'c', 'd', 'e'];
    characters.extend_from_within(2..);
    assert_eq!(characters, ['a', 'b', 'c', 'd', 'e', 'c', 'd', 'e']);

    let mut numbers = vec![0, 1, 2, 3, 4];
    numbers.extend_from_within(..2);
    assert_eq!(numbers, [0, 1, 2, 3, 4, 0, 1]);

    let mut strings = vec![String::from("hello"), String::from("world"), String::from("!")];
    strings.extend_from_within(1..=2);
    assert_eq!(strings, ["hello", "world", "!", "world", "!"]);

    //////////////////////////////////////////////////////////////////
    // impl<T, A: Allocator, const N: usize> Vec<[T; N], A> {

    // ===============================================================================================
    // pub fn into_flattened(self) -> Vec<T, A>

    // Takes a `Vec<[T; N]>` and flattens it into a `Vec<T>`.
    //
    // # Panics
    // Panics if the length of the resulting vector would overflow a `usize`.
    //
    // This is only possible when flattening a vector of arrays of zero-sized types, and thus tends to be irrelevant in
    // practice. If `size_of::<T>() > 0`, this will never panic.

    let mut vec = vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    assert_eq!(vec.pop(), Some([7, 8, 9]));

    let mut flattened = vec.into_flattened();
    assert_eq!(flattened.pop(), Some(6));

    //////////////////////////////////////////////////////////////////
    // impl<T: PartialEq, A: Allocator> Vec<T, A> {

    // pub fn dedup(&mut self)

    // Removes consecutive repeated elements in the vector according to the [`PartialEq`] trait implementation.
    // If the vector is sorted, this removes all duplicates.

    let mut vec = vec![1, 2, 2, 3, 2];
    vec.dedup();
    assert_eq!(vec, [1, 2, 3, 2]);

    //////////////////////////////////////////////////////
    // Common trait implementations for Vec
    //////////////////////////////////////////////////////

    // impl<T, A: Allocator> ops::Deref for Vec<T, A> {
    //     type Target = [T];
    //
    //     #[inline]
    //     fn deref(&self) -> &[T] {
    //         self.as_slice()
    //     }
    // }

    // impl<T, A: Allocator> ops::DerefMut for Vec<T, A> {
    //     #[inline]
    //     fn deref_mut(&mut self) -> &mut [T] {
    //         self.as_mut_slice()
    //     }
    // }

    // impl<T: Clone, A: Allocator + Clone> Clone for Vec<T, A> {
    //     #[track_caller]
    //     fn clone(&self) -> Self {
    //         let alloc = self.allocator().clone();
    //         <[T]>::to_vec_in(&**self, alloc)
    //     }

    // -------------------------------

    // Overwrites the contents of `self` with a clone of the contents of `source`.
    //
    // This method is preferred over simply assigning `source.clone()` to `self`,
    // as it avoids reallocation if possible. Additionally, if the element type
    // `T` overrides `clone_from()`, this will reuse the resources of `self`'s
    // elements as well.
    //
    // # Examples
    //
    // ```
    // let x = vec![5, 6, 7];
    // let mut y = vec![8, 9, 10];
    // let yp: *const i32 = y.as_ptr();
    //
    // y.clone_from(&x);
    //
    // // The value is the same
    // assert_eq!(x, y);
    //
    // // And no reallocation occurred
    // assert_eq!(yp, y.as_ptr());

    // #[track_caller]
    // fn clone_from(&mut self, source: &Self) {
    //     crate::slice::SpecCloneIntoVec::clone_into(source.as_slice(), self);
    // }

    // -------------------------------

    // The hash of a vector is the same as that of the corresponding slice,
    // as required by the `core::borrow::Borrow` implementation.
    //
    use std::hash::BuildHasher;

    let b = std::hash::RandomState::new();
    let v: Vec<u8> = vec![0xa8, 0x3c, 0x09];
    let s: &[u8] = &[0xa8, 0x3c, 0x09];
    assert_eq!(b.hash_one(v), b.hash_one(s));

    // #[stable(feature = "rust1", since = "1.0.0")]
    // impl<T: Hash, A: Allocator> Hash for Vec<T, A> {
    //     #[inline]
    //     fn hash<H: Hasher>(&self, state: &mut H) {
    //         Hash::hash(&**self, state)
    //     }
    // }

    // -------------------------------

    // impl<T, I: SliceIndex<[T]>, A: Allocator> Index<I> for Vec<T, A> {
    //     type Output = I::Output;

    //     #[inline]
    //     fn index(&self, index: I) -> &Self::Output {
    //         Index::index(&**self, index)
    //     }
    // }

    // -------------------------------

    // impl<T, I: SliceIndex<[T]>, A: Allocator> IndexMut<I> for Vec<T, A> {
    //     #[inline]
    //     fn index_mut(&mut self, index: I) -> &mut Self::Output {
    //         IndexMut::index_mut(&mut **self, index)
    //     }
    // }

    // -------------------------------

    // Collects an iterator into a Vec, commonly called via [`Iterator::collect()`]
    //
    // # Allocation behavior
    //
    // In general `Vec` does not guarantee any particular growth or allocation strategy.
    // That also applies to this trait impl.
    //
    // **Note:** This section covers implementation details and is therefore exempt from
    // stability guarantees.
    //
    // Vec may use any or none of the following strategies,
    // depending on the supplied iterator:
    //
    // * preallocate based on [`Iterator::size_hint()`]
    //   * and panic if the number of items is outside the provided lower/upper bounds
    // * use an amortized growth strategy similar to `pushing` one item at a time
    // * perform the iteration in-place on the original allocation backing the iterator
    //
    // The last case warrants some attention. It is an optimization that in many cases reduces peak memory
    // consumption and improves cache locality. But when big, short-lived allocations are created,
    // only a small fraction of their items get collected, no further use is made of the spare capacity
    // and the resulting `Vec` is moved into a longer-lived structure, then this can lead to the large
    // allocations having their lifetimes unnecessarily extended which can result in increased memory
    // footprint.
    //
    // In cases where this is an issue, the excess capacity can be discarded with [`Vec::shrink_to()`],
    // [`Vec::shrink_to_fit()`] or by collecting into [`Box<[T]>`][owned slice] instead, which additionally reduces
    // the size of the long-lived struct.
    //
    // [owned slice]: Box
    //
    use std::sync::Mutex;
    static LONG_LIVED: Mutex<Vec<Vec<u16>>> = Mutex::new(Vec::new());

    for i in 0..10 {
        let big_temporary: Vec<u16> = (0..1024).collect();
        // discard most items
        let mut result: Vec<_> = big_temporary.into_iter().filter(|i| i % 100 == 0).collect();
        // without this a lot of unused capacity might be moved into the global
        result.shrink_to_fit();
        LONG_LIVED.lock().unwrap().push(result);
    }

    // impl<T> FromIterator<T> for Vec<T> {
    //     #[inline]
    //     #[track_caller]
    //     fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vec<T> {
    //         <Self as SpecFromIter<T, I::IntoIter>>::from_iter(iter.into_iter())
    //     }
    // }

    // -------------------------------

    // impl<T, A: Allocator> IntoIterator for Vec<T, A> {
    //     type Item = T;
    //     type IntoIter = IntoIter<T, A>;

    // Creates a consuming iterator, that is, one that moves each value out of
    // the vector (from start to end). The vector cannot be used after calling
    // this.
    let v = vec!["a".to_string(), "b".to_string()];
    let mut v_iter = v.into_iter();

    let first_element: Option<String> = v_iter.next();

    assert_eq!(first_element, Some("a".to_string()));
    assert_eq!(v_iter.next(), Some("b".to_string()));
    assert_eq!(v_iter.next(), None);

    // #[inline]
    // fn into_iter(self) -> Self::IntoIter { }
    // }

    // -------------------------------

    // impl<'a, T, A: Allocator> IntoIterator for &'a Vec<T, A> {
    //     type Item = &'a T;
    //     type IntoIter = slice::Iter<'a, T>;

    //     fn into_iter(self) -> Self::IntoIter {
    //         self.iter()
    //     }
    // }

    // -------------------------------

    // impl<'a, T, A: Allocator> IntoIterator for &'a mut Vec<T, A> {
    //     type Item = &'a mut T;
    //     type IntoIter = slice::IterMut<'a, T>;

    //     fn into_iter(self) -> Self::IntoIter {
    //         self.iter_mut()
    //     }
    // }

    // -------------------------------

    // impl<T, A: Allocator> Extend<T> for Vec<T, A> {
    //     #[inline]
    //     #[track_caller]
    //     fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
    //         <Self as SpecExtend<T, I::IntoIter>>::spec_extend(self, iter.into_iter())
    //     }

    // #[inline]
    // #[track_caller]
    // fn extend_one(&mut self, item: T) {
    //     self.push(item);
    // }

    // #[inline]
    // #[track_caller]
    // fn extend_reserve(&mut self, additional: usize) {
    //     self.reserve(additional);
    // }

    //     #[inline]
    //     unsafe fn extend_one_unchecked(&mut self, item: T) {
    //         // SAFETY: Our preconditions ensure the space has been reserved, and `extend_reserve` is implemented correctly.
    //         unsafe {
    //             let len = self.len();
    //             ptr::write(self.as_mut_ptr().add(len), item);
    //             self.set_len(len + 1);
    //         }
    //     }
    // }

    // -------------------------------

    // Creates a splicing iterator that replaces the specified range in the vector
    // with the given `replace_with` iterator and yields the removed items.
    // `replace_with` does not need to be the same length as `range`.
    //
    // `range` is removed even if the `Splice` iterator is not consumed before it is dropped.
    //
    // It is unspecified how many elements are removed from the vector
    // if the `Splice` value is leaked.
    //
    // The input iterator `replace_with` is only consumed when the `Splice` value is dropped.
    //
    // This is optimal if:
    //
    // * The tail (elements in the vector after `range`) is empty,
    // * or `replace_with` yields fewer or equal elements than `range`’s length
    // * or the lower bound of its `size_hint()` is exact.
    //
    // Otherwise, a temporary vector is allocated and the tail is moved twice.
    //
    // # Panics
    //
    // Panics if the starting point is greater than the end point or if
    // the end point is greater than the length of the vector.

    let mut v = vec![1, 2, 3, 4];
    let new = [7, 8, 9];
    let u: Vec<_> = v.splice(1..3, new).collect();
    assert_eq!(v, [1, 7, 8, 9, 4]);
    assert_eq!(u, [2, 3]);

    // Using `splice` to insert new items into a vector efficiently at a specific position
    // indicated by an empty range:

    let mut v = vec![1, 5];
    let new = [2, 3, 4];
    v.splice(1..1, new);
    assert_eq!(v, [1, 2, 3, 4, 5]);

    // #[cfg(not(no_global_oom_handling))]
    // #[inline]
    // #[stable(feature = "vec_splice", since = "1.21.0")]
    // pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, A>
    // where
    //     R: RangeBounds<usize>,
    //     I: IntoIterator<Item = T>,
    // {
    //     Splice {
    //         drain: self.drain(range),
    //         replace_with: replace_with.into_iter(),
    //     }
    // }

    // -------------------------------

    // Implements comparison of vectors, [lexicographically](Ord#lexicographical-comparison).
    // impl<T, A1, A2> PartialOrd<Vec<T, A2>> for Vec<T, A1>
    // where
    //     T: PartialOrd,
    //     A1: Allocator,
    //     A2: Allocator,
    // {
    //     #[inline]
    //     fn partial_cmp(&self, other: &Vec<T, A2>) -> Option<Ordering> {
    //         PartialOrd::partial_cmp(&**self, &**other)
    //     }
    // }

    // impl<T: Eq, A: Allocator> Eq for Vec<T, A> {}

    // // Implements ordering of vectors, [lexicographically](Ord#lexicographical-comparison).
    // #[stable(feature = "rust1", since = "1.0.0")]
    // impl<T: Ord, A: Allocator> Ord for Vec<T, A> {
    //     #[inline]
    //     fn cmp(&self, other: &Self) -> Ordering {
    //         Ord::cmp(&**self, &**other)
    //     }
    // }

    // #[stable(feature = "rust1", since = "1.0.0")]
    // unsafe impl<#[may_dangle] T, A: Allocator> Drop for Vec<T, A> {
    //     fn drop(&mut self) {
    //         unsafe {
    //             // use drop for [T]
    //             // use a raw slice to refer to the elements of the vector as weakest necessary type;
    //             // could avoid questions of validity in certain cases
    //             ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.as_mut_ptr(), self.len))
    //         }
    //         // RawVec handles deallocation
    //     }
    // }

    // #[stable(feature = "rust1", since = "1.0.0")]
    // impl<T> Default for Vec<T> {
    //     // Creates an empty `Vec<T>`.
    //     //
    //     // The vector will not allocate until elements are pushed onto it.
    //     fn default() -> Vec<T> {
    //         Vec::new()
    //     }
    // }

    // #[stable(feature = "rust1", since = "1.0.0")]
    // impl<T: fmt::Debug, A: Allocator> fmt::Debug for Vec<T, A> {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         fmt::Debug::fmt(&**self, f)
    //     }
    // }

    // #[stable(feature = "rust1", since = "1.0.0")]
    // impl<T, A: Allocator> AsRef<Vec<T, A>> for Vec<T, A> {
    //     fn as_ref(&self) -> &Vec<T, A> {
    //         self
    //     }
    // }

    // #[stable(feature = "vec_as_mut", since = "1.5.0")]
    // impl<T, A: Allocator> AsMut<Vec<T, A>> for Vec<T, A> {
    //     fn as_mut(&mut self) -> &mut Vec<T, A> {
    //         self
    //     }
    // }

    // #[stable(feature = "rust1", since = "1.0.0")]
    // impl<T, A: Allocator> AsRef<[T]> for Vec<T, A> {
    //     fn as_ref(&self) -> &[T] {
    //         self
    //     }
    // }

    // #[stable(feature = "vec_as_mut", since = "1.5.0")]
    // impl<T, A: Allocator> AsMut<[T]> for Vec<T, A> {
    //     fn as_mut(&mut self) -> &mut [T] {
    //         self
    //     }
    // }

    // #[cfg(not(no_global_oom_handling))]
    // #[stable(feature = "rust1", since = "1.0.0")]
    // impl<T: Clone> From<&[T]> for Vec<T> {
    // Allocates a `Vec<T>` and fills it by cloning `s`'s items.

    assert_eq!(Vec::from(&[1, 2, 3][..]), vec![1, 2, 3]);

    //     #[cfg(not(test))]
    //     #[track_caller]
    //     fn from(s: &[T]) -> Vec<T> {
    //         s.to_vec()
    //     }
    //     #[cfg(test)]
    //     fn from(s: &[T]) -> Vec<T> {
    //         crate::slice::to_vec(s, Global)
    //     }
    // }

    // impl<T: Clone> From<&mut [T]> for Vec<T> {
    // Allocates a `Vec<T>` and fills it by cloning `s`'s items.
    assert_eq!(Vec::from(&mut [1, 2, 3][..]), vec![1, 2, 3]);

    //      #[cfg(not(test))]
    //     #[track_caller]
    //     fn from(s: &mut [T]) -> Vec<T> {
    //         s.to_vec()
    //     }
    //     #[cfg(test)]
    //     fn from(s: &mut [T]) -> Vec<T> {
    //         crate::slice::to_vec(s, Global)
    //     }
    // }

    // impl<T: Clone, const N: usize> From<&[T; N]> for Vec<T> {
    // Allocates a `Vec<T>` and fills it by cloning `s`'s items.
    assert_eq!(Vec::from(&[1, 2, 3]), vec![1, 2, 3]);

    //     #[track_caller]
    //     fn from(s: &[T; N]) -> Vec<T> {
    //         Self::from(s.as_slice())
    //     }
    // }

    // impl<T: Clone, const N: usize> From<&mut [T; N]> for Vec<T> {
    // Allocates a `Vec<T>` and fills it by cloning `s`'s items.

    assert_eq!(Vec::from(&mut [1, 2, 3]), vec![1, 2, 3]);

    //     #[track_caller]
    //     fn from(s: &mut [T; N]) -> Vec<T> {
    //         Self::from(s.as_mut_slice())
    //     }
    // }

    // impl<T, const N: usize> From<[T; N]> for Vec<T> {
    // Allocates a `Vec<T>` and moves `s`'s items into it.
    assert_eq!(Vec::from([1, 2, 3]), vec![1, 2, 3]);

    //     #[cfg(not(test))]
    //     #[track_caller]
    //     fn from(s: [T; N]) -> Vec<T> {
    //         <[T]>::into_vec(Box::new(s))
    //     }

    //     #[cfg(test)]
    //     fn from(s: [T; N]) -> Vec<T> {
    //         crate::slice::into_vec(Box::new(s))
    //     }
    // }

    // impl<'a, T> From<Cow<'a, [T]>> for Vec<T>
    // where
    //     [T]: ToOwned<Owned = Vec<T>>,
    // {
    // Converts a clone-on-write slice into a vector.
    //
    // If `s` already owns a `Vec<T>`, it will be returned directly.
    // If `s` is borrowing a slice, a new `Vec<T>` will be allocated and
    // filled by cloning `s`'s items into it.

    use std::borrow::Cow;
    let o: Cow<'_, [i32]> = Cow::Owned(vec![1, 2, 3]);
    let b: Cow<'_, [i32]> = Cow::Borrowed(&[1, 2, 3]);
    assert_eq!(Vec::from(o), Vec::from(b));

    //     #[track_caller]
    //     fn from(s: Cow<'a, [T]>) -> Vec<T> {
    //         s.into_owned()
    //     }
    // }

    // impl<T, A: Allocator> From<Box<[T], A>> for Vec<T, A> {
    // Converts a boxed slice into a vector by transferring ownership of
    // the existing heap allocation.

    let b: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
    assert_eq!(Vec::from(b), vec![1, 2, 3]);

    //     fn from(s: Box<[T], A>) -> Self {
    //         s.into_vec()
    //     }
    // }

    // impl<T, A: Allocator> From<Vec<T, A>> for Box<[T], A> {
    // Converts a vector into a boxed slice.
    //
    // Before doing the conversion, this method discards excess capacity like [`Vec::shrink_to_fit`].

    assert_eq!(Box::from(vec![1, 2, 3]), vec![1, 2, 3].into_boxed_slice());

    // Any excess capacity is removed:
    let mut vec = Vec::with_capacity(10);
    vec.extend([1, 2, 3]);
    assert_eq!(Box::from(vec), vec![1, 2, 3].into_boxed_slice());

    //     #[track_caller]
    //     fn from(v: Vec<T, A>) -> Self {
    //         v.into_boxed_slice()
    //     }
    // }

    // -------------------------------

    // impl From<&str> for Vec<u8> {
    // Allocates a `Vec<u8>` and fills it with a UTF-8 string.

    assert_eq!(Vec::from("123"), vec![b'1', b'2', b'3']);

    //     #[track_caller]
    //     fn from(s: &str) -> Vec<u8> {
    //         From::from(s.as_bytes())
    //     }
    // }

    // -------------------------------

    // impl<T, A: Allocator, const N: usize> TryFrom<Vec<T, A>> for [T; N] {
    //     type Error = Vec<T, A>;

    // Gets the entire contents of the `Vec<T>` as an array,
    // if its size exactly matches that of the requested array.

    assert_eq!(vec![1, 2, 3].try_into(), Ok([1, 2, 3]));
    assert_eq!(<Vec<i32>>::new().try_into(), Ok([]));

    // If the length doesn't match, the input comes back in `Err`:

    let r: Result<[i32; 4], _> = (0..10).collect::<Vec<_>>().try_into();
    assert_eq!(r, Err(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));

    // If you're fine with just getting a prefix of the `Vec<T>`,
    // you can call [`.truncate(N)`](Vec::truncate) first.

    let mut v = String::from("hello world").into_bytes();
    v.sort();
    v.truncate(2);
    let [a, b]: [_; 2] = v.try_into().unwrap();
    assert_eq!(a, b' ');
    assert_eq!(b, b'd');

    //     fn try_from(mut vec: Vec<T, A>) -> Result<[T; N], Vec<T, A>> {
    //         if vec.len() != N {
    //             return Err(vec);
    //         }

    //         // SAFETY: `.set_len(0)` is always sound.
    //         unsafe { vec.set_len(0) };

    //         // SAFETY: A `Vec`'s pointer is always aligned properly, and
    //         // the alignment the array needs is the same as the items.
    //         // We checked earlier that we have sufficient items.
    //         // The items will not double-drop as the `set_len`
    //         // tells the `Vec` not to also drop them.
    //         let array = unsafe { ptr::read(vec.as_ptr() as *const [T; N]) };
    //         Ok(array)
    //     }
    // }
}
