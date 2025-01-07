use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Wild,
    Skip,
}

#[derive(Debug, Clone)]
enum Value {
    Number(u8), // 1-12
    Wild,
    Skip,
}

#[derive(Debug, Clone)]
struct Card {
    color: Color,
    value: Value,
}

const NUM_WILD_CARDS: usize = 8;
const NUM_SKIP_CARDS: usize = 4;
const HAND_SIZE: usize = 10;
const NUM_HANDS: usize = 4;

fn generate_deck() -> Vec<Card> {
    let mut deck = Vec::new();

    // Add number cards (2 of each per color)
    for &color in &[Color::Red, Color::Blue, Color::Green, Color::Yellow] {
        for value in 1..=12 {
            deck.push(Card {
                color: color,
                value: Value::Number(value),
            });
            deck.push(Card {
                color: color,
                value: Value::Number(value),
            });
        }
    }

    // Add Wild and Skip cards
    for _ in 0..NUM_WILD_CARDS {
        deck.push(Card {
            color: Color::Wild,
            value: Value::Wild,
        });
    }

    for _ in 0..NUM_SKIP_CARDS {
        deck.push(Card {
            color: Color::Skip,
            value: Value::Skip,
        });
    }

    deck
}

fn shuffle_deck(deck: &mut Vec<Card>, times: usize) {
    let mut rng = thread_rng();
    for _ in 0..times {
        deck.shuffle(&mut rng);
    }
}

fn riffle_shuffle(deck: &mut Vec<Card>) {
    let mut rng = thread_rng();
    let mid = rng.gen_range(deck.len() / 2 - 2..=deck.len() / 2 + 2);
    let (left, right) = deck.split_at(mid);
    let mut shuffled = Vec::with_capacity(deck.len());

    let mut left_iter = left.iter();
    let mut right_iter = right.iter();

    while left_iter.len() > 0 || right_iter.len() > 0 {
        let take_from_left = rng.gen_range(1..=3);
        for _ in 0..take_from_left {
            if let Some(l) = left_iter.next() {
                shuffled.push(l.clone());
            }
        }

        let take_from_right = rng.gen_range(1..=3);
        for _ in 0..take_from_right {
            if let Some(r) = right_iter.next() {
                shuffled.push(r.clone());
            }
        }
    }

    *deck = shuffled;
}

fn overhand_shuffle(deck: &mut Vec<Card>, passes: usize) {
    let mut rng = thread_rng();
    for _ in 0..passes {
        let mut shuffled = Vec::with_capacity(deck.len());
        while !deck.is_empty() {
            let chunk_size = rng.gen_range(1..=deck.len().min(10));
            let chunk: Vec<Card> = deck.drain(0..chunk_size).collect();
            shuffled.splice(0..0, chunk); // Insert chunk at the beginning
        }
        *deck = shuffled;
    }
}

fn deal_hands(deck: &mut Vec<Card>, num_hands: usize, hand_size: usize) -> Vec<Vec<Card>> {
    let mut hands = vec![Vec::with_capacity(hand_size); num_hands];

    for _ in 0..hand_size {
        for hand in &mut hands {
            if let Some(card) = deck.pop() {
                hand.push(card);
            }
        }
    }

    hands
}

fn calculate_entropy(counts: &HashMap<String, usize>, total: usize) -> f64 {
    counts
        .values()
        .map(|&count| {
            let p = count as f64 / total as f64;
            if p > 0.0 {
                -p * p.log2()
            } else {
                0.0
            }
        })
        .sum()
}

fn analyze_randomness(hands: &[Vec<Card>]) -> HashMap<String, f64> {
    let mut color_count = HashMap::new();
    let mut value_count = HashMap::new();
    let mut total_cards = 0;

    for hand in hands {
        for card in hand {
            let color_key = format!("{:?}", card.color);
            let value_key = match &card.value {
                Value::Number(v) => format!("Number: {}", v),
                Value::Wild => "Wild".to_string(),
                Value::Skip => "Skip".to_string(),
            };

            *color_count.entry(color_key).or_insert(0) += 1;
            *value_count.entry(value_key).or_insert(0) += 1;
            total_cards += 1;
        }
    }

    let mut metrics = HashMap::new();

    // Calculate color distribution
    for (color, count) in &color_count {
        metrics.insert(format!("Color: {}", color), *count as f64 / total_cards as f64);
    }

    // Calculate value distribution
    for (value, count) in &value_count {
        metrics.insert(format!("Value: {}", value), *count as f64 / total_cards as f64);
    }

    // Calculate entropy
    let color_entropy = calculate_entropy(&color_count, total_cards);
    let value_entropy = calculate_entropy(&value_count, total_cards);

    metrics.insert("Color Entropy".to_string(), color_entropy);
    metrics.insert("Value Entropy".to_string(), value_entropy);

    metrics
}

fn main() {
    let iterations = 1000; // Configure number of iterations
    let shuffle_times = 2; // Configure shuffle repetitions

    println!("{} 2xoverhand plus riffle", shuffle_times);

    let mut randomness_results = Vec::new();

    for _ in 0..iterations {
        let mut deck = generate_deck();
	//shuffle_deck(&mut deck, 5);

        //for _ in 0..shuffle_times {
        //    overhand_shuffle(&mut deck, 3); // Example: Use overhand shuffle with 3 passes
        //    overhand_shuffle(&mut deck, 3); // Example: Use overhand shuffle with 3 passes
        //    riffle_shuffle(&mut deck);
        //}

        let hands = deal_hands(&mut deck, NUM_HANDS, HAND_SIZE);
        let metrics = analyze_randomness(&hands);

        randomness_results.push(metrics);
    }

    // Aggregate metrics
    let mut aggregated_metrics = HashMap::new();
    let mut total_color_entropy = 0.0;
    let mut total_value_entropy = 0.0;

    for result in &randomness_results {
        for (key, value) in result {
            *aggregated_metrics.entry(key.clone()).or_insert(0.0) += value;

            if key == "Color Entropy" {
                total_color_entropy += value;
            }
            if key == "Value Entropy" {
                total_value_entropy += value;
            }
        }
    }

    // Compute overall metrics
    let avg_color_entropy = total_color_entropy / iterations as f64;
    let avg_value_entropy = total_value_entropy / iterations as f64;

    println!("Randomness Analysis over {} iterations:", iterations);
    for (key, total_value) in &aggregated_metrics {
        println!("{}: {:.4}", key, total_value / iterations as f64);
    }

    println!("\nOverall Metrics:");
    println!("Average Color Entropy: {:.4}", avg_color_entropy);
    println!("Average Value Entropy: {:.4}", avg_value_entropy);
}

