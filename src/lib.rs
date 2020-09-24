//! The crate demonstrates a type-safe vector with type-level length.
//!
//! ## The type-safe vector type
//! The vector type has a type-level length, which length can be either dynamic or static.
//! That is, the length can be given in compile time or is only known in runtime.
//!
//! ```rust
//! use type_vec::{Vect, Dyn};
//! use typenum::consts::*; // imports U0
//!
//! // vector with static length
//! let vec = Vect::<usize, U0>::new();
//!
//! // vector with dynamic length
//! let vec = Vect::<usize, Dyn>::new();
//! ```
//!
//! ## Type-inferred and type-checked vector operations
//! The vector supports common operations, such as `push`, `pop` and `insert`.
//! The output length is inferred in compile time on each vector operation.
//! Thus, you can gain the benefit of type safety to avoid common errors.
//!
//! ```rust
//! use type_vec::Vect;
//! use typenum::consts::*;
//!
//! let vec = Vect::<usize, U0>::new();
//! let vec: Vect<usize, U1> = vec.push(3);
//! let vec: Vect<usize, U2> = vec.push(1);
//!
//! // This line does not compile due to incorrect output length assumption.
//! /* let vec: Vect<usize, U2> = vec.push(1); */
//!
//! // You can omit the type annotation and leave it to compiler
//! let (vec, item) = vec.pop();
//! let (vec, item) = vec.pop();
//!
//! // This line causes compile error because we cannot pop an empty vector.
//! /* let vec = vec.pop(); */
//! ```
//!
//! ## Zero-abstraction and efficient implementation
//! The design promises zero-abstraction. Whenever the length is known in compile time,
//! it picks the more efficient implementation.
//! Let's see the element accessing using [get](Vect::get). If the index is static,
//! The index is checked against the length in compile time, and returns the element directly
//! if it compiles. Otherwise, it returns an `Option<&T>` like usual [Vec].
//!
//! ```rust
//! use type_vec::Vect;
//! use typenum::consts::*;
//!
//! let vec = Vect::<usize, U0>::new();
//! let vec: Vect<usize, U1> = vec.push(3);
//! let vec: Vect<usize, U2> = vec.push(1);
//! let vec: Vect<usize, U3> = vec.push(4);
//!
//! // get element by static index
//! // the index is checked in compile time and returns the element directly
//! let elem = vec.get(U1::new());
//! assert_eq!(elem, &1);
//!
//! // get element by dynamic index
//! // it returns an `Option` depending on the index
//! let elem = vec.get(1);
//! assert_eq!(elem, Some(&1));
//! ```
//!
//! ## How it works
//! The construction of the vector type heavily relies on the
//! [TYP type-level programming langauge](https://github.com/jerry73204/typ).
//! It enables complex type-level computation done by simple Rusty syntax.
//! Those interested can read the [TYP book](https://github.com/jerry73204/typ-book/).

pub(crate) mod common;
pub mod impls;
pub mod size;
pub mod vect;

pub use size::{Dyn, Size};
pub use vect::Vect;
