use rand::Rng;
use rand::distr::{Distribution, Uniform};

pub fn random_value() -> String {
    let mut rng = rand::rng();

    match rng.random_range(0u8..100) {
        0..=19 => {
            // int — 20%
            format!("{}", rng.random_range(-1_000_000..=1_000_000))
        }
        20..=39 => {
            // float — 20%
            format!("{:.4}", rng.random_range(-1000.0f64..1000.0))
        }
        40..=69 => {
            // string — 30%
            let len = rng.random_range(5..=30);
            random_alpha(len, &mut rng)
        }
        70..=79 => {
            // date — 10%
            let year = rng.random_range(2000u16..=2040);
            let month = rng.random_range(1u8..=12);
            let day = rng.random_range(1u8..=28);
            format!("{year:04}-{month:02}-{day:02}")
        }
        80..=89 => {
            // bool — 10%
            if rng.random_bool(0.5) {
                "TRUE".into()
            } else {
                "FALSE".into()
            }
        }
        _ => {
            // empty — 10%
            String::new()
        }
    }
}

fn random_alpha(len: usize, rng: &mut impl Rng) -> String {
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let dist = Uniform::new(0, CHARS.len()).unwrap();
    (0..len).map(|_| CHARS[dist.sample(rng)] as char).collect()
}
