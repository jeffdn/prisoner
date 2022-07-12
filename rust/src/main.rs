use std::sync::mpsc::channel;

use rand::Rng;
use threadpool::ThreadPool;

fn _allocate_boxes(count: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();

    // There are `count` numbered slips and `count` numbered boxes, one for each
    // prisoner, and each slip is randomly placed in a box.
    let mut boxes: Vec<Option<usize>> = (0..count).map(|_| None).collect();

    for slip in 0..count {
        let mut slip_box: usize;

        loop {
            slip_box = rng.gen_range(0..count);

            match boxes[slip_box as usize] {
                Some(_) => continue,
                None => {
                    boxes[slip_box] = Some(slip);
                    break;
                }
            };
        }
    }

    boxes.into_iter().map(|slip| slip.unwrap()).collect()
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
fn run(count: usize) -> bool {
    let chances = count / 2;
    let boxes = _allocate_boxes(count);
    let mut prisoners: Vec<bool> = (0..count).map(|_| false).collect();

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

    prisoners.into_iter().find(|found| *found == false).is_none()
}

/// This version of the solution has two optimizations. The first is that if any of the
/// prisoners open fifty boxes without success, the function exits early. Additionally,
/// any previously seen slip is cached -- if the slip has been seen by a previous
/// prisoner, and the function didn't exit early, that means that the slip is
/// necessarily in a loop that does not contain more than fifty boxes.
#[allow(unused)]
fn run_optimized(count: usize) -> bool {
    let chances: usize = count / 2;
    let boxes = _allocate_boxes(count);
    let mut slips_seen: Vec<bool> = (0..count).map(|_| false).collect();

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
fn run_naive(count: usize) -> bool {
    let mut rng = rand::thread_rng();

    let chances = count / 2;
    let boxes = _allocate_boxes(count);
    let mut prisoners: Vec<bool> = (0..count).map(|_| false).collect();
    let _opened_boxes: Vec<bool> = prisoners.clone();

    for (prisoner, found) in prisoners.iter_mut().enumerate() {
        let mut opened_boxes = _opened_boxes.clone();

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
    }

    prisoners.iter().find(|found| **found == false).is_none()
}

fn main() {
    let pool = ThreadPool::new(16);
    let (tx, rx) = channel();
    let runs: u32 = 1_000_000;

    for _ in 0..runs {
        let tx = tx.clone();

        pool.execute(move || tx.send(run_naive(100) as u32).unwrap());
    }

    let wins: u32 = rx.iter().take(runs as usize).fold(0, |a, b| a + b);

    println!(
        "complete! of {:} runs, {:} were successful ({:.2}%)",
        runs,
        wins,
        (wins as f32 / runs as f32) * 100.0,
    );
}
