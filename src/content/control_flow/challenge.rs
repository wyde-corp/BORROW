//! The module challenge for Module 03.

use crate::content::types::{ExerciseKind, SeedExercise};

pub const GAUNTLET_CODE: &str = r#"// Stage 1 — classify a score with a match producing a value
fn classify(score: u32) -> &'static str {
    match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        _ => "F",
    }
}

// Stage 2 — sum only the valid scores, skipping the rest
fn average(scores: &[u32]) -> Option<f64> {
    let mut total = 0;
    let mut count = 0;

    for score in scores {
        if *score > 100 {
            continue;
        }
        total += score;
        count += 1;
    }

    if count == 0 {
        return None;
    }

    Some(total as f64 / count as f64)
}

// Stage 3 — find the first critical hit, or report that there was none
fn first_critical(rolls: &[u8]) -> Option<usize> {
    let mut index = 0;

    loop {
        if index >= rolls.len() {
            break None;
        }
        if rolls[index] >= 10 {
            break Some(index);
        }
        index += 1;
    }
}

// Stage 4 — describe a weapon without matching on every field
enum Weapon {
    Dagger { damage: u8 },
    Bow { damage: u8, range: u8 },
    Staff { damage: u8, mana: u16 },
}

fn short(weapon: &Weapon) -> String {
    match weapon {
        Weapon::Dagger { damage } => format!("dagger/{}", damage),
        Weapon::Bow { damage, range } => format!("bow/{}/{}", damage, range),
        Weapon::Staff { damage, .. } => format!("staff/{}", damage),
    }
}

fn main() {
    let scores = [95, 40, 150, 85, 200, 72];
    let rolls = [3, 7, 12, 4];
    let weapons = [
        Weapon::Dagger { damage: 4 },
        Weapon::Bow { damage: 9, range: 30 },
        Weapon::Staff { damage: 6, mana: 40 },
    ];

    for score in &scores {
        println!("{}: {}", score, classify(*score));
    }

    match average(&scores) {
        Some(avg) => println!("average: {:.1}", avg),
        None => println!("no valid scores"),
    }

    match first_critical(&rolls) {
        Some(i) => println!("critical at index {}", i),
        None => println!("no critical"),
    }

    for weapon in &weapons {
        println!("{}", short(weapon));
    }
}"#;

pub const GAUNTLET_SOLUTION: &str = r#"A fully working version, with the reasoning for each stage:

STAGE 1 — match producing a value
    Ranges cover 90..=100, 80..=89, 70..=79. The _ arm catches everything
    below 70 AND anything above 100. That is worth noticing: classify(150)
    returns "F", which is arguably wrong. A careful version would check the
    upper bound first and return "invalid". That is a real design question,
    not a syntax one.

STAGE 2 — continue to skip, early return for the empty case
    `continue` skips out-of-range scores so they never reach `total`. Then
    if nothing valid was seen, we return None BEFORE dividing — otherwise we
    would divide by zero. The `count == 0` check has to come first, and that
    ordering is the whole trick of this stage.

    The casts (`total as f64`) are needed because f64 division of two u32s
    would otherwise be integer division. You will meet this properly in the
    conversions lesson, but it is worth seeing here where it shows up in real
    code.

STAGE 3 — loop with break value
    `loop` because the exit is discovered inside — we do not know how far we
    will scan. `break None` and `break Some(index)` both exit WITH a value,
    which is what makes the function's return type Option<usize> work.

    Note the index check comes first. If the roll check came first, index
    would panic on an empty slice. Order matters.

STAGE 4 — matching without binding what you do not need
    Dagger and Bow have different fields, so those arms bind everything they
    have. Staff has `mana` that `short` does not use, so it is ignored with
    `..` rather than binding a name we would never read.

    We take &Weapon rather than Weapon so the array in main is not consumed
    and the loop can borrow each item in turn.

WHAT MAKES THIS HARDER
    Add a `Wand { charges: u8 }` variant and a `total_power` function that
    sums damage across a slice, using continue for a zero-damage weapon."#;

pub fn challenges() -> Vec<SeedExercise> {
    vec![
        SeedExercise {
            lesson_index: None,
            kind: ExerciseKind::Challenge,
            prompt: "THE CONTROL FLOW GAUNTLET — Four stages. Every control-flow tool you have learned, \
                     in one program."
                .to_string(),
            code: Some(GAUNTLET_CODE.to_string()),
            code_b: None,
            options: vec![
                "Stage 1 — rewrite classify with match guards, and handle scores above 100 explicitly".to_string(),
                "Stage 2 — fix the division so valid scores average correctly, and handle the empty case".to_string(),
                "Stage 3 — rewrite first_critical with a for loop and return the same Option".to_string(),
                "Stage 4 — add a Wand variant and a total_power function that sums damage with continue".to_string(),
                "Predict the exact output of the program as written before you change anything".to_string(),
            ],
            correct_answer: 0,
            explanation: GAUNTLET_SOLUTION.to_string(),
            xp_reward: 150,
            hints: vec![
                "Stage 2: what happens if count is zero? Move the check before the division.".to_string(),
                "Stage 3: why is the length check before the roll check? Swap them and see what breaks.".to_string(),
                "Stage 4: Staff has a field short() never reads. How do you skip binding it?".to_string(),
                "Run the program first. Predicting the output before running it is the whole exercise — if you get it right, you understand every line.".to_string(),
            ],
        },
        SeedExercise {
            lesson_index: None,
            kind: ExerciseKind::BuildIt,
            prompt: "BUILD IT — Write a turn-based combat resolver from scratch.".to_string(),
            code: None,
            code_b: None,
            options: vec![
                "Define an `Action` enum with at least four variants: Attack(u8), Heal(u8), Flee, and Wait".to_string(),
                "Write `resolve(action, health) -> String` that uses a match on the enum to produce a message".to_string(),
                "Include a guard: attacking while health is 0 should produce a different message".to_string(),
                "Write a `simulate(actions: &[Action]) -> u8` that loops over the actions, uses continue to skip healing when already at full health, and returns the final health".to_string(),
                "Use a loop with `break value` to stop early if health reaches 0, returning the index of the fatal blow".to_string(),
                "Print a turn-by-turn log. Choose whether to use a for loop or a while let over a Vec<Action>".to_string(),
            ],
            correct_answer: 0,
            explanation: "No single right answer. What matters is that every construct you used had a \
                         reason, and that you can explain why you chose a match over an if-else chain, or a \
                         loop with break over a for."
                .to_string(),
            xp_reward: 100,
            hints: vec![
                "Start with the enum. The variants you choose determine which match arms you need.".to_string(),
                "The early-break loop is exactly the `first_critical` pattern from the challenge, applied to health.".to_string(),
                "If your simulate function needs a mutable health variable across iterations, that is fine — but ask yourself whether a match expression could avoid it entirely.".to_string(),
            ],
        },
    ]
}
