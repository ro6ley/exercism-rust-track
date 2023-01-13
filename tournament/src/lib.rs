use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

struct Team {
    name: String,
    matches: u16,
    points: u16,
    wins: u16,
    draws: u16,
    losses: u16,
}

impl Team {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            matches: 0,
            points: 0,
            wins: 0,
            draws: 0,
            losses: 0,
        }
    }

    pub fn win(&mut self) {
        self.matches += 1;
        self.wins += 1;
        self.points += 3;
    }

    pub fn draw(&mut self) {
        self.matches += 1;
        self.draws += 1;
        self.points += 1;
    }

    pub fn loss(&mut self) {
        self.matches += 1;
        self.losses += 1;
    }
}

impl ToString for Team {
    fn to_string(&self) -> String {
        format!(
            "{: <30} | {: >2} | {: >2} | {: >2} | {: >2} | {: >2}",
            self.name, self.matches, self.wins, self.draws, self.losses, self.points
        )
    }
}

enum Outcome {
    Win,
    Draw,
    Loss,
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
                match outcome {
                    Outcome::Win => {
                        results.entry(frags[0]).and_modify(|t| t.win());
                        results.entry(frags[1]).and_modify(|t| t.loss());
                    }
                    Outcome::Draw => {
                        results.entry(frags[0]).and_modify(|t| t.draw());
                        results.entry(frags[1]).and_modify(|t| t.draw());
                    }
                    Outcome::Loss => {
                        results.entry(frags[0]).and_modify(|t| t.loss());
                        results.entry(frags[1]).and_modify(|t| t.win());
                    }
                };
            }
            Err(err) => err,
        };
    });

    let mut final_results = results.into_values().collect::<Vec<Team>>();

    final_results.sort_by(|a, b| a.name.cmp(&b.name));
    final_results.sort_by(|a, b| b.points.cmp(&a.points));

    let mut output_str: String = HEADER.to_string();
    final_results.iter().for_each(|fr| {
        output_str.push_str("\n");
        output_str.push_str(&fr.to_string());
    });

    println!("{}", output_str);
    output_str
}
