// l65_random_walk
// Scientific American puzzle: https://www.scientificamerican.com/game/math-puzzle-wanderers-return/
//
// 2025-06-30   PV

// A wanderer lives in a remote landscape with three villages: Avalon, Belthar and Cresthaven. Every morning, they leave
// the village they are in and travel to one of the other two villages, choosing between the two destinations at random
// with an equal 50–50 chance. If the wanderer begins in their home village of Avalon and stops after 100 days of
// travel, are they more likely to end up in Avalon or in Belthar? Or are both destinations equally likely?


// Gemini answer:
// The Logic Behind the Puzzle 🤔
// 
// Let Pn​(A) be the probability that the wanderer is in Avalon on day n.
//     On Day 0, the wanderer is in Avalon, so P0​(A)=1.
//     On Day 1, they must leave Avalon, so P1​(A)=0.
//     On Day 2, they can return to Avalon from either Belthar or Cresthaven. The probability of doing so is P2​(A)=0.5.
// 
// To be in Avalon on any given day, the wanderer must have been in Belthar or Cresthaven the day before. The total
// probability of being in Belthar or Cresthaven is simply 1−Pn​(A). Since the choice is always 50/50, the probability
// of returning to Avalon is half of that.
// 
// This gives us the recurrence relation: Pn+1​(A) = (1−Pn​(A)​)/2
// 
// This formula shows that the probability of being in Avalon oscillates around the equilibrium state of 1/3.
//     For even days (like Day 100), the probability Pn​(A) is always slightly greater than 1/3.
//     For odd days, the probability Pn​(A) is always slightly less than 1/3.
// 
// Since the remaining probability is split evenly between Belthar and Cresthaven, their individual probabilities will
// be slightly less than 1/3 on an even day. Therefore, on Day 100, the wanderer is most likely to be in Avalon.

#![allow(unused)]

use rand::rngs::SmallRng;
use rand::{Rng, RngExt, SeedableRng};
use std::time::{SystemTime, UNIX_EPOCH};

// Define the three villages for clarity and type safety.
#[derive(Debug, Clone, Copy)]
enum Village {
    Avalon,
    Belthar,
    Cresthaven,
}

// --- Constants for the simulation ---
const NUM_SIMULATIONS: u32 = 100_000;
const NUM_DAYS: u32 = 100;
const RANDOM_SEED:u64 = 2;      // 0 = use time as seed, <>0 = use this specific seed

fn main() {
    // Initialize a random number generator.
    let mut rng: SmallRng = if RANDOM_SEED != 0 {
        SmallRng::seed_from_u64(RANDOM_SEED)
    } else {
        let time_seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        SmallRng::seed_from_u64(time_seed)
    };

    // Counters for final destinations.
    let mut avalon_ends = 0;
    let mut belthar_ends = 0;
    let mut cresthaven_ends = 0;

    println!(
        "Running {} simulations for a {}-day journey...",
        NUM_SIMULATIONS, NUM_DAYS
    );

    // Main simulation loop.
    for _ in 0..NUM_SIMULATIONS {
        // Each journey starts in Avalon.
        let mut current_village = Village::Avalon;

        // Simulate the 100 days of travel.
        for _ in 0..NUM_DAYS {
            // Determine the two possible destinations.
            let destinations = match current_village {
                Village::Avalon => [Village::Belthar, Village::Cresthaven],
                Village::Belthar => [Village::Avalon, Village::Cresthaven],
                Village::Cresthaven => [Village::Avalon, Village::Belthar],
            };

            // Randomly choose one of the two destinations (50/50 chance).
            if rng.random_bool(0.5) {
                current_village = destinations[0];
            } else {
                current_village = destinations[1];
            }
        }

        // After 100 days, record the final village.
        match current_village {
            Village::Avalon => avalon_ends += 1,
            Village::Belthar => belthar_ends += 1,
            Village::Cresthaven => cresthaven_ends += 1,
        }
    }

    // --- Calculate and display the final percentages ---
    let total = NUM_SIMULATIONS as f64;
    let avalon_percent = (avalon_ends as f64 / total) * 100.0;
    let belthar_percent = (belthar_ends as f64 / total) * 100.0;
    let cresthaven_percent = (cresthaven_ends as f64 / total) * 100.0;

    println!("\n--- Simulation Results ---");
    println!("Ended in Avalon:     {:.2}% ({} times)", avalon_percent, avalon_ends);
    println!("Ended in Belthar:    {:.2}% ({} times)", belthar_percent, belthar_ends);
    println!("Ended in Cresthaven: {:.2}% ({} times)", cresthaven_percent, cresthaven_ends);
}
