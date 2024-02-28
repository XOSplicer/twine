#[derive(Debug, Clone, Copy)]
pub struct Twine<'a> (TwineKind<'a>);

#[derive(Debug, Clone, Copy)]
enum TwineKind<'a> {
    Null,
    Empty,
    Unary(TwineChild<'a>),
    Binary(TwineChild<'a>, TwineChild<'a>),
}

#[derive(Debug, Clone, Copy)]
enum TwineChild<'a> {
    Twine(&'a Twine<'a>),
    Str(&'a str),
    Char(&'a char),
    DecU64(&'a u64),
    DecU32(&'a u32),
    DecU16(&'a u16),
    DecI64(&'a i64),
    DecI32(&'a i32),
    DecI16(&'a i16),
    HexU64(&'a u64),
}

impl<'a> From<&'a str> for Twine<'a> {
    fn from(t: &'a str) -> Twine<'a> {
        if t.is_empty() {
            Twine::new_empty()
        } else {
            Twine(TwineKind::Unary(TwineChild::Str(t)))
        }
    }
}

impl<'a> From<&'a String> for Twine<'a> {
    fn from(t: &'a String) -> Twine<'a> {
        Twine::from(t.as_str())
    }
}


impl<'a> From<&'a char> for Twine<'a> {
    fn from(t: &'a char) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::Char(t)))
    }
}

impl<'a> From<&'a u64> for Twine<'a> {
    fn from(t: &'a u64) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::DecU64(t)))
    }
}

impl<'a> From<&'a u32> for Twine<'a> {
    fn from(t: &'a u32) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::DecU32(t)))
    }
}

impl<'a> From<&'a u16> for Twine<'a> {
    fn from(t: &'a u16) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::DecU16(t)))
    }
}

impl<'a> From<&'a i64> for Twine<'a> {
    fn from(t: &'a i64) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::DecI64(t)))
    }
}

impl<'a> From<&'a i32> for Twine<'a> {
    fn from(t: &'a i32) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::DecI32(t)))
    }
}

impl<'a> From<&'a i16> for Twine<'a> {
    fn from(t: &'a i16) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::DecI16(t)))
    }
}

impl<'a> Twine<'a> {

    pub fn new_null() -> Twine<'a> {
        Twine(TwineKind::Null)
    }

    pub fn new_empty() -> Twine<'a> {
        Twine(TwineKind::Empty)
    }

    pub fn new_hex(t: &'a u64) -> Twine<'a> {
        Twine(TwineKind::Unary(TwineChild::HexU64(t)))
    }

    fn flatten(&'a self) -> &'a Twine<'a> {
        match self.0 {
            // TODO: better flattening by moving flatten to TwineChild
            TwineKind::Unary(TwineChild::Twine(t)) => t,
            _ => self,
        }
    }

    pub fn new_concat(lhs: &'a Twine<'a>, rhs: &'a Twine<'a>) -> Twine<'a> {
        match (lhs.flatten().0, rhs.flatten().0) {
            (TwineKind::Null, _) => Twine(TwineKind::Null),
            (_, TwineKind::Null) => Twine(TwineKind::Null),
            (TwineKind::Empty, _) => *rhs,
            (_, TwineKind::Empty) => *lhs,
            (TwineKind::Unary(l), TwineKind::Unary(r)) => Twine(TwineKind::Binary(l, r)),
            _ => Twine(TwineKind::Binary(TwineChild::Twine(lhs), TwineChild::Twine(rhs))),
        }
    }

    pub fn new_concat_strs(lhs: &'a str, rhs: &'a str) -> Twine<'a> {
        Twine(TwineKind::Binary(TwineChild::Str(lhs), TwineChild::Str(rhs)))
    }

    pub fn concat(&'a self, other: &'a Twine<'a>) -> Twine<'a> {
        Twine::new_concat(self, other)
    }

    pub fn is_nullary(&self) -> bool {
        matches!(self.0, TwineKind::Empty | TwineKind::Null)
    }

    pub fn is_unary(&self) -> bool {
        matches!(self.0, TwineKind::Unary(_))
    }

    pub fn is_binary(&self) -> bool {
        matches!(self.0, TwineKind::Binary(_, _))
    }

    pub fn is_trivially_empty(&self) -> bool {
       self.is_nullary()
    }

    pub fn is_empty(&self) -> bool {
        match self.0 {
            TwineKind::Null => true,
            TwineKind::Empty => true,
            TwineKind::Unary(child) => child.is_empty(),
            TwineKind::Binary(l_child, r_child) => l_child.is_empty() && r_child.is_empty()
        }
    }

    pub fn min_len(&self) -> usize {
        match self.0 {
            TwineKind::Null => 0,
            TwineKind::Empty => 0,
            TwineKind::Unary(child) => child.min_len(),
            TwineKind::Binary(l_child, r_child) => l_child.min_len() + r_child.min_len()
        }
    }

    pub fn print_to(&self, s: &mut String) {
        match self.0 {
            TwineKind::Null => {},
            TwineKind::Empty => {},
            TwineKind::Unary(child) => child.print_to(s),
            TwineKind::Binary(l_child, r_child) => {
                l_child.print_to(s);
                r_child.print_to(s);
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::with_capacity(self.min_len());
        self.print_to(&mut s);
        s
    }

}

impl<'a> TwineChild<'a> {
    fn is_empty(&self) -> bool {
        match self {
            TwineChild::Twine(t) => t.is_empty(),
            TwineChild::Str(s) => s.is_empty(),
            _ => false
        }
    }

    fn min_len(&self) -> usize {
        match self {
            TwineChild::Twine(t) => t.min_len(),
            TwineChild::Str(string) => string.len(),
            TwineChild::Char(ch) => ch.len_utf8(),
            TwineChild::DecU64(_) => 1,
            TwineChild::DecU32(_) => 1,
            TwineChild::DecU16(_) => 1,
            TwineChild::DecI64(_) => 1,
            TwineChild::DecI32(_) => 1,
            TwineChild::DecI16(_) => 1,
            TwineChild::HexU64(_) => 1,
        }
    }

    fn print_to(&self, s: &mut String) {
        match self {
            TwineChild::Twine(t) => t.print_to(s),
            TwineChild::Str(string) => s.push_str(*string),
            TwineChild::Char(ch) => s.push(**ch),
            TwineChild::DecU64(x) => s.push_str(&x.to_string()),
            TwineChild::DecU32(x) => s.push_str(&x.to_string()),
            TwineChild::DecU16(x) => s.push_str(&x.to_string()),
            TwineChild::DecI64(x) => s.push_str(&x.to_string()),
            TwineChild::DecI32(x) => s.push_str(&x.to_string()),
            TwineChild::DecI16(x) => s.push_str(&x.to_string()),
            TwineChild::HexU64(x) => s.push_str(&format!("{:x}", x)),
        }
    }
}
