use std::{
    cmp::PartialEq,
    convert::From,
    fmt::{Binary, Display, Error, Formatter, LowerHex},
    iter::{IntoIterator, Iterator},
    mem::size_of,
    ops::{Add, AddAssign, Deref, DerefMut},
};

/// SVec is a vector collection type using only stack.
/// # Feature
/// - Faster than Vec
/// - Only for copy type
/// - For small use

/// # Examples
/// ```
/// use util::svec::SVec;
/// let mut svec: SVec<16, i32> = SVec::new();
///
/// svec.push(1);
/// svec.push(2);
/// svec.push(3);
/// svec.push(4);
///
/// assert_eq!(svec[0], 1);
/// assert_eq!(svec[1], 2);
/// assert_eq!(svec.pop(), 4);
/// assert_eq!(svec.capacity(), 16);
/// assert_eq!(svec.len(), 3);
///
/// svec.resize(7);
///
/// assert_eq!(svec.len(), 7);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct SVec<const C: usize, T: Copy + Default> {
    array: [T; C],
    len: usize,
}

impl<const C: usize, T: Copy + Default> SVec<C, T> {
    /// Construct SVec
    pub fn new() -> Self {
        SVec {
            array: [T::default(); C],
            len: 0,
        }
    }

    /// Push value to SVec
    pub fn push(&mut self, value: T) -> &mut SVec<C, T> {
        if self.len() == C {
            panic!("max length")
        } else {
            self.len += 1;
            self.array[self.len - 1] = value;
            self
        }
    }

    /// Push value as little indian
    /// Size must be a multipile of T
    pub fn push_raw<U: Copy>(&mut self, value: U) -> &mut Self {
        let size = size_of::<U>();
        let t_size = size_of::<T>();

        if size % t_size != 0 {
            panic!("invalid type");
        }

        let ptr = &value as *const U as *const T;
        for i in 0..size / size_of::<T>() {
            let v: T = unsafe { *(ptr.add(i)) };
            self.push(v);
        }
        panic!()
    }

    /// Pop value to SVec
    pub fn pop(&mut self) -> T {
        if self.len() == 0 {
            panic!("zero length")
        } else {
            let elm = self[self.len() - 1];
            self.len -= 1;
            elm
        }
    }

    /// Resize SVec
    pub fn resize(&mut self, len: usize) -> &mut SVec<C, T> {
        if C < len {
            panic!("buffer overflowed")
        } else {
            let old_len = self.len();
            self.len = len;
            if old_len < len {
                for i in old_len..len {
                    self[i] = T::default();
                }
            }
            self
        }
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        C
    }

    /// Get length
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Resize buffer
    pub fn resize_buff<const D: usize>(self) -> SVec<D, T> {
        if D < self.len {
            panic!("too small")
        } else {
            let mut new_svec = SVec::new();

            for element in self {
                new_svec.push(element);
            }

            new_svec
        }
    }

    /// convert to Vec<T>
    pub fn as_vec(self) -> Vec<T> {
        let mut vec = Vec::new();

        for i in self {
            vec.push(i);
        }

        vec
    }

    /// Create SVec statically from array
    pub const fn from_raw(arr: [T; C], len: usize) -> SVec<C, T> {
        SVec {
            array: arr,
            len: len,
        }
    }

    pub fn push_to(self, vec: &mut Vec<T>) {
        for v in self {
            vec.push(v);
        }
    }
}

impl<const C: usize> SVec<C, u8> {
    /// Create SVec from u128 value
    pub fn from_value(value: u128, len: usize) -> SVec<C, u8> {
        if C <= 8 && len <= C {
            let mut svec = SVec::new();
            for i in 0..len {
                svec.push(((value >> i * 8) & 0xff) as u8);
            }
            svec
        } else {
            panic!("invalid capacity")
        }
    }
}

impl<const C: usize, T: Copy + Default> Deref for SVec<C, T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.array[0..self.len]
    }
}

impl<const C: usize, T: Copy + Default> DerefMut for SVec<C, T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.array[0..self.len]
    }
}

impl<const C: usize, const D: usize, T: Copy + Default> AddAssign<SVec<D, T>> for SVec<C, T> {
    fn add_assign(&mut self, rhs: SVec<D, T>) {
        if C <= self.len() + rhs.len() {
            panic!("buffer overflow");
        }

        for v in rhs {
            self.push(v);
        }
    }
}

impl<const C: usize, const D: usize, T: Copy + Default> Add<SVec<D, T>> for SVec<C, T> {
    type Output = SVec<C, T>;

    fn add(mut self, rhs: SVec<D, T>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<const C: usize, T: Copy + Default + Display> Display for SVec<C, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "[")?;

        for i in 0..self.len() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", self[i])?;
        }

        write!(f, "]")?;

        Ok(())
    }
}

impl<const C: usize, T: Copy + Default + Binary> Binary for SVec<C, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "[")?;

        for i in 0..self.len() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:b}", self[i])?;
        }

        write!(f, "]")?;

        Ok(())
    }
}

impl<const C: usize, T: Copy + Default + LowerHex> LowerHex for SVec<C, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "[")?;

        for i in 0..self.len() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:x}", self[i])?;
        }

        write!(f, "]")?;

        Ok(())
    }
}

impl<const C: usize, T: Copy + Default> From<&[T]> for SVec<C, T> {
    fn from(value: &[T]) -> SVec<C, T> {
        if C < value.len() {
            panic!("buffer overflowed");
        }

        let mut new_svec = SVec::new();
        for i in 0..value.len() {
            new_svec.push(value[i]);
        }
        new_svec
    }
}

impl<const C: usize, const L: usize, T: Copy + Default> From<[T; L]> for SVec<C, T> {
    fn from(value: [T; L]) -> SVec<C, T> {
        if C < L {
            panic!("buffer overflowed");
        }

        let mut new_svec = SVec::new();
        for i in 0..L {
            new_svec.push(value[i]);
        }
        new_svec
    }
}

impl<const C: usize, T: Copy + Default> IntoIterator for SVec<C, T> {
    type Item = T;
    type IntoIter = SVecIterator<C, T>;

    fn into_iter(self) -> Self::IntoIter {
        SVecIterator::new(self)
    }
}

impl<const C: usize, const D: usize, T: Copy + Default + PartialEq> PartialEq<SVec<D, T>>
    for SVec<C, T>
{
    fn eq(&self, other: &SVec<D, T>) -> bool {
        if self.len() == other.len() {
            for i in 0..self.len() {
                if self[i] != other[i] {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

/// Iterator for SVec
pub struct SVecIterator<const C: usize, T: Copy + Default> {
    svec: SVec<C, T>,
    index: usize,
}

impl<const C: usize, T: Copy + Default> SVecIterator<C, T> {
    pub fn new(svec: SVec<C, T>) -> Self {
        SVecIterator {
            svec: svec,
            index: 0,
        }
    }
}

impl<const C: usize, T: Copy + Default> Iterator for SVecIterator<C, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.index == self.svec.len() {
            None
        } else {
            self.index += 1;
            Some(self.svec[self.index - 1])
        }
    }
}

impl<const C: usize, T: Copy + Default> FromIterator<T> for SVec<C, T> {
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = T>,
    {
        let mut new_svec = SVec::new();
        for i in iter {
            new_svec.push(i);
        }
        new_svec
    }
}
