use std::collections::HashMap;

#[derive(Clone, Hash, Debug, Default, PartialEq, Eq)]
pub struct Player {
    pub score: u32,
    pub position: u32,
}

pub struct DeterministicDice {
    pub value: u32,
    pub roll_counter: u32,
}

impl DeterministicDice {
    pub fn roll(&mut self) -> u32 {
        self.roll_counter += 3;

        let mut roll_sum = 0;
        for _ in 1..=3 {
            self.value += 1;
            if self.value > 100 {
                self.value = 1;
            }

            roll_sum += self.value;
        }

        roll_sum
    }
}

#[derive(Clone, Hash, Debug, Default, PartialEq, Eq)]
pub struct Game {
    player: Vec<Player>,
    next_player: usize,
}

impl Game {
    pub fn new(pos1: u16, pos2: u16) -> Self {
        Self {
            player: vec![
                Player {
                    position: pos1 as u32,
                    score: 0,
                },
                Player {
                    position: pos2 as u32,
                    score: 0,
                },
            ],
            next_player: 0,
        }
    }

    pub fn get_wins(self, visited: &mut HashMap<Self, [u64; 2]>) -> [u64; 2] {
        if visited.contains_key(&self) {
            return *visited.get(&self).unwrap();
        }
        let mut res = [0, 0];
        for a in 1..=3 {
            for b in 1..=3 {
                for c in 1..=3 {
                    let mut n = self.clone();
                    n.player[n.next_player].position =
                        (n.player[n.next_player].position + a + b + c - 1) % 10 + 1;
                    n.player[n.next_player].score += n.player[n.next_player].position;
                    if n.player[n.next_player].score >= 21 {
                        res[n.next_player] += 1;
                    } else {
                        n.next_player = 1 - n.next_player;
                        let r = n.get_wins(visited);
                        res[0] += r[0];
                        res[1] += r[1];
                    }
                }
            }
        }
        visited.insert(self, res);
        res
    }
}