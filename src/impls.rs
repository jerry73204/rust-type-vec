//! The implementations of vector operations.

use crate::{
    common::*,
    size::{self, Dyn, Size},
    vect::Vect,
};

typ! {
    pub fn Push<ty, size: Size>(Vect::<ty, size>: _) {
        let new_size: Size = size::IncreaseOne(size);
        Vect::<ty, new_size>
    }

    pub fn Pop<ty, size: Size>(Vect::<ty, size>: _) {
        match size::DecreaseOne(size) {
            Option::<Dyn> => Option::<Vect<ty, Dyn>>,
            UTerm => Vect::<ty, UTerm>,
            #[generics(uint: Unsigned, bit: Bit)]
            UInt::<uint, bit> => {
                let new_size = UInt::<uint, bit>;
                Vect::<ty, new_size>
            }
        }
    }
}

// VectFactory

pub trait VectFactory<T> {
    fn from_vec(vec: Vec<T>) -> Self;
}

impl<T, S> VectFactory<T> for Vect<T, S>
where
    S: Size,
{
    fn from_vec(vec: Vec<T>) -> Self {
        Vect {
            data: vec,
            _phantom: PhantomData,
        }
    }
}

// push
pub use push::{PushImpl, PushImplOp};

mod push {
    use super::*;

    /// Implements vector appending.
    pub trait PushImpl<Input, Item> {
        type Output;
        fn impl_push(input: Input, elem: Item) -> Self::Output;
    }

    impl<T, S> PushImpl<Vect<T, S>, T> for ()
    where
        S: Size,
        (): Push<Vect<T, S>>,
        PushOp<Vect<T, S>>: VectFactory<T>,
    {
        type Output = PushOp<Vect<T, S>>;

        fn impl_push(input: Vect<T, S>, item: T) -> Self::Output {
            let mut data = input.data;
            data.push(item);
            PushOp::<Vect<T, S>>::from_vec(data)
        }
    }

    pub type PushImplOp<Input, Item> = <() as PushImpl<Input, Item>>::Output;
}

// pop
pub use pop::{PopImpl, PopImplOp};

mod pop {
    use super::*;

    /// Implements dropping an element at the end of vector.
    pub trait PopImpl<Input> {
        type Output;
        fn impl_pop(input: Input) -> Self::Output;
    }

    impl<Input> PopImpl<Input> for ()
    where
        (): PopPrivate<Input, PopOp<Input>> + Pop<Input>,
    {
        type Output = <() as PopPrivate<Input, PopOp<Input>>>::Output;

        fn impl_pop(input: Input) -> Self::Output {
            <() as PopPrivate<Input, PopOp<Input>>>::impl_pop(input)
        }
    }

    pub type PopImplOp<Input> = <() as PopImpl<Input>>::Output;

    pub trait PopPrivate<Input, Out> {
        type Output;
        fn impl_pop(input: Input) -> Self::Output;
    }

    impl<T, S, Output> PopPrivate<Vect<T, S>, Option<Output>> for ()
    where
        S: Size,
        Output: VectFactory<T>,
    {
        type Output = Option<(Output, T)>;

        fn impl_pop(input: Vect<T, S>) -> Self::Output {
            let mut data = input.data;
            let elem = data.pop()?;
            Some((Output::from_vec(data), elem))
        }
    }

    impl<T, S1, S2> PopPrivate<Vect<T, S1>, Vect<T, S2>> for ()
    where
        S1: Size,
        S2: Size,
        Vect<T, S2>: VectFactory<T>,
    {
        type Output = (Vect<T, S2>, T);

        fn impl_pop(input: Vect<T, S1>) -> Self::Output {
            let mut data = input.data;
            let elem = data.pop().unwrap();
            (Vect::<T, S2>::from_vec(data), elem)
        }
    }
}

// get

pub use get::{GetImpl, GetImplOp};

mod get {
    use super::*;

    /// Implements accessing an vector on vector by type level index.
    pub trait GetImpl<'a, Input, Index> {
        type Output;
        fn impl_get(input: &'a Input, index: Index) -> Self::Output;
    }

    impl<'a, T, S, Index> GetImpl<'a, Vect<T, S>, Index> for ()
    where
        S: Size,
        Index: Size,
        (): GetPrivate<'a, Vect<T, S>, Index, size::CheckIndexOp<S, Index>>
            + size::CheckIndex<S, Index>,
    {
        type Output =
            <() as GetPrivate<'a, Vect<T, S>, Index, size::CheckIndexOp<S, Index>>>::Output;

        fn impl_get(input: &'a Vect<T, S>, index: Index) -> Self::Output {
            <() as GetPrivate<'a, Vect<T, S>, Index, size::CheckIndexOp<S, Index>>>::impl_get(
                input, index,
            )
        }
    }

    pub type GetImplOp<'a, Input, Index> = <() as GetImpl<'a, Input, Index>>::Output;

    pub trait GetPrivate<'a, Input, Index, Out> {
        type Output;
        fn impl_get(input: &'a Input, index: Index) -> Self::Output;
    }

    impl<'a, T, S, Index> GetPrivate<'a, Vect<T, S>, Index, ()> for ()
    where
        T: 'a,
        S: Size,
        Index: Unsigned + Size,
    {
        type Output = &'a T;

        fn impl_get(input: &'a Vect<T, S>, _index: Index) -> Self::Output {
            unsafe { input.data.get_unchecked(Index::USIZE) }
        }
    }

    impl<'a, T, S, Index> GetPrivate<'a, Vect<T, S>, Index, Option<()>> for ()
    where
        T: 'a,
        S: Size,
        Index: Size,
    {
        type Output = Option<&'a T>;

        fn impl_get(input: &'a Vect<T, S>, index: Index) -> Self::Output {
            input.data.get(index.to_usize())
        }
    }
}

// insert

pub use insert::{InsertImpl, InsertImplOp};

mod insert {
    use super::*;

    /// Implements element insertion to a vector.
    pub trait InsertImpl<Input, Index, Item> {
        type Output;
        fn impl_insert(input: Input, index: Index, item: Item) -> Self::Output;
    }
    pub type InsertImplOp<Input, Index, Item> = <() as InsertImpl<Input, Index, Item>>::Output;

    impl<S, Index, Item> InsertImpl<Vect<Item, S>, Index, Item> for ()
    where
        S: Size,
        Index: Size,
        (): size::IncreaseOne<S> + size::CheckIndexInclusive<S, Index>,
    {
        type Output = Vect<Item, size::IncreaseOneOp<S>>;

        fn impl_insert(input: Vect<Item, S>, index: Index, item: Item) -> Self::Output {
            let mut data = input.data;
            data.insert(index.to_usize(), item);
            <Self::Output as VectFactory<Item>>::from_vec(data)
        }
    }
}

// remove

pub use remove::{RemoveImpl, RemoveImplOp};

mod remove {
    use super::*;

    /// Implements element removal from a vector.
    pub trait RemoveImpl<Input, Index> {
        type Output;
        fn impl_remove(input: Input, index: Index) -> Self::Output;
    }
    pub type RemoveImplOp<Input, Index> = <() as RemoveImpl<Input, Index>>::Output;

    impl<S, Index, Item> RemoveImpl<Vect<Item, S>, Index> for ()
    where
        S: Size,
        Index: Size,
        (): size::DecreaseOne<S> + size::CheckIndex<S, Index>,
    {
        type Output = (Vect<Item, size::DecreaseOneOp<S>>, Item);

        fn impl_remove(input: Vect<Item, S>, index: Index) -> Self::Output {
            let mut data = input.data;
            let item = data.remove(index.to_usize());
            (
                <Vect<Item, size::DecreaseOneOp<S>> as VectFactory<Item>>::from_vec(data),
                item,
            )
        }
    }
}
