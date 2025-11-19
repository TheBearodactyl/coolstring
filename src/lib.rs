use std::ops::{Add, AddAssign, Index, Mul, MulAssign, Range, RangeFull, Sub, SubAssign};

pub struct CoolString(pub String);

impl Add for CoolString {
    type Output = String;
    fn add(self, rhs: Self) -> Self::Output {
        format!("{}{}", self.0, rhs.0)
    }
}

impl Add<&str> for CoolString {
    type Output = String;

    fn add(mut self, rhs: &str) -> Self::Output {
        self.0.push_str(rhs);
        self.0
    }
}

impl From<String> for CoolString {
    fn from(value: String) -> Self {
        CoolString(value)
    }
}

impl AddAssign<&str> for CoolString {
    fn add_assign(&mut self, rhs: &str) {
        self.0.push_str(rhs);
    }
}

impl AddAssign<String> for CoolString {
    fn add_assign(&mut self, rhs: String) {
        self.0.push_str(&rhs);
    }
}

impl AddAssign for CoolString {
    fn add_assign(&mut self, rhs: CoolString) {
        self.0.push_str(&rhs.0);
    }
}

impl Mul<usize> for CoolString {
    type Output = String;
    fn mul(self, rhs: usize) -> Self::Output {
        let mut s = String::new();
        s.reserve(self.0.len() * rhs);
        for _ in 0..rhs {
            s.push_str(&self.0);
        }
        CoolString(s).0
    }
}

impl Mul<usize> for &CoolString {
    type Output = String;
    fn mul(self, rhs: usize) -> Self::Output {
        let mut s = String::new();
        s.reserve(self.0.len() * rhs);
        for _ in 0..rhs {
            s.push_str(&self.0);
        }
        CoolString(s).0
    }
}

impl MulAssign<usize> for CoolString {
    fn mul_assign(&mut self, rhs: usize) {
        let original = self.0.clone();
        self.0.clear();
        self.0.reserve(original.len() * rhs);
        for _ in 0..rhs {
            self.0.push_str(&original);
        }
    }
}

impl Index<Range<usize>> for CoolString {
    type Output = str;
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<RangeFull> for CoolString {
    type Output = str;
    fn index(&self, _index: RangeFull) -> &Self::Output {
        &self.0
    }
}

impl PartialEq<str> for CoolString {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for CoolString {
    fn eq(&self, other: &&str) -> bool {
        &*self.0 == *other
    }
}

impl Sub<&str> for CoolString {
    type Output = String;
    fn sub(self, rhs: &str) -> Self::Output {
        CoolString(self.0.replace(rhs, "")).0
    }
}

impl Sub<String> for CoolString {
    type Output = String;
    fn sub(self, rhs: String) -> Self::Output {
        CoolString(self.0.replace(&rhs, "")).0
    }
}

impl Sub for CoolString {
    type Output = String;
    fn sub(self, rhs: CoolString) -> String {
        CoolString(self.0.replace(&rhs.0, "")).0
    }
}

impl SubAssign<&str> for CoolString {
    fn sub_assign(&mut self, rhs: &str) {
        *self = CoolString(self.0.replace(rhs, ""));
    }
}

impl SubAssign<String> for CoolString {
    fn sub_assign(&mut self, rhs: String) {
        *self = CoolString(self.0.replace(&rhs, ""));
    }
}

impl SubAssign for CoolString {
    fn sub_assign(&mut self, rhs: CoolString) {
        *self = CoolString(self.0.replace(&rhs.0, ""));
    }
}
