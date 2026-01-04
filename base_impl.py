from fractions import Fraction
from math import gcd
from typing import Iterable

DIGITS = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"


def to_base_fraction_iter_fast_v2(frac: Fraction, base: int = 10) -> Iterable[str]:
    if frac < 0:
        yield "-"
        frac = -frac

    q = frac.denominator

    # Integer part
    integer, rem = divmod(frac.numerator, q)

    if integer == 0:
        yield DIGITS[0]

    digits = []
    while integer:
        integer, d = divmod(integer, base)
        digits.append(DIGITS[d])

    yield from reversed(digits)

    if not rem:
        return

    yield "."

    # --- terminating part ---
    q_term = q
    g = base
    while (g := gcd(q_term, g)) != 1:
        q_term //= g

        d, rem = divmod(rem * base, q)
        yield DIGITS[d]

    if not rem:
        return

    yield "("

    # --- repeating part ---
    seen = rem

    while True:
        d, rem = divmod(rem * base, q)
        yield DIGITS[d]
        if rem == seen:
            break

    yield ")"
