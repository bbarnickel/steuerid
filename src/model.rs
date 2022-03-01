use std::fmt::Display;

use rand::{prelude::SliceRandom, Rng};

#[derive(Debug, Clone)]
pub enum SteuerIdCheckError {
    ZeroAtStart,
    InvalidDigit(usize),
    InvalidNumbers,
    InvalidChecksum,
}

impl Display for SteuerIdCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SteuerIdCheckError::ZeroAtStart => "0 als erste Ziffer".into(),
                SteuerIdCheckError::InvalidDigit(pos) =>
                    format!("Ziffer {0} ist nicht in 0..9", pos + 1),
                SteuerIdCheckError::InvalidNumbers => "Ungültige Nummernfolge".into(),
                SteuerIdCheckError::InvalidChecksum => "Ungültige Prüfziffer".into(),
            }
        )
    }
}

pub struct SteuerId(pub [u8; 11]);

impl Display for SteuerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // war bis jetzt die schnellste Implementierung, die sehr wahrscheinlich auch am
        // wenigsten memory impact hat (u8.to_string() reserviert jedesmal einen String mit
        // capacity 3 wg. utf-8, aber das brauchen wir hier nicht. und wir reservieren auch
        // nur einmal.)
        let mut repr = String::with_capacity(11);
        for d in self.0 {
            repr.push((d + 48) as char);
        }
        write!(f, "{}", repr)
    }
}

impl SteuerId {
    pub fn try_create(digits: [u8; 11]) -> Result<Self, SteuerIdCheckError> {
        let result = Self(digits);
        result.check().map(|_| result)
    }

    pub fn create_random() -> Self {
        let mut rng = rand::thread_rng();
        let mut digits = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0];
        let i1 = rng.gen_range(0..10usize);
        let i2 = loop {
            let i = rng.gen_range(0..10usize);
            if i != i1 {
                break i;
            }
        } as usize;
        digits[i1] = digits[i2];
        loop {
            digits[..10].shuffle(&mut rng);
            if digits[0] != 0 {
                break;
            }
        }
        digits[10] = calc_pz(&digits[..10]);

        Self(digits)
    }

    fn check(&self) -> Result<(), SteuerIdCheckError> {
        self.0
            .iter()
            .take(10)
            .enumerate()
            .fold(Ok([0usize; 10]), |a, (i, &d)| match a {
                Err(e) => Err(e),
                Ok(mut a) => match d {
                    0 if i == 0 => Err(SteuerIdCheckError::ZeroAtStart),
                    10.. => Err(SteuerIdCheckError::InvalidDigit(i)),
                    _ => match a[d as usize] {
                        3.. => Err(SteuerIdCheckError::InvalidNumbers),
                        _ => {
                            a[d as usize] += 1;
                            Ok(a)
                        }
                    },
                },
            })
            .and_then(|ca| {
                match ca.iter().fold([0usize; 4], |mut ca, &c| {
                    ca[c] += 1;
                    ca
                }) {
                    [1, 8, 1, 0] => Ok(()),
                    [2, 7, 0, 1] => Ok(()),
                    _ => Err(SteuerIdCheckError::InvalidNumbers),
                }
            })
            .and_then(|_| {
                let pz = self.0[10];
                if pz == calc_pz(&self.0[..11]) {
                    Ok(())
                } else {
                    Err(SteuerIdCheckError::InvalidChecksum)
                }
            })
    }
}

fn calc_pz(digits: &[u8]) -> u8 {
    let product = digits.iter().take(10).fold(10u8, |p, d| {
        let sum = match (d + p) % 10u8 {
            0 => 10u8,
            n => n,
        };
        (sum * 2) % 11
    });
    match 11 - product {
        10 => 0,
        n => n,
    }
}

#[cfg(test)]
mod test_model {
    use super::*;

    #[test]
    fn test_create_valid_one_replaced() {
        let id = SteuerId::try_create([1, 0, 3, 7, 4, 9, 1, 8, 2, 5, 8]);
        assert!(id.is_ok());
        let id = id.unwrap();
        assert_eq!(id.to_string(), "10374918258".to_owned());
    }

    #[test]
    fn test_create_valid_two_replaced() {
        let id = SteuerId::try_create([1, 0, 3, 7, 1, 9, 1, 8, 2, 5, 7]);
        assert!(id.is_ok());
        let id = id.unwrap();
        assert_eq!(id.to_string(), "10371918257".to_owned());
    }

    #[test]
    fn test_invalid_zero_at_start() {
        let id = SteuerId::try_create([0, 1, 3, 7, 4, 9, 1, 8, 2, 5, 7]);
        assert!(matches!(id, Err(SteuerIdCheckError::ZeroAtStart)));
    }

    #[test]
    fn test_invalid_checksum() {
        let id = SteuerId::try_create([1, 0, 3, 7, 4, 9, 1, 8, 2, 5, 7]);
        assert!(matches!(id, Err(SteuerIdCheckError::InvalidChecksum)));
    }

    #[test]
    fn test_invalid_digits() {
        let id = SteuerId::try_create([1, 13, 3, 7, 4, 8, 1, 5, 2, 6, 8]);
        assert!(matches!(id, Err(SteuerIdCheckError::InvalidDigit(1))));
    }

    #[test]
    fn test_invalid_numbers_all_different() {
        let id = SteuerId::try_create([1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1]);
        assert!(matches!(id, Err(SteuerIdCheckError::InvalidNumbers)));
    }

    #[test]
    fn test_invalid_numbers_two_and_two() {
        let id = SteuerId::try_create([1, 2, 1, 2, 5, 6, 7, 8, 9, 0, 1]);
        assert!(matches!(id, Err(SteuerIdCheckError::InvalidNumbers)));
    }

    #[test]
    fn test_invalid_numbers_more_than_three() {
        let id = SteuerId::try_create([1, 1, 1, 2, 5, 6, 7, 1, 9, 0, 1]);
        assert!(matches!(id, Err(SteuerIdCheckError::InvalidNumbers)));
    }
}
