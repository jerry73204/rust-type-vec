use crate::{
    common::*,
    impls,
    size::{Dyn, IntoSize, Size},
};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Vect<T, S>
where
    S: Size,
{
    pub(crate) data: Vec<T>,
    pub(crate) _phantom: PhantomData<S>,
}

impl<T> Vect<T, U0> {
    pub fn new() -> Vect<T, U0> {
        Self {
            data: vec![],
            _phantom: PhantomData,
        }
    }

    pub fn with_capacity(capacity: usize) -> Vect<T, U0> {
        Self {
            data: Vec::with_capacity(capacity),
            _phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        U0::USIZE
    }
}

impl<T> Vect<T, Dyn> {
    pub fn len(&self) -> usize {
        self.data.len()
    }

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

    pub fn from_vec(data: Vec<T>) -> Self {
        Self {
            data,
            _phantom: PhantomData,
        }
    }
}

impl<T, U, B> Vect<T, UInt<U, B>>
where
    U: Unsigned,
    B: Bit,
{
    pub fn len(&self) -> usize {
        UInt::<U, B>::USIZE
    }
}

impl<T, S> Vect<T, S>
where
    S: Size,
{
    pub fn push(self, elem: T) -> impls::PushImplOp<Self, T>
    where
        (): impls::PushImpl<Self, T>,
    {
        <() as impls::PushImpl<Self, T>>::impl_push(self, elem)
    }

    pub fn pop(self) -> impls::PopImplOp<Self>
    where
        (): impls::PopImpl<Self>,
    {
        <() as impls::PopImpl<Self>>::impl_pop(self)
    }

    pub fn get<'a, I>(&'a self, index: I) -> impls::GetImplOp<'a, Self, I::Output>
    where
        I: IntoSize,
        (): impls::GetImpl<'a, Self, I::Output>,
    {
        <() as impls::GetImpl<'a, Self, I::Output>>::impl_get(self, index.into_size())
    }

    pub fn insert<I>(self, index: I, elem: T) -> impls::InsertImplOp<Self, I::Output, T>
    where
        I: IntoSize,
        (): impls::InsertImpl<Self, I::Output, T>,
    {
        <() as impls::InsertImpl<Self, I::Output, T>>::impl_insert(self, index.into_size(), elem)
    }

    pub fn remove<I>(self, index: I) -> impls::RemoveImplOp<Self, I::Output>
    where
        I: IntoSize,
        (): impls::RemoveImpl<Self, I::Output>,
    {
        <() as impls::RemoveImpl<Self, I::Output>>::impl_remove(self, index.into_size())
    }

    pub fn into_dyn(self) -> Vect<T, Dyn> {
        Vect {
            data: self.data,
            _phantom: PhantomData,
        }
    }

    pub fn into_vec(self) -> Vec<T> {
        self.data
    }
}
