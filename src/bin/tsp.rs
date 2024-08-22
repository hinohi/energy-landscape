use clap::Parser;
use rand::Rng;
use rand_pcg::Mcg128Xsl64;

#[derive(Parser)]
struct Args {
    #[clap(short)]
    n: usize,
    #[clap(short, long)]
    size: f64,
    #[clap(long, default_value = "1")]
    seed: u128,
}

pub trait LexicalPermutation {
    fn next_permutation(&mut self) -> bool;
}

impl<T> LexicalPermutation for [T]
where
    T: Ord,
{
    fn next_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 {
            return false;
        }

        // Step 1: Identify the longest, rightmost weakly decreasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the last-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Find the rightmost element larger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i - 1] {
            j -= 1;
        }

        // Step 3: Swap that element with the pivot
        self.swap(j, i - 1);

        // Step 4: Reverse the (previously) weakly decreasing part
        self[i..].reverse();

        true
    }
}

fn make_town(rng: &mut Mcg128Xsl64, n: usize, size: f64) -> Vec<[f64; 2]> {
    let mut towns = Vec::with_capacity(n);
    for _ in 0..n {
        towns.push([rng.gen_range(0.0..size), rng.gen_range(0.0..size)]);
    }
    towns
}

fn make_dist_mat(towns: &[[f64; 2]]) -> Vec<Vec<f64>> {
    let mut dist_mat = Vec::with_capacity(towns.len());
    for a in towns {
        dist_mat.push(
            towns
                .iter()
                .map(|b| {
                    let x = a[0] - b[0];
                    let y = a[1] - b[1];
                    x.hypot(y)
                })
                .collect(),
        );
    }
    dist_mat
}

fn calc_tour_length(dist_mat: &[Vec<f64>], tour: &[usize]) -> f64 {
    let mut sum = dist_mat[0][tour[0]];
    for i in 0..tour.len() - 1 {
        sum += dist_mat[tour[i]][tour[i + 1]];
    }
    sum + dist_mat[tour.len() - 1][0]
}

fn tour_as_key(tour: &[usize]) -> u64 {
    let mut key = 0;
    for t in tour {
        key <<= 4;
        key += (*t - 1) as u64;
    }
    key
}

fn main() {
    let args = Args::parse();
    let mut rng = Mcg128Xsl64::new(args.seed);
    let towns = make_town(&mut rng, args.n, args.size);
    let dist_mat = make_dist_mat(&towns);
    let mut tour = (1..args.n).collect::<Vec<_>>();
    loop {
        println!(
            "{} {}",
            tour_as_key(&tour),
            calc_tour_length(&dist_mat, &tour)
        );
        if !tour.next_permutation() {
            break;
        }
    }
}
