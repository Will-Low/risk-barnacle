use rand::distributions::Triangular;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug, Deserialize)]
pub struct Range {
    // Used to create a triangle probuency distribution.
    // Low and High fields are mandatory, but error checking is delegated
    // to the range_checks method. Declaring all three fields here as
    // optional so we can use this struct prior to populating
    // these fields.
    pub low: Option<f64>,
    pub mode: Option<f64>,
    pub high: Option<f64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IntRange {
    // Used to create a triangle probuency distribution.
    // Low and High fields are mandatory, but error checking is delegated
    // to the range_checks method. Declaring all three fields here as
    // optional so we can use this struct prior to populating
    // these fields.
    pub low: Option<i64>,
    pub mode: Option<i64>,
    pub high: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    #[serde(rename = "event")]
    pub description: String,
    pub includes_controls: Option<bool>,
    #[serde(flatten)]
    pub range: IntRange,
    #[serde(skip)]
    pub prob: Option<Triangular>,
}

#[derive(Debug, Deserialize)]
pub struct Condition {
    #[serde(rename = "condition")]
    pub description: String,
    #[serde(flatten)]
    pub range: Range,
    #[serde(skip)]
    pub prob: Option<Triangular>,
}

#[derive(Debug, Deserialize)]
pub struct Cost {
    #[serde(rename = "cost")]
    pub description: String,
    #[serde(flatten)]
    pub range: Range,
    #[serde(skip)]
    pub prob: Option<Triangular>,
}

#[derive(Debug, Deserialize)]
pub struct Leaf {
    pub weight: f64,
    pub scenario: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
pub struct Play {
    pub description: String,
    // file_name to be appended on load.
    // TODO kill any file_name specified in a play file
    pub file_name: Option<String>,
    #[serde(flatten)]
    pub event: Event,
    pub scenario: Vec<Entry>,
    pub magnitude: Range,
    #[serde(skip)]
    pub magnitude_prob: Option<Triangular>,
    pub costs: Vec<Cost>,
}

impl Play {
    pub fn build_models(&mut self, events: &[Event], conditions: &[Condition], costs: &[Cost]) {
        // Build map of events, conditions, and costs to match against items in play
        let mut hashed_events = HashMap::new();
        for event in events {
            hashed_events.insert(&event.description, &event.range);
        }

        let mut hashed_conditions = HashMap::new();
        for condition in conditions {
            hashed_conditions.insert(&condition.description, &condition.range);
        }

        let mut hashed_costs = HashMap::new();
        for cost in costs {
            hashed_costs.insert(&cost.description, &cost.range);
        }

        // Match event values in play and move their values to the play
        self.event.range.low = hashed_events.get(&self.event.description).unwrap().low;
        self.event.range.mode = hashed_events.get(&self.event.description).unwrap().mode;
        self.event.range.high = hashed_events.get(&self.event.description).unwrap().high;
        self.event.prob = Some(Triangular::new(
            self.event.range.low.unwrap() as f64,
            self.event.range.high.unwrap() as f64,
            self.event.range.mode.unwrap() as f64,
        ));

        populate_conditions(&mut self.scenario, &hashed_conditions);

        // Make distribution for magnitude
        self.magnitude_prob = Some(Triangular::new(
            self.magnitude.low.unwrap(),
            self.magnitude.high.unwrap(),
            self.magnitude.mode.unwrap(),
        ));

        // Match cost values in play and move their values to the play
        for cost in &mut self.costs {
            hashed_costs.get(&cost.description).expect(
                format!(
                    "[ERROR] cost in play \"{}\" not found in \"costs.yaml\".",
                    &cost.description
                )
                .as_str(),
            );
            cost.range.low = hashed_costs.get(&cost.description).unwrap().low;
            cost.range.mode = hashed_costs.get(&cost.description).unwrap().mode;
            cost.range.high = hashed_costs.get(&cost.description).unwrap().high;
            cost.prob = Some(Triangular::new(
                cost.range.low.unwrap(),
                cost.range.high.unwrap(),
                cost.range.mode.unwrap(),
            ));
        }
    }
}

fn populate_conditions(entries: &mut Vec<Entry>, hashed_conditions: &HashMap<&String, &Range>) {
    for entry in entries {
        match entry {
            Entry::Single(condition) => {
                hashed_conditions.get(&condition.description).expect(
                    format!(
                        "[ERROR] condition in play \"{}\" not found in \"conditions.yaml\".",
                        &condition.description
                    )
                    .as_str(),
                );
                condition.range.low = hashed_conditions.get(&condition.description).unwrap().low;
                condition.range.mode = hashed_conditions.get(&condition.description).unwrap().mode;
                condition.range.high = hashed_conditions.get(&condition.description).unwrap().high;
                condition.prob = Some(Triangular::new(
                    condition.range.low.unwrap(),
                    condition.range.high.unwrap(),
                    condition.range.mode.unwrap(),
                ))
            }
            Entry::Branch(leaves) => {
                for leaf in leaves {
                    populate_conditions(&mut leaf.scenario, &hashed_conditions);
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum Entry {
    #[serde(rename = "single")]
    Single(Condition),
    #[serde(rename = "branch")]
    Branch(Vec<Leaf>),
}

pub struct MonteCarloResult {
    pub description: String,
    pub annual_loss_event_prob: f64,
    pub fifth_percentile: f64,
    pub ninety_fifth_percentile: f64,
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
}

impl Range {
    pub fn range_checks(&self, data_type: &str, description: &str) {
        match (self.low, self.high) {
            (None, None) => panic!(
                "{} \"{}\" missing both low and high values. \
                 These are mandatory.",
                data_type, description
            ),
            (None, Some(_)) => panic!(
                "{} \"{}\" missing low value. This is mandatory.",
                data_type, description
            ),
            (Some(_), None) => panic!(
                "{} \"{}\" missing high value. This is mandatory.",
                data_type, description
            ),
            (Some(_), Some(_)) => (),
        }

        assert!(
            self.low > Some(0.0),
            format!(
                "{} \"{}\" low value is '{}'. This field must be zero or greater.",
                data_type,
                description,
                self.low.unwrap()
            )
        );
        assert!(
            self.high > Some(0.0),
            format!(
                "{} \"{}\" high value is '{}'. This field must be zero or greater.",
                data_type,
                description,
                self.high.unwrap()
            )
        );

        // Check Low <= High
        if self.low.unwrap() > self.high.unwrap() {
            panic!(
                "{} \"{}\" low value '{}' is larger than high value '{}'",
                data_type,
                description,
                self.low.unwrap(),
                self.high.unwrap()
            );
        }

        // If Mode exists, check Low <= Mode; check Mode <= High.
        if self.mode.is_some() {
            assert!(
                self.mode > Some(0.0),
                format!(
                    "{} \"{}\" mode value is '{}'. This field must be zero or greater.",
                    data_type,
                    description,
                    self.mode.unwrap()
                )
            );
            if self.low.unwrap() > self.mode.unwrap() {
                panic!(
                    "{} \"{}\" low value '{}' is larger than mode value '{}'",
                    data_type,
                    description,
                    self.low.unwrap(),
                    self.mode.unwrap()
                );
            }
            if self.mode.unwrap() > self.high.unwrap() {
                panic!(
                    "{} \"{}\" mode value '{}' is larger than high value '{}'",
                    data_type,
                    description,
                    self.mode.unwrap(),
                    self.high.unwrap()
                );
            }
        }
    }
}

pub trait Validation {
    fn validate(&self);
}

impl Validation for Vec<Condition> {
    fn validate(&self) {
        let mut descriptions = HashSet::new(); // For duplicate checking
        for condition in self {
            // Check range logic
            condition
                .range
                .range_checks("conditions.yaml", &condition.description);
            // Check for duplicates
            if descriptions.contains(&condition.description) {
                panic!("Found multiple entries of \"{}\" in \"conditions.yaml\". Exactly one entry is required.", condition.description);
            } else {
                descriptions.insert(condition.description.clone());
            }
        }
    }
}

impl IntRange {
    pub fn range_checks(&self, data_type: &str, description: &str) {
        match (self.low, self.high) {
            (None, None) => panic!(
                "{} \"{}\" missing both low and high values. \
                 These are mandatory.",
                data_type, description
            ),
            (None, Some(_)) => panic!(
                "{} \"{}\" missing low value. This is mandatory.",
                data_type, description
            ),
            (Some(_), None) => panic!(
                "{} \"{}\" missing high value. This is mandatory.",
                data_type, description
            ),
            (Some(_), Some(_)) => (),
        }

        assert!(
            self.low > Some(0),
            format!(
                "{} \"{}\" low value is '{}'. This field must be zero or greater.",
                data_type,
                description,
                self.low.unwrap()
            )
        );
        assert!(
            self.high > Some(0),
            format!(
                "{} \"{}\" high value is '{}'. This field must be zero or greater.",
                data_type,
                description,
                self.high.unwrap()
            )
        );

        // Check Low <= High
        if self.low.unwrap() > self.high.unwrap() {
            panic!(
                "{} \"{}\" low value '{}' is larger than high value '{}'",
                data_type,
                description,
                self.low.unwrap(),
                self.high.unwrap()
            );
        }

        // If Mode exists, check Low <= Mode; check Mode <= High.
        if self.mode.is_some() {
            assert!(
                self.mode > Some(0),
                format!(
                    "{} \"{}\" mode value is '{}'. This field must be zero or greater.",
                    data_type,
                    description,
                    self.mode.unwrap()
                )
            );
            if self.low.unwrap() > self.mode.unwrap() {
                panic!(
                    "{} \"{}\" low value '{}' is larger than mode value '{}'",
                    data_type,
                    description,
                    self.low.unwrap(),
                    self.mode.unwrap()
                );
            }
            if self.mode.unwrap() > self.high.unwrap() {
                panic!(
                    "{} \"{}\" mode value '{}' is larger than high value '{}'",
                    data_type,
                    description,
                    self.mode.unwrap(),
                    self.high.unwrap()
                );
            }
        }
    }
}

impl Validation for Vec<Event> {
    fn validate(&self) {
        let mut descriptions = HashSet::new(); // For duplicate checking
        for event in self {
            event
                .range
                .clone()
                .range_checks("events.yaml", &event.description);
            // Check for duplicates
            if descriptions.contains(&event.description) {
                panic!("Found multiple entries of \"{}\" in \"events.yaml\". Exactly one entry is required.", event.description);
            } else {
                descriptions.insert(event.description.clone());
            }
        }
    }
}

impl Validation for Vec<Cost> {
    fn validate(&self) {
        let mut descriptions = HashSet::new(); // For duplicate checking
        for cost in self {
            cost.range.range_checks("costs.yaml", &cost.description);
            // Check for duplicates
            if descriptions.contains(&cost.description) {
                panic!("Found multiple entries of \"{}\" in \"costs.yaml\". Exactly one entry is required.", cost.description);
            } else {
                descriptions.insert(cost.description.clone());
            }
        }
    }
}

impl Validation for Play {
    fn validate(&self) {
        let file_name = self.file_name.clone().unwrap();
        self.magnitude
            .range_checks(&format!("Play: {}", file_name), &String::from("magnitude"));

        self.scenario.check_weight_totals();
    }
}

trait WeightChecking {
    fn check_weight_totals(&self);
}

impl WeightChecking for Vec<Entry> {
    fn check_weight_totals(&self) {
        for each in self {
            if let Entry::Branch(leaves) = each {
                let mut weight_total = 0.0;
                for leaf in leaves {
                    weight_total += leaf.weight;
                    leaf.scenario.check_weight_totals();
                }
                assert!(
                    weight_total == 100.0,
                    "Weight totals for a branch don't equal 100."
                );
            }
        }
    }
}
