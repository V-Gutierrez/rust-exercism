// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    fn new(level: u32) -> Player {
        Self {
            health: 100,
            mana: if level < 10 {
                None
            } else {
                Some(100)
            },
            level: level,
        }
    }

    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player::new(self.level)),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        // if no mana => take from health and return damage 0
        // if mana => take from mana and return damage
        match self.mana {
            Some(current_mana) => {
                if current_mana > mana_cost {
                    let new_mana = current_mana - mana_cost;

                    self.mana = Some(new_mana);

                    return mana_cost * 2;
                } else {
                    return 0;
                }
            }
            None => {
                if self.health >= mana_cost {
                    self.health -= mana_cost;
                } else {
                    self.health = 0;
                }

                return 0;
            }
        }
    }
}