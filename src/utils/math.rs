use rand::Rng;

/// Generate a random number in a range, inclusive of the ceiling.
pub fn random_num(min: usize, max: usize) -> usize {
    let result: usize = rand::thread_rng().gen_range(min..=max);

    result
}

mod tests {
    #[test]
    fn rand_nums_out_of_bounds() {
        let loops: usize = 500;
        let mut numbers: Vec<usize> = vec![];

        fn looper(numbers: &mut Vec<usize>, loops: usize) {
            let min: usize = 0;
            let max: usize = loops;

            let rand = super::random_num(min, max);

            if rand < min || rand > max {
                panic!("The random number generator went out of bounds.")
            }

            numbers.push(rand);

            if loops == 0 {
                return;
            }

            looper(numbers, loops - 1)
        }

        looper(&mut numbers, loops);
        println!("{:?}", numbers);
    }
}
