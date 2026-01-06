use std::io::{self, Read, Write};

use basers::{BaseConvertor, ProperFraction, Token};

type UInt = u32;
type Base = u32;

const DIGITS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn parse_input(s: &str) -> (UInt, UInt, Base) {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        panic!("expected input: p[/q][@b]");
    }

    let (frac_part, base_part) = match trimmed.split_once('@') {
        Some((left, right)) => (left, Some(right)),
        None => (trimmed, None),
    };

    let base = match base_part {
        Some(b) if !b.is_empty() => b.parse::<Base>().expect("invalid base"),
        Some(_) => panic!("invalid base"),
        None => 10,
    };

    let (p, q) = match frac_part.split_once('/') {
        Some((p, q)) => (
            p.parse::<UInt>().expect("invalid numerator"),
            q.parse::<UInt>().expect("invalid denominator"),
        ),
        None => (frac_part.parse::<UInt>().expect("invalid numerator"), 1),
    };

    (p, q, base)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (p, q, base) = parse_input(&input);

    /* ---- integer part ---- */

    let (mut int, frac) = ProperFraction::new(p, q);
    let mut out = io::BufWriter::new(io::stdout());

    if int.is_zero() {
        out.write_all(&[DIGITS[0]])?;
    } else {
        let mut int_buf = Vec::new();
        while !int.is_zero() {
            int_buf.push(DIGITS[int.pop_digit(base) as usize]);
        }

        int_buf
            .into_iter()
            .rev()
            .try_for_each(|c| out.write_all(&[c]))?;
    }

    if frac.is_zero() {
        return out.write_all(b"\n");
    }

    out.write_all(b".")?;

    /* ---- fractional part (streaming) ---- */

    let mut conv = BaseConvertor::new_fraction(frac, base);

    loop {
        match conv.next_token() {
            Token::Terminal(d) => out.write_all(&[DIGITS[d as usize]])?,

            Token::Repeating(d) => {
                out.write_all(&[b'(', DIGITS[d as usize]])?;

                break loop {
                    match conv.next_token() {
                        Token::Terminal(_) => unreachable!(),
                        Token::Repeating(d) => out.write_all(&[DIGITS[d as usize]])?,
                        Token::RepeatingEnd(_) => break out.write_all(b")"),
                    };
                };
            }

            Token::RepeatingEnd(_) => break out.write_all(b"\n"),
        }
    }
}
