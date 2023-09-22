use rand::{SeedableRng, Rng, seq::SliceRandom};


pub fn rnd_two_of_vec<'a, T>(
    slice: &'a mut [T],
    seed: &mut (impl SeedableRng + Rng),
) -> [&'a T; 2] {
    slice.shuffle(seed);
    [&slice[0], &slice[1]]
}

pub fn rnd_one_of_vec<'a, T>(
    slice: &'a mut [T],
    seed: &mut (impl SeedableRng + Rng),
) -> &'a T {
    slice.shuffle(seed);
    &slice[0]
}