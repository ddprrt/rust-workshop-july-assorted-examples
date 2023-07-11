use std::{sync::Arc, thread};

use rand::seq::SliceRandom;

fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    vec.push(6);
    let vec_ref = &mut vec;
    vec_ref.push(7);
    println!("{:?}", vec_ref);
    println!("{:?}", vec);

    let shared_ref = &vec;
    println!("{:?}", shared_ref);

    let vec = vec![1, 2, 3, 4, 5];
    let vec = Arc::new(vec);
    // Mara Bos: Atomics and Locks
    let mut handles = Vec::new();
    for i in 0..5 {
        let vec = vec.clone();
        let handle = thread::spawn(move || {
            println!("{}: {:?}", i, vec);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut pot: Vec<u32> = (1..=50).collect();
    let mut rng = rand::thread_rng();
    pot.shuffle(&mut rng);
    let results: Vec<u32> = pot.into_iter().take(5).collect();
    println!("{:?}", results);
    let mut lotto = Lotto::new(50, &mut rng);
    println!("{:?}", lotto);
    let results = lotto.take(5);
    println!("{:?}", results);

    let mut lotto2 = Lotto::new(50, &mut rng);
    let results = lotto2.take(5);
    println!("{:?}", results);
}

#[derive(Debug)]
struct Lotto<'a> {
    pot: Vec<u32>,
    rng: &'a mut rand::rngs::ThreadRng,
}

impl<'a> Lotto<'a> {
    fn new(pot_size: u32, rng: &'a mut rand::rngs::ThreadRng) -> Self {
        Self {
            pot: (1..=pot_size).collect(),
            rng,
        }
    }

    fn take(&mut self, amount: usize) -> Vec<u32> {
        let mut pot = self.pot.clone();
        pot.shuffle(&mut self.rng);
        pot.into_iter().take(amount).collect()
    }
}
