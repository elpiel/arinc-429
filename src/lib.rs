use nom::{IResult, bytes::streaming::take};

pub fn parse_word(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take(4usize)(i)
}

pub fn is_word_correct(i: &[u8; 4]) -> bool {
    // "odd" parity - there must be an odd number of "1" bits in the word
    i.iter().fold(0, |acc, byte| acc + byte.count_ones()) % 2 != 0
}

fn take_byte(i: &[u8]) -> IResult<&[u8], u8> {
    use nom::bytes::complete::take;
    use nom::combinator::map;

    map(take(1_usize), |bytes: &[u8]| bytes[0])(i)
}

pub type Label = u8;

pub fn parse_label(i: &[u8]) -> IResult<&[u8], Label> {
    take_byte(i)
}

/// Source/Destination Identifiers
#[derive(Debug, PartialEq, Eq)]
pub enum SDI {
    First, // 00
    Second, // 01
    Third, // 10
    Fourth, // 11
}

pub fn bits_pair(i: (&[u8], usize)) ->  IResult<(&[u8], usize), u8> {
    nom::bits::complete::take(2_u8)(i)
}

pub fn parse_sdi(i: &[u8]) -> IResult<&[u8], SDI> {
    use nom::combinator::map;
    use nom::bits::bits;

    // @TODO: Fix this! We should collect the other bits, or they will be thrown away
    map(bits(bits_pair), |sdi_num: u8| {
        match sdi_num {
            0 => SDI::First,
            1 => SDI::Second,
            2 => SDI::Third,
            3 => SDI::Fourth,
            _ => unreachable!("Unreachable! We only get 2 bits.."),
        }
    })(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_sdi() {
        let expected_results: Vec<(SDI, u8)> = vec![
            // 00
            (SDI::First, 0b00_111111),
            // 01
            (SDI::Second, 0b01_111111),
            // 10
            (SDI::Third, 0b10_111111),
             //11
            (SDI::Fourth, 0b11_111111),
        ];
        
        for (expected, value) in expected_results {
            let input = [value];
            let actual = parse_sdi(&input).expect("Should succeed");
            dbg!(actual.0);

            assert_eq!(actual.1, expected);
        }
    }

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
