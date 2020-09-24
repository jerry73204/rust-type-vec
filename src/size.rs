use crate::common::*;

// IntoSize

pub trait IntoSize
where
    Self::Output: Size,
{
    type Output;
    fn into_size(self) -> Self::Output;
}

impl IntoSize for usize {
    type Output = Dyn;

    fn into_size(self) -> Self::Output {
        Dyn(self)
    }
}

impl IntoSize for Dyn {
    type Output = Self;

    fn into_size(self) -> Self::Output {
        self
    }
}

impl IntoSize for UTerm {
    type Output = Self;

    fn into_size(self) -> Self::Output {
        self
    }
}

impl<U, B> IntoSize for UInt<U, B>
where
    U: Unsigned,
    B: Bit,
{
    type Output = Self;

    fn into_size(self) -> Self::Output {
        self
    }
}

// Size

pub trait Size {
    fn to_usize(&self) -> usize;
}

impl Size for Dyn {
    fn to_usize(&self) -> usize {
        self.0
    }
}

impl Size for UTerm {
    fn to_usize(&self) -> usize {
        Self::USIZE
    }
}

impl<U, B> Size for UInt<U, B>
where
    U: Unsigned,
    B: Bit,
{
    fn to_usize(&self) -> usize {
        Self::USIZE
    }
}

// Dyn

pub struct Dyn(usize);

// ops

typ! {
    pub fn IsDyn<size>(size: Size) -> Bit {
        match size {
            Dyn => true,
            UTerm => false,
            #[generics(uint: Unsigned, bit: Bit)]
            UInt::<uint, bit> => false,
        }
    }

    pub fn IncreaseOne<size>(size: Size) -> Size {
        match size {
            Dyn => Dyn,
            UTerm => U1,
            #[generics(uint: Unsigned, bit: Bit)]
            UInt::<uint, bit> => UInt::<uint, bit> + 1u,
        }
    }

    pub fn DecreaseOne<size>(size: Size) -> Size {
        match size {
            Dyn => Option::<Dyn>,
            #[generics(uint: Unsigned, bit: Bit)]
            UInt::<uint, bit> => UInt::<uint, bit> - 1u
        }
    }

    pub fn SizeAdd<lhs, rhs>(lhs: Size, rhs: Size) -> Size {
        if IsDyn(lhs) || IsDyn(rhs) {
            Dyn
        } else {
            lhs + rhs
        }
    }

    pub fn CheckIndex<length, index>(length: Size, index: Size) {
        if IsDyn(length) || IsDyn(index) {
            Option::<()>
        } else {
            match index >= 0u && index < length {
                B1 => (),
            }
        }
    }

    pub fn CheckIndexInclusive<length, index>(length: Size, index: Size) {
        if IsDyn(length) || IsDyn(index) {
            Option::<()>
        } else {
            match index >= 0u && index <= length {
                B1 => (),
            }
        }
    }
}
