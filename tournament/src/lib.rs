use std::collections::HashMap;
use std::ops::Neg;
use std::str::FromStr;
use std::string::ToString;

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

struct Team {
    name: String,
    wins: u16,
    draws: u16,
    losses: u16,
}

impl Team {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            wins: 0,
            draws: 0,
            losses: 0,
        }
    }

    pub fn handle_outcome(&mut self, outcome: Outcome) {
        match outcome {
            Outcome::Win => {
                self.wins += 1;
            }
            Outcome::Draw => {
                self.draws += 1;
            }
            Outcome::Loss => {
                self.losses += 1;
            }
        }
    }

    pub fn matches(&self) -> u16 {
        self.wins + self.losses + self.draws
    }

    pub fn points(&self) -> u16 {
        (self.wins * 3) + self.draws
    }
}

impl ToString for Team {
    fn to_string(&self) -> String {
        format!(
            "{: <30} | {: >2} | {: >2} | {: >2} | {: >2} | {: >2}",
            self.name,
            self.matches(),
            self.wins,
            self.draws,
            self.losses,
            self.points()
        )
    }
}

#[derive(Copy, Clone)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Neg for Outcome {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Outcome::Win => Outcome::Loss,
            Outcome::Draw => Outcome::Draw,
            Outcome::Loss => Outcome::Win,
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(input: &str) -> Result<Outcome, Self::Err> {
        match input {
            "win" => Ok(Outcome::Win),
            "draw" => Ok(Outcome::Draw),
            "loss" => Ok(Outcome::Loss),
            _ => Err(()),
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut results: HashMap<&str, Team> = HashMap::new();

    match_results.lines().for_each(|result| {
        let frags: Vec<&str> = result.split(";").collect();
        results.entry(frags[0]).or_insert(Team::new(frags[0]));
        results.entry(frags[1]).or_insert(Team::new(frags[1]));

        match Outcome::from_str(frags[2]) {
            Ok(outcome) => {
                results
                    .entry(frags[0])
                    .and_modify(|t| t.handle_outcome(outcome));
                results
                    .entry(frags[1])
                    .and_modify(|t| t.handle_outcome(-outcome));
            }
            Err(err) => err,
        };
    });

    let mut final_results = results.into_values().collect::<Vec<Team>>();
    final_results.sort_by(|a, b| a.name.cmp(&b.name));
    final_results.sort_by(|a, b| b.points().cmp(&a.points()));

    let mut output_str: String = HEADER.to_string();
    final_results.iter().for_each(|fr| {
        output_str.push_str("\n");
        output_str.push_str(&fr.to_string());
    });

    output_str
}
