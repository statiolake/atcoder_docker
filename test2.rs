use proconio::input;
#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::stdin;
#[allow(unused_imports)]
use std::iter::FromIterator;
mod util {
    use std::fmt::Debug;
    use std::io::stdin;
    use std::str::FromStr;
    #[allow(dead_code)]
    pub fn line() -> String {
        let mut line: String = String::new();
        stdin().read_line(&mut line).unwrap();
        line.trim().to_string()
    }
    #[allow(dead_code)]
    pub fn gets<T: FromStr>() -> Vec<T>
    where
        <T as FromStr>::Err: Debug,
    {
        let mut line: String = String::new();
        stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|t| t.parse().unwrap())
            .collect()
    }
}
#[allow(unused_macros)]
macro_rules ! get { ( $ t : ty ) => { { let mut line : String = String :: new ( ) ; stdin ( ) . read_line ( & mut line ) . unwrap ( ) ; line . trim ( ) . parse ::<$ t > ( ) . unwrap ( ) } } ; ( $ ( $ t : ty ) ,* ) => { { let mut line : String = String :: new ( ) ; stdin ( ) . read_line ( & mut line ) . unwrap ( ) ; let mut iter = line . split_whitespace ( ) ; ( $ ( iter . next ( ) . unwrap ( ) . parse ::<$ t > ( ) . unwrap ( ) , ) * ) } } ; ( $ t : ty ; $ n : expr ) => { ( 0 ..$ n ) . map ( | _ | get ! ( $ t ) ) . collect ::< Vec < _ >> ( ) } ; ( $ ( $ t : ty ) ,*; $ n : expr ) => { ( 0 ..$ n ) . map ( | _ | get ! ( $ ( $ t ) ,* ) ) . collect ::< Vec < _ >> ( ) } ; ( $ t : ty ;; ) => { { let mut line : String = String :: new ( ) ; stdin ( ) . read_line ( & mut line ) . unwrap ( ) ; line . split_whitespace ( ) . map ( | t | t . parse ::<$ t > ( ) . unwrap ( ) ) . collect ::< Vec < _ >> ( ) } } ; }
#[allow(unused_macros)]
macro_rules ! debug { ( $ ( $ a : expr ) ,* ) => { println ! ( concat ! ( $ ( stringify ! ( $ a ) , " = {:?}, " ) ,* ) , $ ( $ a ) ,* ) ; } }

const TRUE: &'static bool = &true;
const FALSE: &'static bool = &false;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Efficient bool collection
pub struct BitSet {
    buf: Vec<u64>,
    size: usize,
}

impl BitSet {
    pub fn new(size: usize) -> BitSet {
        BitSet {
            buf: vec![0; (size + 63) / 64],
            size: size,
        }
    }

    pub fn set(&mut self, i: usize, b: bool) {
        assert!(i < self.size);
        if b {
            self.buf[i >> 6] |= 1 << (i & 63);
        } else {
            self.buf[i >> 6] &= !(1 << (i & 63));
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn count_ones(&self) -> u32 {
        self.buf.iter().map(|x| x.count_ones()).sum()
    }

    pub fn count_zeros(&self) -> u32 {
        self.buf.iter().map(|x| x.count_zeros()).sum()
    }

    fn chomp(&mut self) {
        let r = self.size & 63;
        if r != 0 {
            if let Some(x) = self.buf.last_mut() {
                let d = 64 - r;
                *x = (*x << d) >> d;
            }
        }
    }
}

impl std::ops::Index<usize> for BitSet {
    type Output = bool;
    fn index(&self, index: usize) -> &bool {
        [FALSE, TRUE][(self.buf[index >> 6] >> (index & 63)) as usize & 1]
    }
}

impl std::ops::ShlAssign<usize> for BitSet {
    fn shl_assign(&mut self, x: usize) {
        let q = x >> 6;
        let r = x & 63;

        if q >= self.buf.len() {
            for x in &mut self.buf {
                *x = 0;
            }
            return;
        }

        if r == 0 {
            for i in (q..self.buf.len()).rev() {
                *unsafe { self.buf.get_unchecked_mut(i) } =
                    *unsafe { self.buf.get_unchecked(i - q) };
            }
        } else {
            for i in (q + 1..self.buf.len()).rev() {
                *unsafe { self.buf.get_unchecked_mut(i) } =
                    (unsafe { self.buf.get_unchecked(i - q) } << r)
                        | (unsafe { self.buf.get_unchecked(i - q - 1) } >> (64 - r));
            }
            *unsafe { self.buf.get_unchecked_mut(q) } = unsafe { self.buf.get_unchecked(0) } << r;
        }

        for x in &mut self.buf[..q] {
            *x = 0;
        }

        self.chomp();
    }
}

impl std::ops::Shl<usize> for BitSet {
    type Output = Self;

    fn shl(mut self, x: usize) -> Self {
        self <<= x;
        self
    }
}

impl<'a> std::ops::Shl<usize> for &'a BitSet {
    type Output = BitSet;
    fn shl(self, x: usize) -> Self::Output {
        let mut result = self.clone();
        result <<= x;
        result
    }
}

impl std::ops::ShrAssign<usize> for BitSet {
    fn shr_assign(&mut self, x: usize) {
        let q = x >> 6;
        let r = x & 63;

        if q >= self.buf.len() {
            for x in &mut self.buf {
                *x = 0;
            }
            return;
        }

        if r == 0 {
            for i in 0..self.buf.len() - q {
                self.buf[i] = self.buf[i + q];
            }
        } else {
            for i in 0..self.buf.len() - q - 1 {
                self.buf[i] = (self.buf[i + q] >> r) | (self.buf[i + q + 1] << (64 - r));
            }
            let len = self.buf.len();
            self.buf[len - q - 1] = self.buf[len - 1] >> r;
        }

        let len = self.buf.len();
        for x in &mut self.buf[len - q..] {
            *x = 0;
        }
    }
}

impl std::ops::Shr<usize> for BitSet {
    type Output = Self;

    fn shr(mut self, x: usize) -> Self {
        self >>= x;
        self
    }
}

impl<'a> std::ops::Shr<usize> for &'a BitSet {
    type Output = BitSet;

    fn shr(self, x: usize) -> Self::Output {
        let mut result = self.clone();
        result >>= x;
        result
    }
}

impl<'a> std::ops::BitAndAssign<&'a BitSet> for BitSet {
    fn bitand_assign(&mut self, rhs: &'a Self) {
        for (a, b) in self.buf.iter_mut().zip(rhs.buf.iter()) {
            *a &= *b;
        }
    }
}

impl<'a> std::ops::BitAnd<&'a BitSet> for BitSet {
    type Output = Self;
    fn bitand(mut self, rhs: &'a Self) -> Self::Output {
        self &= rhs;
        self
    }
}

impl<'a, 'b> std::ops::BitAnd<&'b BitSet> for &'a BitSet {
    type Output = BitSet;
    fn bitand(self, rhs: &'b BitSet) -> Self::Output {
        let mut result = self.clone();
        result &= rhs;
        result
    }
}

impl<'a> std::ops::BitOrAssign<&'a BitSet> for BitSet {
    fn bitor_assign(&mut self, rhs: &'a Self) {
        for (a, b) in self.buf.iter_mut().zip(rhs.buf.iter()) {
            *a |= *b;
        }
        self.chomp();
    }
}

impl<'a> std::ops::BitOr<&'a BitSet> for BitSet {
    type Output = Self;
    fn bitor(mut self, rhs: &'a Self) -> Self::Output {
        self |= rhs;
        self
    }
}

impl<'a, 'b> std::ops::BitOr<&'b BitSet> for &'a BitSet {
    type Output = BitSet;
    fn bitor(self, rhs: &'b BitSet) -> Self::Output {
        let mut result = self.clone();
        result |= rhs;
        result
    }
}

impl<'a> std::ops::BitXorAssign<&'a BitSet> for BitSet {
    fn bitxor_assign(&mut self, rhs: &'a Self) {
        for (a, b) in self.buf.iter_mut().zip(rhs.buf.iter()) {
            *a ^= *b;
        }
        self.chomp();
    }
}

impl<'a> std::ops::BitXor<&'a BitSet> for BitSet {
    type Output = Self;
    fn bitxor(mut self, rhs: &'a Self) -> Self {
        self ^= rhs;
        self
    }
}

impl<'a, 'b> std::ops::BitXor<&'b BitSet> for &'a BitSet {
    type Output = BitSet;
    fn bitxor(self, rhs: &'b BitSet) -> Self::Output {
        let mut result = self.clone();
        result ^= rhs;
        result
    }
}

impl std::ops::Not for BitSet {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for x in &mut self.buf {
            *x = !*x;
        }
        self.chomp();
        self
    }
}

impl<'a> std::ops::Not for &'a BitSet {
    type Output = BitSet;
    fn not(self) -> Self::Output {
        !self.clone()
    }
}

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    };
    let sum: usize = aa.iter().sum();

    let mut bitset = BitSet::new(sum + 1);
    bitset.set(0, true);

    for &a in &aa {
        bitset |= &(&bitset << a);
    }

    let ans = ((sum + 1) / 2..).find(|&i| bitset[i]).unwrap();
    println!("{}", ans);
}
