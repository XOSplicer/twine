//! # Twine
//!
//! A lightweight data structure for efficiently representing the concatenation
//! of temporary values as strings.
//!
//! Use the various `From::from` implementations to crate a new value.
//!
//! Use `Twine::concat` to concatinate multiple Twines.
//!
//! Similar to LLVM's llvm::Twine.
//!
//! A Twine is a lightweight string structure.
//! It represents a concatenated string using a binary-tree.
//! The Twine can be rendered into a string or a buffer when the result of a concatination is used.
//! If the result is never used, the cost of intermediate strings is avoided.
//! The Twine also tracks the type of concatinated primitive values that are not strings.
//! This allows further reduction of temporary strings for theses types.
//! Therefore appending an integer to a string is very cheap.
//!
//! Twines have a special `null` values, which always yields a `null` value on concatination
//! and is rendered an an empty string.
//!
//! ## Features
//!
//! - `std`: enable support for `std::String`. Use `Twine::to_string_preallocating()` to render into a String.
//!

#![no_std]

#[cfg(feature = "std")]
extern crate std;

// TODO: add #[inline(always)]

/// The Twine lightweight data structure for efficiently representing the concatenation
/// of temporary values as strings.
#[derive(Debug, Clone, Copy)]
pub struct Twine<'a>(TwineKind<'a>);

/// Inner representation of a Twine.
///
/// Private, as enum variants of public items would also be public.
#[derive(Debug, Clone, Copy)]
enum TwineKind<'a> {
    Null,
    Empty,
    Unary(TwineChild<'a>),
    Binary(TwineChild<'a>, TwineChild<'a>),
}

/// Inner representation of a non-empty Twine
#[derive(Debug, Clone, Copy)]
enum TwineChild<'a> {
    Twine(&'a Twine<'a>),
    Str(&'a str),
    Char(&'a char),
    DecUsize(&'a usize),
    DecU64(&'a u64),
    DecU32(&'a u32),
    DecU16(&'a u16),
    DecIsize(&'a isize),
    DecI64(&'a i64),
    DecI32(&'a i32),
    DecI16(&'a i16),
    HexU64(&'a u64),
    HexUsize(&'a usize),
    FmtArgs(&'a core::fmt::Arguments<'a>),
}

impl<'a> From<&'a str> for Twine<'a> {
    #[inline(always)]
    fn from(t: &'a str) -> Twine<'a> {
        if t.is_empty() {
            Twine::empty()
        } else {
            Twine(TwineKind::Unary(TwineChild::Str(t)))
        }
    }
}

#[cfg(feature = "std")]
impl<'a> From<&'a std::string::String> for Twine<'a> {
    #[inline(always)]
    fn from(t: &'a std::string::String) -> Twine<'a> {
        Twine::from(t.as_str())
    }
}

impl<'a> From<&'a char> for Twine<'a> {
    #[inline(always)]
    fn from(t: &'a char) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::Char(t)))
    }
}

impl<'a> From<&'a usize> for Twine<'a> {
    #[inline(always)]
    fn from(t: &'a usize) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::DecUsize(t)))
    }
}

impl<'a> From<&'a u64> for Twine<'a> {
    #[inline(always)]
    fn from(t: &'a u64) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::DecU64(t)))
    }
}

impl<'a> From<&'a u32> for Twine<'a> {
    #[inline(always)]
    fn from(t: &'a u32) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::DecU32(t)))
    }
}

impl<'a> From<&'a u16> for Twine<'a> {
    #[inline(always)]
    fn from(t: &'a u16) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::DecU16(t)))
    }
}

impl<'a> From<&'a isize> for Twine<'a> {
    #[inline(always)]
    fn from(t: &'a isize) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::DecIsize(t)))
    }
}

impl<'a> From<&'a i64> for Twine<'a> {
    #[inline(always)]
    fn from(t: &'a i64) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::DecI64(t)))
    }
}

impl<'a> From<&'a i32> for Twine<'a> {
    #[inline(always)]
    fn from(t: &'a i32) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::DecI32(t)))
    }
}

impl<'a> From<&'a i16> for Twine<'a> {
    #[inline(always)]
    fn from(t: &'a i16) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::DecI16(t)))
    }
}

impl<'a> From<&'a core::fmt::Arguments<'a>> for Twine<'a> {
    #[inline(always)]
    fn from(t: &'a core::fmt::Arguments<'a>) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::FmtArgs(t)))
    }
}

impl<'a> From<(&'a str, &'a str)> for Twine<'a> {
    #[inline(always)]
    fn from((lhs, rhs): (&'a str, &'a str)) -> Twine<'a> {
        match (lhs.is_empty(), rhs.is_empty()) {
            (true, true) => Twine::empty(),
            (true, false) => Twine::from(rhs),
            (false, true) => Twine::from(lhs),
            (false, false) => Twine(TwineKind::Binary(
                TwineChild::Str(lhs),
                TwineChild::Str(rhs),
            )),
        }
    }
}

impl<'a> Twine<'a> {
    /// Create a new `null` value Twine.
    ///
    /// The `null` value will allways return a `null` value on concatination.
    ///
    /// # Example
    /// ```
    /// # use twine::Twine;
    /// let a = &Twine::null();
    /// let b = &Twine::from("foo");
    /// let c = a.concat(b);
    /// assert!(c.is_null());
    /// assert_eq!(c.to_string(), "");
    /// ```
    #[inline(always)]
    pub fn null() -> Twine<'a> {
        Twine(TwineKind::Null)
    }

    /// Create a new empty Twine, effecively equivalent to str: "".
    ///
    /// # Example
    /// ```
    /// # use twine::Twine;
    /// let a = &Twine::empty();
    /// assert_eq!(a.to_string(), "");
    /// ```
    #[inline(always)]
    pub fn empty() -> Twine<'a> {
        Twine(TwineKind::Empty)
    }

    /// Create a new Twine that is rendered as the hexadecimal value of the input.
    ///
    /// # Example
    /// ```
    /// # use twine::Twine;
    /// let a = &Twine::hex_u64(&0x42);
    /// assert_eq!(a.to_string(), "42");
    /// ```
    #[inline(always)]
    pub fn hex_u64(t: &'a u64) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::HexU64(t)))
    }

    /// Create a new Twine that is rendered as the hexadecimal value of the input.
    ///
    /// # Example
    /// ```
    /// # use twine::Twine;
    /// let a = &Twine::hex_usize(&0x42);
    /// assert_eq!(a.to_string(), "42");
    /// ```
    #[inline(always)]
    pub fn hex_usize(t: &'a usize) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::HexUsize(t)))
    }

    /// Flatten a nested unary Twine
    #[inline(always)]
    fn flatten(&'a self) -> &'a Twine<'a> {
        match self.0 {
            // TODO: better flattening by moving flatten to TwineChild
            TwineKind::Unary(TwineChild::Twine(t)) => t,
            _ => self,
        }
    }

    /// Create a new Twine by concatinating two Twines.
    #[inline(always)]
    pub fn new_concat(lhs: &'a Twine<'a>, rhs: &'a Twine<'a>) -> Twine<'a> {
        match (lhs.flatten().0, rhs.flatten().0) {
            (TwineKind::Null, _) => Twine(TwineKind::Null),
            (_, TwineKind::Null) => Twine(TwineKind::Null),
            (TwineKind::Empty, _) => *rhs,
            (_, TwineKind::Empty) => *lhs,
            (TwineKind::Unary(l), TwineKind::Unary(r)) => Twine(TwineKind::Binary(l, r)),
            _ => Twine(TwineKind::Binary(
                TwineChild::Twine(lhs),
                TwineChild::Twine(rhs),
            )),
        }
    }

    /// Create a new Twine by concatinating another Twine to this one.
    ///
    /// Concatinating a `null` value always returns a null value
    ///
    /// # Example
    /// ```
    /// # use twine::Twine;
    /// let a = &Twine::from("1234");
    /// let b = &Twine::from(&56u32);
    /// let c = a.concat(b);
    /// assert_eq!(c.to_string(), "123456");
    /// let null = &Twine::null();
    /// let d = a.concat(null);
    /// assert_eq!(d.is_null(), true);
    /// assert_eq!(d.to_string(), "");
    /// ```
    #[inline(always)]
    pub fn concat(&'a self, other: &'a Twine<'a>) -> Twine<'a> {
        Twine::new_concat(self, other)
    }

    /// Checks if the Twine has 0 childs
    ///
    /// # Example
    /// ```
    /// # use twine::Twine;
    /// assert_eq!(Twine::null().is_nullary(), true);
    /// assert_eq!(Twine::empty().is_nullary(), true);
    /// assert_eq!(Twine::from("foo").is_nullary(), false);
    /// ```
    #[inline(always)]
    pub fn is_nullary(&self) -> bool {
        matches!(self.0, TwineKind::Empty | TwineKind::Null)
    }

    /// Checks if the Twine has 1 childs
    ///
    /// # Example
    /// ```
    /// # use twine::Twine;
    /// assert_eq!(Twine::null().is_unary(), false);
    /// assert_eq!(Twine::empty().is_unary(), false);
    /// assert_eq!(Twine::from("foo").is_unary(), true);
    /// assert_eq!(Twine::from(("foo", "bar")).is_unary(), false);
    /// ```
    #[inline(always)]
    pub fn is_unary(&self) -> bool {
        matches!(self.0, TwineKind::Unary(_))
    }

    /// Checks if the Twine has 2 childs
    ///
    /// # Example
    /// ```
    /// # use twine::Twine;
    /// assert_eq!(Twine::null().is_binary(), false);
    /// assert_eq!(Twine::empty().is_binary(), false);
    /// assert_eq!(Twine::from("foo").is_binary(), false);
    /// assert_eq!(Twine::from(("foo", "bar")).is_binary(), true);
    /// ```
    #[inline(always)]
    pub fn is_binary(&self) -> bool {
        matches!(self.0, TwineKind::Binary(_, _))
    }

    /// Checks if the Twine is a null value
    ///
    /// # Example
    /// ```
    /// # use twine::Twine;
    /// assert_eq!(Twine::null().is_null(), true);
    /// assert_eq!(Twine::empty().is_null(), false);
    /// assert_eq!(Twine::from("foo").is_null(), false);
    /// assert_eq!(Twine::from(("foo", "bar")).is_null(), false);
    /// ````
    #[inline(always)]
    pub fn is_null(&self) -> bool {
        matches!(self.0, TwineKind::Null)
    }

    /// Checks if the Twine is a single, possibly empty, str
    ///
    /// # Example
    /// ```
    /// # use twine::Twine;
    /// assert_eq!(Twine::null().is_single_str(), false);
    /// assert_eq!(Twine::empty().is_single_str(), true);
    /// assert_eq!(Twine::from("").is_single_str(), true);
    /// assert_eq!(Twine::from("foo").is_single_str(), true);
    /// assert_eq!(Twine::from(("foo", "bar")).is_single_str(), false);
    /// ```
    #[inline(always)]
    pub fn is_single_str(&self) -> bool {
        self.as_single_str().is_some()
    }

    /// Returns the Twine as a single str if it only contains one str.
    ///
    /// # Example
    /// ```
    /// # use twine::Twine;
    /// assert_eq!(Twine::null().as_single_str(), None);
    /// assert_eq!(Twine::empty().as_single_str(), Some(""));
    /// assert_eq!(Twine::from("").as_single_str(), Some(""));
    /// assert_eq!(Twine::from("foo").as_single_str(), Some("foo"));
    /// assert_eq!(Twine::from(("foo", "bar")).as_single_str(), None);
    /// ```
    #[inline(always)]
    pub fn as_single_str(&self) -> Option<&'a str> {
        match self.0 {
            TwineKind::Empty => Some(""),
            TwineKind::Unary(TwineChild::Str(s)) => Some(s),
            _ => None,
        }
    }

    /// Checks if the Twine is trivially empty because it does not have children.
    /// Even if false, the Twine still might render to an empty string.
    ///
    /// To check if the twine actually renders to an empty string use `is_empty()`
    ///
    /// # Example
    /// ```
    /// # use twine::Twine;
    /// assert_eq!(Twine::null().is_trivially_empty(), true);
    /// assert_eq!(Twine::empty().is_trivially_empty(), true);
    /// assert_eq!(Twine::from("").is_trivially_empty(), true);
    /// assert_eq!(Twine::from("foo").is_trivially_empty(), false);
    /// assert_eq!(Twine::from(("foo", "bar")).is_trivially_empty(), false);
    /// assert_eq!(Twine::from(("", "")).is_trivially_empty(), true);
    /// ```
    #[inline(always)]
    pub fn is_trivially_empty(&self) -> bool {
        self.is_nullary()
    }

    /// Check if the Twine actually renders to an empty string.
    /// This requires actually rendering parts of the twine and might need allocations.
    ///
    /// # Example
    /// ```
    /// # use twine::Twine;
    /// assert_eq!(Twine::null().is_empty(), true);
    /// assert_eq!(Twine::empty().is_empty(), true);
    /// assert_eq!(Twine::from("").is_empty(), true);
    /// assert_eq!(Twine::from("foo").is_empty(), false);
    /// assert_eq!(Twine::from(("foo", "bar")).is_empty(), false);
    /// assert_eq!(Twine::from(("", "")).is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        struct WriteCounter(usize);
        impl core::fmt::Write for WriteCounter {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                self.0 += s.len();
                Ok(())
            }
        }
        let mut w = WriteCounter(0);
        let _ = self.write_to(&mut w);
        w.0 == 0
    }

    /// The estimated capacity needed to store the Twine as a String.
    /// This method returns a vague lower bound needed.
    ///
    /// Use `next_power_of_two()` on the return value to enable efficient allocations
    /// and reduce re-allocations.
    ///
    /// # Example
    /// ```
    /// # use twine::Twine;
    /// assert_eq!(Twine::null().estimated_capacity(), 0);
    /// assert_eq!(Twine::empty().estimated_capacity(), 0);
    /// assert_eq!(Twine::from("").estimated_capacity(), 0);
    /// assert_eq!(Twine::from("foo").estimated_capacity(), 3);
    /// assert_eq!(Twine::from(("foo", "bar")).estimated_capacity(), 6);
    /// assert_eq!(Twine::from(("", "")).estimated_capacity(), 0);
    /// assert!(Twine::from(&42u32).estimated_capacity() >= 1);
    /// ```
    pub fn estimated_capacity(&self) -> usize {
        match self.0 {
            TwineKind::Null => 0,
            TwineKind::Empty => 0,
            TwineKind::Unary(child) => child.estimated_capacity(),
            TwineKind::Binary(l_child, r_child) => {
                l_child.estimated_capacity() + r_child.estimated_capacity()
            }
        }
    }

    /// Render the Twine as a string in the buffer of the writer.
    ///
    /// # Example
    /// ```
    /// # use twine::Twine;
    /// let a = &Twine::from("1234");
    /// let b = &Twine::from(&56u32);
    /// let c = a.concat(b);
    /// let mut s = String::new();
    /// c.write_to(&mut s);
    /// assert_eq!(s, "123456");
    /// ```
    pub fn write_to<W: core::fmt::Write>(&self, w: &mut W) -> core::fmt::Result {
        match self.0 {
            TwineKind::Null => {}
            TwineKind::Empty => {}
            TwineKind::Unary(child) => child.write_to(w)?,
            TwineKind::Binary(l_child, r_child) => {
                l_child.write_to(w)?;
                r_child.write_to(w)?;
            }
        };
        Ok(())
    }

    /// Converts the given Twine to a String
    ///
    /// Specialization of the `to_string()` method that pre-allocates an estimated capacity
    ///
    /// # Example
    /// ```
    /// # use twine::Twine;
    /// let a = &Twine::from("1234");
    /// let b = &Twine::from(&56u32);
    /// let c = a.concat(b);
    /// let s = c.to_string_preallocating();
    /// assert_eq!(s, "123456");
    /// assert!(s.capacity() >= c.estimated_capacity());
    /// assert_eq!(s.capacity(), 8);
    /// ```
    #[cfg(feature = "std")]
    pub fn to_string_preallocating(&self) -> std::string::String {
        let mut s =
            std::string::String::with_capacity(self.estimated_capacity().next_power_of_two());
        // dbg!(s.capacity());
        self.write_to(&mut s).expect("could not format into String");
        // dbg!(s.capacity());
        s
    }
}

impl<'a> core::fmt::Display for Twine<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.write_to(f)
    }
}

impl<'a> core::ops::Add<&'a Twine<'a>> for &'a Twine<'a> {
    type Output = Twine<'a>;

    #[inline(always)]
    fn add(self, rhs: &'a Twine<'a>) -> Self::Output {
        Twine::new_concat(self, rhs)
    }
}

impl<'a> TwineChild<'a> {
    fn estimated_capacity(&self) -> usize {
        match self {
            TwineChild::Twine(t) => t.estimated_capacity(),
            TwineChild::Str(string) => string.len(),
            TwineChild::Char(ch) => ch.len_utf8(),
            TwineChild::DecUsize(_) => 1,
            TwineChild::DecU64(_) => 1,
            TwineChild::DecU32(_) => 1,
            TwineChild::DecU16(_) => 1,
            TwineChild::DecIsize(_) => 1,
            TwineChild::DecI64(_) => 1,
            TwineChild::DecI32(_) => 1,
            TwineChild::DecI16(_) => 1,
            TwineChild::HexU64(_) => 1,
            TwineChild::HexUsize(_) => 1,
            TwineChild::FmtArgs(a) => a.as_str().map(|s| s.len()).unwrap_or(1),
        }
    }

    fn write_to<W: core::fmt::Write>(&self, w: &mut W) -> core::fmt::Result {
        match self {
            TwineChild::Twine(t) => t.write_to(w),
            TwineChild::Str(string) => w.write_str(string),
            TwineChild::Char(ch) => w.write_char(**ch),
            TwineChild::DecUsize(x) => write!(w, "{}", x),
            TwineChild::DecU64(x) => write!(w, "{}", x),
            TwineChild::DecU32(x) => write!(w, "{}", x),
            TwineChild::DecU16(x) => write!(w, "{}", x),
            TwineChild::DecIsize(x) => write!(w, "{}", x),
            TwineChild::DecI64(x) => write!(w, "{}", x),
            TwineChild::DecI32(x) => write!(w, "{}", x),
            TwineChild::DecI16(x) => write!(w, "{}", x),
            TwineChild::HexU64(x) => write!(w, "{:x}", x),
            TwineChild::HexUsize(x) => write!(w, "{:x}", x),
            TwineChild::FmtArgs(f) => w.write_fmt(**f),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn size_of() {
        let size_of_usize = core::mem::size_of::<usize>();
        let size_of_twine = core::mem::size_of::<Twine<'static>>();
        let size_of_str = core::mem::size_of::<&str>();
        assert_eq!(size_of_twine, 6 * size_of_usize);
        assert_eq!(size_of_twine, 3 * &size_of_str);
    }

    #[test]
    fn string_in_bumpalo() {
        let bump = bumpalo::Bump::new();
        let base = bump.alloc_str("bumpalloc-");
        let t = &*bump.alloc(Twine::from(&*base));
        let t1 = t + &*bump.alloc(Twine::hex_u64(&1));
        let mut s1 = bumpalo::collections::String::with_capacity_in(
            t1.estimated_capacity().next_power_of_two(),
            &bump,
        );
        let _ = t1.write_to(&mut s1);
        assert_eq!(s1, "bumpalloc-1");
        assert!(s1.capacity() >= 11);
    }
}
