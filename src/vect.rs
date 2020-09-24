//! Defines the type-safe vector.

use crate::{
    common::*,
    impls,
    size::{Dyn, IntoSize, Size},
};

/// The type-safe vector with type-level length.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Vect<T, S>
where
    S: Size,
{
    pub(crate) data: Vec<T>,
    pub(crate) _phantom: PhantomData<S>,
}

impl<T> Vect<T, U0> {
    /// Creates an empty vector with static length.
    pub fn new() -> Self {
        Self {
            data: vec![],
            _phantom: PhantomData,
        }
    }

    /// Creates an empty vector with static length and with specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            _phantom: PhantomData,
        }
    }

    /// Gets the number of elements.
    pub fn len(&self) -> usize {
        U0::USIZE
    }
}

impl<T> Vect<T, Dyn> {
    /// Creates an empty vector with dynamic length.
    pub fn new() -> Self {
        Self {
            data: vec![],
            _phantom: PhantomData,
        }
    }

    /// Creates a vector from [Vec](Vec).
    pub fn from_vec(data: Vec<T>) -> Self {
        Self {
            data,
            _phantom: PhantomData,
        }
    }

    /// Creates an empty vector with dynamic length and with specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            _phantom: PhantomData,
        }
    }

    /// Gets the number of elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Converts a vector with static length.
    ///
    /// The vector size must be equal to the specified static size.
    /// Otherwise it returns `None`.
    pub fn into_static<S>(self) -> Option<Vect<T, S>>
    where
        S: Unsigned + Size,
    {
        let data = self.data;
        if data.len() == S::USIZE {
            Some(Vect {
                data,
                _phantom: PhantomData,
            })
        } else {
            None
        }
    }
}

impl<T, U, B> Vect<T, UInt<U, B>>
where
    U: Unsigned,
    B: Bit,
{
    /// Gets the number of elements.
    pub fn len(&self) -> usize {
        UInt::<U, B>::USIZE
    }
}

impl<T, S> Vect<T, S>
where
    S: Size,
{
    /// Appends an element to the end of the vector.
    pub fn push(self, elem: T) -> impls::PushImplOp<Self, T>
    where
        (): impls::PushImpl<Self, T>,
    {
        <() as impls::PushImpl<Self, T>>::impl_push(self, elem)
    }

    /// Removes an element from the end of the vector.
    pub fn pop(self) -> impls::PopImplOp<Self>
    where
        (): impls::PopImpl<Self>,
    {
        <() as impls::PopImpl<Self>>::impl_pop(self)
    }

    /// Returns a reference to an element depending on the index.
    ///
    /// If both length and index have static sizes, it returns `&T`. Otherwise, it returns `Option<&T>`.
    pub fn get<'a, I>(&'a self, index: I) -> impls::GetImplOp<'a, Self, I::Output>
    where
        I: IntoSize,
        (): impls::GetImpl<'a, Self, I::Output>,
    {
        <() as impls::GetImpl<'a, Self, I::Output>>::impl_get(self, index.into_size())
    }

    /// Inserts an element at specified index.
    ///
    /// If both length and index have static sizes, it checks if the index is valid in compile time.
    /// Otherwise, it panics if the index is out of bound.
    pub fn insert<I>(self, index: I, elem: T) -> impls::InsertImplOp<Self, I::Output, T>
    where
        I: IntoSize,
        (): impls::InsertImpl<Self, I::Output, T>,
    {
        <() as impls::InsertImpl<Self, I::Output, T>>::impl_insert(self, index.into_size(), elem)
    }

    /// Removes an element at specified index.
    ///
    /// If both length and index have static sizes, it checks if the index is valid in compile time.
    /// Otherwise, it panics if the index is out of bound.
    pub fn remove<I>(self, index: I) -> impls::RemoveImplOp<Self, I::Output>
    where
        I: IntoSize,
        (): impls::RemoveImpl<Self, I::Output>,
    {
        <() as impls::RemoveImpl<Self, I::Output>>::impl_remove(self, index.into_size())
    }

    /// Converts to a vector with dynamic length type.
    pub fn into_dyn(self) -> Vect<T, Dyn> {
        Vect {
            data: self.data,
            _phantom: PhantomData,
        }
    }

    /// Converts to [Vec](Vec).
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }
}
