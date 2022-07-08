use std::sync::mpsc::channel;

use rand::Rng;
use threadpool::ThreadPool;

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
/// corresponding to their number, as described above. If any prisoner opens fifty
/// boxes without success, the function exits early, returning false. Otherwise,
/// all prisoners have necessarily found their number.
fn run() -> bool {
    let mut rng = rand::thread_rng();

    // There are 100 numbered slips, one for each prisoner -- each slip corresponds
    // to a box.
    let mut slips: Vec<Option<usize>> = (0..100).map(|_| None).collect();
    // There are 100 numbered boxes, one for each slip -- each box contains a slip.
    let mut boxes: Vec<Option<usize>> = slips.clone();
    // There are 100 prisoners, and whether or not they've found their slip.
    let mut prisoners: Vec<bool> = (0..100).map(|_| false).collect();

    for slip in 0..100 {
        let mut slip_box: usize;

        loop {
            slip_box = rng.gen_range(0..100);

            match boxes[slip_box as usize] {
                Some(_) => continue,
                None => {
                    slips[slip] = Some(slip_box);
                    boxes[slip_box] = Some(slip);
                    break;
                }
            };
        }
    }

    for (prisoner, found) in prisoners.iter_mut().enumerate() {
        let mut next_box: usize = prisoner;

        for idx in 0..=50 {
            if idx == 50 {
                // We are on the 51st iteration of this search. This means that there
                // is at least one loop with greater than 50 items in it, which means
                // that the premise of the exercise cannot be met.
                return false;
            }

            match boxes[next_box] {
                Some(slip) => {
                    match slip == prisoner {
                        true => {
                            *found = true;
                            break;
                        },
                        false => {
                            next_box = slip;
                        }
                    }
                },
                _ => {},
            }
        }
    }

    prisoners.iter().find(|found| **found == false).is_none()
}

fn main() {
    let pool = ThreadPool::new(16);
    let (tx, rx) = channel();
    let runs: u32 = 10_000_000;

    for _ in 0..runs {
        let tx = tx.clone();

        pool.execute(move || tx.send(run() as u32).unwrap());
    }

    let wins: u32 = rx.iter().take(runs as usize).fold(0, |a, b| a + b);

    println!(
        "complete! of {:} runs, {:} were successful ({:.2}%)",
        runs,
        wins,
        (wins as f32 / runs as f32) * 100.0,
    );
}
