use std::fs;

#[derive(Clone)]
pub struct ScoreRecord {
    pub score: u32,
    pub name: String,
}

pub struct Score {
    pub records: [Option<ScoreRecord>; 5],
}

impl Score {
    pub fn new() -> Score {
        Score {
            records: [None, None, None, None, None],
        }
    }
    pub fn fetch(&mut self) {
        let contents =
            fs::read_to_string("scores.txt").expect("Something went wrong reading the file");
        let lines = contents.lines();
        for (i, line) in lines.enumerate() {
            let mut parts = line.split_whitespace();
            let score = parts.next().unwrap().parse::<u32>().unwrap();
            let name = parts.next().unwrap();
            if i < 5 {
                self.records[i] = Some(ScoreRecord {
                    score,
                    name: String::from(name),
                });
            }
        }
    }
    pub fn store(&self) {
        let mut contents = String::new();
        for record in self.records.iter() {
            match record {
                Some(r) => {
                    contents.push_str(&format!("{} {}\n", r.score, r.name));
                }
                None => {}
            }
        }
        fs::write("scores.txt", contents).expect("Unable to write file");
    }
    pub fn add(&mut self, score: u32, name: String) {
        self.fetch();
        for i in 0..5 {
            match self.records[i] {
                Some(ref r) => {
                    if score > r.score {
                        for j in (i + 1..5).rev() {
                            self.records[j] = self.records[j - 1].clone()
                        }
                        self.records[i] = Some(ScoreRecord {
                            score,
                            name,
                        });
                        break;
                    }
                }
                None => {
                    self.records[i] = Some(ScoreRecord {
                        score,
                        name,
                    });
                    break;
                }
            }
        }
    }
}
