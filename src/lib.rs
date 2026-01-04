// use std::collections::HashSet;

type UInt = u128;
type Base = u128;

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

pub struct IntegerPart {
    n: UInt,
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

#[derive(Clone)]
pub struct ProperFraction {
    r: UInt, // remainder
    // TODO: Make this NonZero
    q: UInt, // original denominator
}

impl ProperFraction {
    pub fn new(p: UInt, q: UInt) -> (IntegerPart, Self) {
        let mut this = Self { r: p, q };
        let n = this.pull_digit(q);

        (IntegerPart { n }, this)
    }

    pub fn simplify(&mut self) -> u128 {
        let g = gcd(self.r, self.q);

        self.q /= g;
        self.r /= g;

        g
    }

    pub fn pull_digit(&mut self, base: Base) -> UInt {
        let (d, new_r) = divmod(self.r * base as UInt, self.q);
        self.r = new_r;
        d
    }

    pub fn denominator(&self) -> UInt {
        self.q
    }
}

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

pub struct BaseConvertor {
    base: Base,
    fractional: ProperFraction,
    state: BaseConvertorState,
}

type Digit = Base;

pub enum Token {
    Terminal(Digit),
    Repeating(Digit),
    RepeatingStart(Digit),
    RepeatingEnd(Digit),
}

impl BaseConvertor {
    pub fn new_fraction(mut f: ProperFraction, base: Base) -> Self {
        f.simplify();

        Self {
            state: BaseConvertorState::Terminal {
                q_term: f.q,
                g: base,
            },
            fractional: f,
            base,
        }
    }
}

impl BaseConvertor {
    pub fn next_token(&mut self) -> Token {
        match &mut self.state {
            BaseConvertorState::Terminal { q_term, g } => {
                *g = gcd(*q_term, *g);
                let d = self.fractional.pull_digit(self.base);

                if g == &1 {
                    self.state = BaseConvertorState::Repeating {
                        start: self.fractional.r,
                    };
                    Token::RepeatingStart(d)
                } else {
                    *q_term /= *g as UInt;
                    Token::Terminal(d)
                }
            }

            BaseConvertorState::Repeating { start } => {
                let d = self.fractional.pull_digit(self.base);

                if self.fractional.r == *start {
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
