use std::process::Command;
use assert_cmd::prelude::*;
use proptest::prelude::*;
use assert_cmd::stdin::CommandStdInExt;

proptest! {
    #[test]
    fn passthrough_for_unrelated_content(s in r"[a-zA-Z0-9_]+") {
        let mut trimmed = {
            let t = s.clone();
            t.trim().to_owned()
        };
        trimmed.push('\n');

        Command::cargo_bin("tnfilt")
            .unwrap()
            .with_stdin().buffer(s.clone())
            .assert().stdout(trimmed);
    }
}

fn to_typenum_type(v: u32) -> String {
    /// type U6 = UInt<UInt<UInt<UTerm, B1>, B1>, B0>;
    use bitvec::prelude::*;
    if v == 0 {
        "typenum::UTerm".to_owned()
    } else {
        let num_leading_zeros = v.leading_zeros();
        let bits = if num_leading_zeros > 0 {
            let s = v.as_bitslice::<BigEndian>();
            let (_leading_zeros, rest) = s.split_at(num_leading_zeros as usize);
            rest
        } else {
            v.as_bitslice()
        };
        let mut iter = bits.iter();
        let mut typestring = format!("typenum::UInt<typenum::UTerm, typenum::B{}>", if iter.next().unwrap() { '1'} else {'0'});
        for bit in iter {
            let bitchar= if bit { '1'} else {'0'};
            typestring = format!("typenum::UInt<{}, typenum::B{}>", typestring, bitchar);
        }
        typestring
    }
}

#[test]
fn spot_check() {
    assert_eq!("typenum::UTerm", to_typenum_type(0).as_str());
    assert_eq!("typenum::UInt<typenum::UTerm, typenum::B1>", to_typenum_type(1).as_str());
    assert_eq!("typenum::UInt<typenum::UInt<typenum::UTerm, typenum::B1>, typenum::B0>", to_typenum_type(2).as_str());
    assert_eq!("typenum::UInt<typenum::UInt<typenum::UInt<typenum::UTerm, typenum::B1>, typenum::B1>, typenum::B0>", to_typenum_type(6).as_str())
}

proptest! {
    #[test]
    fn interprets_typenums(v in any::<u32>()) {
        let s = to_typenum_type(v);
        let expected = format!("U{}\n", v);

        Command::cargo_bin("tnfilt")
            .unwrap()
            .with_stdin().buffer(s.clone())
            .assert().stdout(expected);
    }

    #[test]
    fn interprets_mixed_content(a in any::<u32>(), b in any::<u32>(), s in r"[a-zA-Z0-9_]+") {
        let input = format!("{} {} {}", to_typenum_type(a), s.clone(), to_typenum_type(b));
        let expected = format!("U{} {} U{}\n", a, s.clone(), b);

        Command::cargo_bin("tnfilt")
            .unwrap()
            .with_stdin().buffer(input)
            .assert().stdout(expected);
    }
}
