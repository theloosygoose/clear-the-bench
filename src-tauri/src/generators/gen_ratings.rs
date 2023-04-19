use rand::Rng;
use rand_distr::{Normal};

pub fn generate_rating(mean: f32, std_dev: f32) -> u16 {

    let mut rng = rand::thread_rng();
    let normal= Normal::new(mean, std_dev).unwrap();

    let mut rating = rng.sample(normal).abs();

    while rating > 100.00 || rating < 0.0{
        rating = rng.sample(normal).abs().round();
    }

    return rating.round() as u16;
}

pub fn print_many_ratings(mean:f32, std_dev:f32, athleticism_std_dev:f32, gen_amt:u16){
    let mut log_height = vec![];
    let mut log_athleticism_mean = vec![];
    let mut n = 0;

    while n < gen_amt {
        let height_rating = generate_rating(mean, std_dev);
        log_height.push(height_rating);
        log_athleticism_mean.push(generate_rating((100-height_rating) as f32, athleticism_std_dev));
        n += 1;
    }

    println!("HEIGHT::{:?}", log_height);
    println!("ATHLETICISM::{:?}", log_athleticism_mean);
}
