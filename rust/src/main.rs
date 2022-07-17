use std::sync::mpsc::channel;
use std::time::Instant;

use clap::Parser;
use rand::{seq::SliceRandom, Rng};
use threadpool::ThreadPool;

#[derive(Parser)]
struct Args {
    #[clap(short, long, value_parser, default_value_t = String::from("solved"))]
    version: String,

    #[clap(short, long, action, default_value_t = false)]
    optimized: bool,

    #[clap(short, long, value_parser, default_value_t = 100)]
    prisoners: usize,

    #[clap(short, long, value_parser, default_value_t = 50)]
    chances: usize,

    #[clap(short, long, value_parser, default_value_t = 1_000_000)]
    iterations: usize,
}

fn _allocate_boxes(boxes: &mut Vec<usize>) {
    let mut rng = rand::thread_rng();

    // There are `count` numbered slips and `count` numbered boxes, one for each
    // prisoner, and each slip is randomly placed in a box.
    for (idx, empty_box) in boxes.iter_mut().enumerate() {
        *empty_box = idx;
    }

    // Distrubte the slips randomly amongst the boxes.
    boxes.shuffle(&mut rng);
}

/// There are 100 prisoners. They are given an opportunity to be released. The
/// conditions of this release are as follows:
///  - each prisoner is assigned a unique number, and this number is written on a
///    slip of paper
///  - in a room, there are 100 boxes, and those slips are distributed amongst the
///    boxes randomly
///  - each prisoner must go into the room alone, and can open 50 boxes -- if they
///    find the box with their number in it, they are a winner
///  - if all 100 prisoners each find the box with their number in it, all of the
///    prisoners are freed -- but if even one fails, they are all executed
///  - the prisoners are allowed to coordinate a strategy before the game begins
///
/// The premise of the solution is that each prisoner should start by opening the
/// box with their number on it. If that does not contain the slip with their
/// number, they are to open the box with the same number as that slip. They are to
/// repeat the exercise until they find their number. These chains of numbers will
/// be called "loops" -- given that each slip's number is unique and each box's
/// number is unique, there will necessarily be one way to start with a given box
/// and end up with the box containing a slip with the initial box's number. For
/// example, given a setup with only five numbers:
///
///        0   1   2   3   4   5   6   7   8   9
///       +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+
///       |4| |3| |9| |2| |7| |8| |6| |5| |0| |1|
///       +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+
///
///       In this case, let's start with prisoner #1. This prisoner will
///       go to box #1 first. They will then go to box #3, then #2, #9,
///       and then they will have found their number. There are three total
///       loops in this set:
///
///           0 -> 4 -> 7 -> 5 -> 8
///           ^                   |
///           +-------------------+
///
///           1 -> 3 -> 2 -> 9
///           ^              |
///           +--------------+
///
///           6 -+
///           ^  |
///           +--+
///
/// In just over 31% of cases, all prisoners will be able to find their number
/// without needing to open more than 50 boxes. In the other ~69% of cases more
/// than half of the prisoners will not be able to find their number.
///
/// This function builds 100 slips and boxes, both numbered 0 to 99, and 100
/// prisoners, each with a flag indicating whether or not they've found their slip.
/// Then, using a random number generator, each slip is assigned to a random box,
/// with no slip getting the same box. Finally, the prisoners are iterated through,
/// and each one gets fifty tries to find their slip, by starting with the box
/// corresponding to their number, as described above.
#[allow(unused)]
fn run(boxes: &Vec<usize>, count: usize, chances: usize) -> bool {
    let mut prisoners: Vec<bool> = vec![false; count];

    for (prisoner, found) in prisoners.iter_mut().enumerate() {
        let mut next_box: usize = prisoner;

        for _ in 0..chances {
            let slip = boxes[next_box];

            match slip == prisoner {
                true => {
                    *found = true;
                    break;
                },
                false => next_box = slip,
            }
        }
    }

    prisoners.into_iter().all(|found| found)
}

/// This version of the solution has two optimizations. The first is that if any of the
/// prisoners open fifty boxes without success, the function exits early. Additionally,
/// any previously seen slip is cached -- if the slip has been seen by a previous
/// prisoner, and the function didn't exit early, that means that the slip is
/// necessarily in a loop that does not contain more than fifty boxes.
#[allow(unused)]
fn run_optimized(boxes: &Vec<usize>, count: usize, chances: usize) -> bool {
    let mut slips_seen: Vec<bool> = vec![false; count];

    for prisoner in 0..count {
        let mut next_box: usize = prisoner;

        if slips_seen[prisoner] == true {
            continue;
        }

        for idx in 0..=chances {
            if idx == chances {
                // We are on the 51st iteration of this search. This means that there
                // is at least one loop with greater than 50 items in it, which means
                // that the premise of the exercise cannot be met.
                return false;
            }

            let slip = boxes[next_box];

            slips_seen[slip] = true;
            match slip == prisoner {
                true => break,
                false => next_box = slip,
            }
        }
    }

    true
}

/// The below function is the naive approach to the problem. Each of the prisoners picks
/// a random box to open. They have 50 attempts to pick the box with their number in it.
#[allow(unused)]
fn run_naive(boxes: &Vec<usize>, count: usize, chances: usize) -> bool {
    let mut rng = rand::thread_rng();

    let mut prisoners: Vec<bool> = vec![false; count];
    let mut opened_boxes: Vec<bool> = prisoners.clone();

    for (prisoner, found) in prisoners.iter_mut().enumerate() {
        for _ in 0..chances {
            let mut to_open: usize;

            loop {
                to_open = rng.gen_range(0..count);

                if !opened_boxes[to_open] {
                    opened_boxes[to_open] = true;
                    break;
                }
            }

            if boxes[to_open] == prisoner {
                *found = true;
                break;
            }
        }

        opened_boxes.fill(false);
    }

    prisoners.into_iter().all(|found| found)
}

/// The below function is an optimized version of the naive logic.
#[allow(unused)]
fn run_naive_optimized(boxes: &Vec<usize>, count: usize, chances: usize) -> bool {
    let mut rng = rand::thread_rng();

    let mut to_open: Vec<usize> = boxes.clone();

    for prisoner in 0..count {
        to_open.shuffle(&mut rng);

        for idx in 0..=chances {
            if idx == chances {
                // No need to continue -- one prisoner has failed, so they all have.
                return false;
            }

            if to_open[idx] == prisoner {
                break;
            }
        }
    }

    true
}

fn main() {
    let threads: usize = 16;
    let pool = ThreadPool::new(16);
    let (tx, rx) = channel();

    let args = Args::parse();

    let handler = match (&args.version[..], args.optimized) {
        ("naive", false) => run_naive,
        ("naive", true) => run_naive_optimized,
        (_, false) => run,
        (_, true) => run_optimized,
    };

    let start = Instant::now();

    for i in 0..threads {
        let tx = tx.clone();
        let to_execute = match i + 1 == threads {
            true => (args.iterations / threads) + (args.iterations % threads),
            false => args.iterations / threads,
        };

        pool.execute(move || {
            let mut wins: u32 = 0;
            let mut boxes: Vec<usize> = (0..args.prisoners).collect();

            for _ in 0..to_execute {
                _allocate_boxes(&mut boxes);
                wins += handler(&boxes, args.prisoners, args.chances) as u32;
            }

            tx.send(wins).unwrap();
        });
    }

    let wins: u32 = rx.iter().take(threads as usize).fold(0, |a, b| a + b);

    let finished = start.elapsed();

    println!(
        "complete in {:.3} seconds! of {} runs, {} were successful ({:.2}%)",
        finished.as_millis() as f32 / 1000 as f32,
        args.iterations,
        wins,
        (wins as f32 / args.iterations as f32) * 100.0,
    );
}
