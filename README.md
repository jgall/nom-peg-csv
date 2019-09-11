## A very small implementation of a CSV parser that is to spec with the [CSV spec](https://www.ietf.org/rfc/rfc4180.txt).

The only non-to-spec portion of this parser is that you must use a `\u{003}` (i.e. `End Of Text`) character as a terminal to your input.

## Why build this when other CSV parsers are probably better? 

Building this with [nom-peg](https://github.com/rust-bakery/nom-peg) only took about half an hour following the CSV spec directly from the IETF website. Buidling this was mostly an exercise in seeing how easy it was to use `nom-peg`. I was able to almost exactly copy the `ABNF` from the spec line by line which made the work take very little time. 
