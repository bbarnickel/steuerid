use std::fmt::Display;

use rand::{Rng, prelude::SliceRandom};

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
    pub fn try_create(digits: [u8; 11]) -> Result<Self, String> {
        let result = Self(digits);
        if result.check() {
            Ok(result)
        } else {
            Err("Prüfziffer stimmt nicht!".into())
        }
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

    fn check(&self) -> bool {
        let calc_pz = calc_pz(&self.0[..11]);
        let pz = self.0[10];
        calc_pz == pz
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
        n => n
    }
}

#[cfg(test)]
mod test_model {
    use super::*;

    #[test]
    fn test_create_valid() {
        let id = SteuerId::try_create([1, 0, 3, 7, 4, 9, 1, 8, 2, 5, 8]);
        assert!(id.is_ok());
        let id = id.unwrap();
        assert_eq!(id.to_string(), "10374918258".to_owned());
    }

    #[test]
    fn test_invalid() {
        let id = SteuerId::try_create([1, 0, 3, 7, 4, 9, 1, 8, 2, 5, 7]);
        assert!(id.is_err());
    }
}