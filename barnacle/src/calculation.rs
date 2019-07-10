use data_types::*;
use rand::distributions::{Uniform, Distribution};

pub fn monte_carlo(play: &Play) -> MonteCarloResult {
    let mut round_outcomes: Vec<f64> = vec![];
    let iterations = 100_000;
    for _round in 1..iterations {
        let event_prob = play.event.prob.unwrap().sample(&mut rand::thread_rng());
        let mut rng = rand::thread_rng();
        let simulation_rand = Uniform::new_inclusive(0.0, 1.0);
        let mut simulation_total: f64 = 0.0;
        for _each_event in 1..event_prob.round() as i64 {
            let condition_prob = get_condition_rand(&play.scenario);
            if simulation_rand.sample(&mut rng) <= condition_prob {
                let magnitude_prob = play.magnitude_prob.unwrap()
                                                        .sample(&mut rand::thread_rng());
                let cost_prob = get_cost_rand(&play.costs);
                let simulation_outcome = magnitude_prob * cost_prob;
                simulation_total += simulation_outcome;
            }
        }
        if simulation_total > 0.0 {
            round_outcomes.push(simulation_total);
        }    
    }

    round_outcomes.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let outcomes_length = round_outcomes.len();
    let annual_loss_event_prob = outcomes_length as f64 / iterations as f64 * 100.0; 
    let fifth_percentile = round_outcomes[(outcomes_length as f64 * 0.05) as usize];
    let ninety_fifth_percentile = round_outcomes[(outcomes_length as f64 * 0.95) as usize];
    let median = round_outcomes[(outcomes_length as f64 / 2.0) as usize];
    let mean = mean(&round_outcomes).unwrap();
    let std_dev = std_deviation(&round_outcomes).unwrap();

    MonteCarloResult {
        description: play.description.clone(),
        annual_loss_event_prob,
        fifth_percentile,
        ninety_fifth_percentile,
        mean,
        median,
        std_dev,
    }
}

// From https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html
fn mean(data: &[f64]) -> Option<f64> {
    let sum = data.iter().sum::<f64>();
    let count = data.len();
    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

// From https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html
fn std_deviation(data: &[f64]) -> Option<f64> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - (*value as f64);

                    diff * diff
                })
                .sum::<f64>()
                / count as f64;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

/// Calculates total probability from all of the conditions in a play
fn get_condition_rand(entries: &[Entry]) -> f64 {
    let mut running_total: f64 = 1.0;
    for entry in entries {
        match entry {
            Entry::Single(condition) => {
                running_total *= 0.01 * condition.prob.unwrap().sample(&mut rand::thread_rng());
            }
            Entry::Branch(leaves) => {
                let mut leaves_total = 0.0;
                for leaf in leaves {
                    let modifier = leaf.weight * 0.01;
                    let sub = get_condition_rand(&leaf.scenario);
                    leaves_total += modifier * sub;
                }
                running_total *= leaves_total;
            }
        }
    }
    running_total
}

fn get_cost_rand(costs: &[Cost]) -> f64 {
    let mut running_total: f64 = 1.0;
    for cost in costs {
        running_total += cost.prob.unwrap().sample(&mut rand::thread_rng());
    }
    running_total
}
