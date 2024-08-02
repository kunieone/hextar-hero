use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub struct Seed {
    pub root_seed: u64,
    pub step: u128,
    pub rng: ChaCha8Rng,
}

impl Seed {
    pub fn new(seed: Option<u64>) -> Self {
        let seed = seed.unwrap_or_else(|| rand::thread_rng().next_u64());
        Self {
            root_seed: seed,
            step: 0,
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }
    pub fn from_string(seed_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if seed_str.len() != 12 {
            return Err("Seed string must be 12 characters long".into());
        }
        let seed = u64::from_str_radix(seed_str, 36)?;
        Ok(Self::new(Some(seed)))
    }

    pub fn next_id(&mut self) -> u64 {
        self.step += 1;
        self.rng.next_u64()
    }
}

#[test]
fn bench_test() {
    let mut seed = Seed::new(Some(12345));
    let iterations = 1_000_000;

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        println!("{}", seed.next_id());
    }
    let duration = start.elapsed();

    println!("Generated {} random IDs in {:?}", iterations, duration);
    println!(
        "Average time per random ID generation: {:?}",
        duration / iterations as u32
    );
}
