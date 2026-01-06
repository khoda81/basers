// use std::collections::HashSet;

type UInt = u32;
type Base = u32;

fn gcd(mut a: UInt, mut b: UInt) -> UInt {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn divmod(a: UInt, b: UInt) -> (UInt, UInt) {
    (a / b, a % b)
}

fn divmod_base(a: UInt, b: Base) -> (UInt, Base) {
    // TODO: This can be done more robustly to avoid unwanted overflow
    (a / b as UInt, (a % b as UInt) as Base)
}

pub struct IntegerPart {
    pub n: UInt,
}

impl IntegerPart {
    pub fn new(n: UInt) -> Self {
        Self { n }
    }

    pub fn pop_digit(&mut self, base: Base) -> Base {
        let d;
        (self.n, d) = divmod_base(self.n, base);
        d
    }

    pub fn is_zero(&self) -> bool {
        self.n == 0
    }
}

#[derive(Clone, Debug)]
pub struct ProperFraction {
    p: UInt, // remainder
    // TODO: Make this NonZero
    q: UInt, // original denominator
}

impl ProperFraction {
    pub fn new(p: UInt, q: UInt) -> (IntegerPart, Self) {
        let mut this = Self { p, q };
        let n = this.pull_digit(1);

        (IntegerPart { n }, this)
    }

    pub fn simplify(&mut self) -> UInt {
        let g = gcd(self.p, self.q);

        self.q /= g;
        self.p /= g;

        g
    }

    pub fn pull_digit(&mut self, base: Base) -> UInt {
        let (d, new_r) = divmod(self.p * base as UInt, self.q);
        self.p = new_r;
        d
    }

    pub fn is_zero(&self) -> bool {
        self.numerator() == &0
    }

    pub fn denominator(&self) -> &UInt {
        &self.q
    }

    pub fn numerator(&self) -> &UInt {
        &self.p
    }
}

#[derive(Debug)]
pub enum BaseConvertorState {
    Terminal {
        // TODO: This can be NonZero
        q_term: UInt,
        // TODO: This can be NonZero
        g: UInt,
    },
    Repeating {
        start: UInt,
    },
}

#[derive(Debug)]
pub struct BaseConvertor {
    base: Base,
    fractional: ProperFraction,
    state: BaseConvertorState,
}

type Digit = Base;

#[derive(Debug)]
pub enum Token {
    Terminal(Digit),
    Repeating(Digit),
    RepeatingEnd(Digit),
}

impl BaseConvertor {
    pub fn new_fraction(mut f: ProperFraction, base: Base) -> Self {
        f.simplify();
        let g = gcd(f.q, base);

        Self {
            state: BaseConvertorState::Terminal { q_term: f.q, g },
            fractional: f,
            base,
        }
    }
}

impl BaseConvertor {
    pub fn state(&self) -> &BaseConvertorState {
        &self.state
    }

    pub fn next_token(&mut self) -> Token {
        match &mut self.state {
            BaseConvertorState::Terminal { q_term, g } => {
                let d = self.fractional.pull_digit(self.base);

                *g = gcd(*q_term, *g);
                *q_term /= *g as UInt;

                if g == &1 {
                    self.state = BaseConvertorState::Repeating {
                        start: self.fractional.p,
                    };
                    if self.fractional.numerator() == &0 {
                        Token::RepeatingEnd(d)
                    } else {
                        Token::Repeating(d)
                    }
                } else {
                    Token::Terminal(d)
                }
            }

            BaseConvertorState::Repeating { start } => {
                let d = self.fractional.pull_digit(self.base);

                if self.fractional.p == *start {
                    Token::RepeatingEnd(d)
                } else {
                    Token::Repeating(d)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
