# basers

Fast exact base conversion for rational numbers in Rust.

`basers` converts exact rational values between numeral systems and detects whether an expansion terminates or repeats instead of silently falling back to floating-point approximations.

## What it is for

Base conversion gets subtle as soon as fractions enter the picture. A rational number may terminate in one radix and repeat forever in another; because `basers` works from exact rational values, it can represent that structure directly.

## Goals

- exact rational arithmetic rather than floating-point approximation
- automatic detection of terminating and repeating expansions
- fast conversion suitable for interactive or batch use
- a small, predictable Rust implementation

See the source and tests for the currently supported API and conversion behavior.
