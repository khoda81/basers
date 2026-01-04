// use std::collections::HashSet;

type UInt = u128;
type Base = u32;

fn gcd(a: UInt, b: Base) -> Base {
    todo!()
}

fn divmod(a: UInt, b: UInt) -> (UInt, UInt) {
    (a / b, a % b)
}

fn divmod_base(a: UInt, b: Base) -> (UInt, Base) {
    // TODO: This can be done more robustly to avoid unwanted overflow
    (a / b as UInt, (a % b as UInt) as Base)
}

pub struct IntegerPart(UInt);

impl IntegerPart {
    pub fn new(n: UInt) -> Self {
        Self(n)
    }

    pub fn pop_digit(Self(n): &mut Self, base: Base) -> Base {
        let d;
        (*n, d) = divmod_base(*n, base);
        d
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

#[derive(Copy, Clone)]
pub struct ProperFraction {
    r: UInt, // remainder
    // TODO: Make this NonZero
    q: UInt, // original denominator
}

#[derive(Clone)]
pub struct RepeatingFraction {
    fraction: ProperFraction,
    start: UInt,
    base: Base,
}

#[derive(Clone)]
pub enum Fractional {
    Mixed(UInt, ProperFraction),
    Repeating(RepeatingFraction),
}

impl ProperFraction {
    pub fn new(p: UInt, q: UInt) -> (IntegerPart, Self) {
        let (int, r) = divmod(p, q);
        (IntegerPart(int), Self { r, q })
    }

    pub fn pull_digit(&mut self, base: Base) -> UInt {
        let d;
        (d, self.r) = divmod(self.r * base as UInt, self.q);
        d
    }

    pub fn shifted_fraction(mut self, base: Base) -> Fractional {
        let g = gcd(self.q, base);

        if g == 1 {
            return Fractional::Repeating(RepeatingFraction::new_unchecked(base, self));
        }

        (self.q, _) = divmod_base(self.q, g);
        let d = self.pull_digit(base / g);
        Fractional::Mixed(d, self)
    }
}

impl RepeatingFraction {
    fn new_unchecked(base: Base, fraction: ProperFraction) -> RepeatingFraction {
        RepeatingFraction {
            base,
            fraction,
            start: fraction.r,
        }
    }

    pub fn pop_repeat_digit(&mut self) -> UInt {
        self.fraction.pull_digit(self.base)
    }

    pub fn is_at_start(&self) -> bool {
        self.fraction.r == self.start
    }

    pub fn fraction(&self) -> &ProperFraction {
        &self.fraction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
