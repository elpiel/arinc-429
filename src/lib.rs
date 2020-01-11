use nom::{IResult, bytes::streaming::take};

fn parse_word(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take(4usize)(i)
}

fn is_word_correct(i: &[u8; 4]) -> bool {
    // "odd" parity - there must be an odd number of "1" bits in the word
    i.iter().fold(0, |acc, byte| acc + byte.count_ones()) % 2 != 0
}

fn take_byte(i: &[u8]) -> IResult<&[u8], u8> {
    use nom::bytes::complete::take;
    use nom::combinator::map;

    map(take(1_usize), |bytes: &[u8]| bytes[0])(i)
}

pub type Label = u8;

fn parse_label(i: &[u8]) -> IResult<&[u8], Label> {
    take_byte(i)
}

/// Source/Destination Identifiers
pub enum SDI {
    First, // 00
    Second, // 01
    Third, // 10
    Forth, // 11
}

pub fn parse_sdi(i: &[u8]) -> IResult<&[u8], SDI> {
    use nom::combinator::map_res;
    use nom::bytes::complete::take as take_bytes;
    use nom::bits::{bits, complete::take};

    let byte = take_byte(i);

    let bits = bits::<_, _, _>(take::<_, u8, _, (_, _)>(2_usize))(i);

    dbg!(bits);

    Ok((i, SDI::First))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_label() {
        // the example from wikipedia, but odered from 1..32 bit (as they are received)
        let test = [0b10110000, 0b00100010, 0b00110001, 0b10001001];

        // 260 Octal
        let label = parse_label(&test).expect("Should be OK").1;
        assert_eq!("260", format!("{:o}", label));
    }

    #[test]
    fn parity_bit_correctness() {
        let ok = [0b00000001, 0b00000001, 0b00000001, 0b00000001];
        assert!(!is_word_correct(&ok));

        let not_ok = [0b00000001, 0b00000001, 0b00000001, 0b00000011];
        assert!(is_word_correct(&not_ok));
    }
}
