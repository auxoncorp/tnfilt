extern crate nom;
use nom::*;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Bit {
    B0,
    B1,
}

impl Bit {
    fn val(&self) -> u32 {
        match self {
            Bit::B0 => 0,
            Bit::B1 => 1,
        }
    }
}

#[derive(Debug)]
enum Unsigned {
    UTerm,
    UInt(Box<Unsigned>, Bit),
}

impl Unsigned {
    fn from_tuple((us, b): (Unsigned, Bit)) -> Unsigned {
        Unsigned::UInt(Box::new(us), b)
    }

    fn as_u32(&self) -> u32 {
        match self {
            Unsigned::UTerm => 0,
            Unsigned::UInt(inner, bit) => (inner.as_u32() << 1) | bit.val(),
        }
    }
}

#[derive(Debug)]
enum Term {
    Unsigned(Unsigned),
    Char(char),
}

named!(bit<&str, Bit>,
       preceded!(
           alt_complete!(tag!("typenum::B") | tag!("typenum::bit::B")),
           alt!(
               value!(Bit::B0, tag!("0")) | value!(Bit::B1, tag!("1"))
           )
       )
);

named!(unsigned<&str, Unsigned>,
       alt!(
           value!(Unsigned::UTerm,
                  alt_complete!(tag!("typenum::UTerm") | tag!("typenum::uint::UTerm")))
               |
           delimited!(
               alt_complete!(tag!("typenum::UInt<") | tag!("typenum::uint::UInt<")),
               map!(
                   separated_pair!(unsigned, tag!(", "), bit),
                   Unsigned::from_tuple
               ),
               tag!(">")
           )
       )
);

named!(line<&str, Vec<Term> >,
       many0!(
           alt_complete!(
               map!(unsigned, Term::Unsigned) | map!(anychar, Term::Char)
           )
       )
);

fn main() {
    for l in io::stdin().lock().lines().map(|l| l.unwrap()) {
        for term in line(&l).unwrap().1 {
            match term {
                Term::Char(c) => print!("{}", c),
                Term::Unsigned(u) => print!("U{}", u.as_u32()),
            }
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use super::{line, Term};

    proptest! {

        #[test]
        fn passthrough_for_unrelated_content(s in r"[a-zA-Z0-9_\s\n]*") {
            let (unparsed, terms) = line(&s).unwrap();
            let mut out = String::new();
            for t in terms {
                match t {
                    Term::Char(c) => out.push(c),
                    Term::Unsigned(u) => panic!("Did not expect any unsigned"),
                }
            }
            assert_eq!(s, out.as_ref());
        }
    }

}
