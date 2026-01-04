use std::env;
use std::io::{self, Write};

use basers::{BaseConvertor, ProperFraction, Token};

type UInt = u128;
type Base = u128;

const DIGITS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn parse_fraction(s: &str) -> (UInt, UInt) {
    let (p, q) = s.split_once('/').expect("expected p/q");
    (
        p.parse::<UInt>().expect("invalid numerator"),
        q.parse::<UInt>().expect("invalid denominator"),
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: baser p/q --base N");
        std::process::exit(1);
    }

    let (p, q) = parse_fraction(&args[1]);

    let mut base: Base = 10;
    if let Some(i) = args.iter().position(|a| a == "--base") {
        base = args
            .get(i + 1)
            .expect("missing base")
            .parse()
            .expect("invalid base");
    }

    /* ---- integer part ---- */

    let (mut int, frac) = ProperFraction::new(p, q);

    // dbg!(&int.n);
    // dbg!(frac.numerator(), frac.denominator());

    let mut int_buf = Vec::new();
    while !int.is_zero() {
        int_buf.push(DIGITS[int.pop_digit(base) as usize]);
    }
    int_buf.reverse();

    let mut out = io::BufWriter::new(io::stdout());
    if int_buf.is_empty() {
        out.write_all(&[DIGITS[0]])
    } else {
        out.write_all(&int_buf)
    }
    .unwrap();

    if frac.numerator() == &0 {
        out.write_all(b"\n").unwrap();
        return;
    }

    out.write_all(b".").unwrap();

    /* ---- fractional part (streaming) ---- */

    let mut conv = BaseConvertor::new_fraction(frac, base);

    loop {
        match conv.next_token() {
            Token::Terminal(d) => {
                out.write_all(&[DIGITS[d as usize]]).unwrap();
            }

            Token::Repeating(d) => {
                out.write_all(b"(").unwrap();
                out.write_all(&[DIGITS[d as usize]]).unwrap();

                loop {
                    match conv.next_token() {
                        Token::Terminal(_) => unreachable!(),
                        Token::Repeating(d) => {
                            out.write_all(&[DIGITS[d as usize]]).unwrap();
                        }

                        Token::RepeatingEnd => {
                            out.write_all(b")").unwrap();
                            break;
                        }
                    };
                }

                break;
            }

            Token::RepeatingEnd => {
                break;
            }
        }
    }
    // loop {
    //     dbg!(&conv);
    //     match dbg!(conv.next_token()) {
    //         Token::Terminal(d) => {
    //             out.write_all(b"Term(").unwrap();
    //             out.write_all(&[DIGITS[d as usize]]).unwrap();
    //             out.write_all(b")\n").unwrap();
    //         }

    //         Token::Repeating(d) => {
    //             out.write_all(b"Rep(").unwrap();
    //             out.write_all(&[DIGITS[d as usize]]).unwrap();
    //             out.write_all(b")\n").unwrap();
    //         }

    //         Token::RepeatingEnd(d) => {
    //             out.write_all(b"End(").unwrap();
    //             // out.write_all(&[DIGITS[d as usize]]).unwrap();
    //             out.write_all(b")\n").unwrap();
    //             break;
    //         }
    //     }
    // }

    // let seen_repeating = false;

    out.write_all(b"\n").unwrap();
}
