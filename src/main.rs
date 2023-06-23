use rand_chacha::rand_core::SeedableRng;
use rand_chacha::rand_core::RngCore;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(81094);
    let random_nums: Vec<u32> = (0..16384).map(|_| rng.next_u32()).collect();

    let res_mod_256: Vec<u8> = random_nums.iter().map(|x| mod256(*x)).collect();
    let res_fold: Vec<u8> = random_nums.iter().map(|x| fold(*x)).collect();
    let res_other_fold: Vec<u8> = random_nums.iter().map(|x| other_fold(*x)).collect();

    let hist_mod_256 = gen_histogram(res_mod_256);
    let hist_fold = gen_histogram(res_fold);
    let hist_other_fold = gen_histogram(res_other_fold);

    write_to_file(hist_mod_256, "hist_mod_256.txt");
    write_to_file(hist_fold, "hist_fold.txt");
    write_to_file(hist_other_fold, "hist_other_fold.txt");
}

fn mod256(x: u32) -> u8 { (x % 256) as u8 }

fn fold(x: u32) -> u8 {
    let mut b:[u8; 4] = [0; 4];
    for i in 0..4 {
        b[i] = ((x >> (i * 8))) as u8;
    }

    b[0] ^ b[1] ^ b[2] ^ b[3]
}

fn other_fold(x: u32) -> u8 {
    let mut b:[u8; 4] = [0; 4];
    for i in 0..4 {
        b[i] = ((x >> (i * 8))) as u8;
    }

    (b[0].wrapping_add(b[2])) ^ (b[1].wrapping_add(b[3]))
}

fn gen_histogram(data: Vec<u8>) -> Vec<u32> {
    let mut hist: Vec<u32> = vec![0; 256];
    for i in data {
        hist[i as usize] += 1;
    }
    hist
}

fn write_to_file(data: Vec<u32>, filename: &str) {
    let mut file = File::create(filename).unwrap();
    for (i, val) in data.iter().enumerate() {
        file.write_all(i.to_string().as_bytes()).unwrap();
        file.write_all(" ".as_bytes()).unwrap();
        file.write_all(val.to_string().as_bytes()).unwrap();
        file.write_all("\n".as_bytes()).unwrap();
    }
}
