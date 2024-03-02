#![allow(dead_code)]

use std::collections::VecDeque;
use std::env;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Sub};

use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use rand::RngCore;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref DATA_DIR: String = match env::var("CUBE_DIR") {
        Ok(d) => d,
        Err(_) => match env::var("HOME") {
            Ok(h) => format!("{}/.local/share/Cubatrice/data", h),
            Err(_) => String::from("./data"),
        },
    };
}

/// Game Entity representation
pub mod entity;
/// Game state representation
pub mod state;

/// Common number type to represent fractions, when floating point isn't
/// necessary, and fractions make more sense.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Fraction {
    n: isize,
    d: isize,
}

impl Fraction {
    /// Creates a fraction with a given numerator and denominator
    pub fn new(n: isize, d: isize) -> Self {
        let mut f = Fraction { n, d };
        f.reduce();
        f
    }

    /// Gets the floating point value of the fraction
    pub fn value(&self) -> f64 {
        (self.n as f64) / (self.d as f64)
    }

    /// Creates a new fraction with numerator and denominator swapped
    pub fn reciprocal(&self) -> Self {
        Fraction {
            n: self.d,
            d: self.n,
        }
    }

    /// Gets the integer component of the fraction
    pub fn integer(&self) -> isize {
        self.n / self.d
    }

    /// Gets the remainder of the fraction after the integer component is
    /// removed
    pub fn remainder(&self) -> isize {
        self.n % self.d
    }

    /// Numerator of the fraction
    pub fn numerator(&self) -> isize {
        self.n
    }

    /// Denominator of the fraction
    pub fn denominator(&self) -> isize {
        self.d
    }

    /// Internal function to reduce the fraction down to simplest form.
    /// Called after every operation to ensure that fractions stay in
    /// simplest form at all times.
    fn reduce(&mut self) {
        let gcd = gcd(self.n, self.d);
        self.n /= gcd;
        self.d /= gcd;
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.n * other.d).cmp(&(self.d * other.n))
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.integer(),
            if self.remainder() > 0 {
                format!(" + {}/{}", self.remainder(), self.denominator())
            } else {
                String::new()
            }
        )
    }
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Self) -> Self::Output {
        let mut f = Fraction {
            n: (self.n * rhs.d) + (rhs.n * self.d),
            d: (self.d * rhs.d),
        };
        f.reduce();
        f
    }
}

impl Add<isize> for Fraction {
    type Output = Fraction;

    fn add(self, rhs: isize) -> Self::Output {
        let mut f = Fraction {
            n: self.n + (self.d * rhs),
            d: self.d,
        };
        f.reduce();
        f
    }
}

impl Neg for Fraction {
    type Output = Fraction;

    fn neg(self) -> Self::Output {
        Fraction {
            n: -self.n,
            d: self.d,
        }
    }
}

impl Sub for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Sub<isize> for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: isize) -> Self::Output {
        self + -rhs
    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut f = Fraction {
            n: (self.n * rhs.n),
            d: (self.d * rhs.d),
        };
        f.reduce();
        f
    }
}

impl Mul<isize> for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: isize) -> Self::Output {
        let mut f = Fraction {
            n: (self.n * rhs),
            d: self.d,
        };
        f.reduce();
        f
    }
}

impl Div for Fraction {
    type Output = Fraction;

    fn div(self, rhs: Self) -> Self::Output {
        self * (rhs.reciprocal())
    }
}

impl Div<isize> for Fraction {
    type Output = Fraction;

    fn div(self, rhs: isize) -> Self::Output {
        Fraction {
            n: self.n,
            d: self.d * rhs,
        }
    }
}

/// Does what it says on the tin. You've seen this algorithm before.
fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[derive(Clone, Default, Debug)]
pub struct Deck<T> {
    items: VecDeque<T>,
}

impl<T> Deck<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items: items.into(),
        }
    }

    pub fn new_shuffled(mut items: Vec<T>) -> Self {
        items.shuffle(&mut rand::thread_rng());
        Self {
            items: items.into(),
        }
    }

    pub fn new_shuffled_with_rng<R>(mut items: Vec<T>, rng: &mut R) -> Self
    where
        R: RngCore,
    {
        items.shuffle(rng);
        Self {
            items: items.into(),
        }
    }

    pub fn draw_next(&mut self) -> Option<T> {
        self.draw_next_matches(|_| true)
    }

    pub fn draw_next_matches<P>(&mut self, pred: P) -> Option<T>
    where
        P: Fn(&T) -> bool,
    {
        if self.items.len() == 0 {
            return None;
        }
        let mut item = self.items.pop_front().unwrap();
        let mut count = 0;
        while !pred(&item) {
            count += 1;
            self.items.push_back(item);
            if count > self.items.len() {
                return None;
            }
            item = self.items.pop_front().unwrap();
        }
        Some(item)
    }

    pub fn add_to_bottom(&mut self, item: T) {
        self.items.push_back(item);
    }

    pub fn add_to_top(&mut self, item: T) {
        self.items.push_front(item);
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.get(0)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
