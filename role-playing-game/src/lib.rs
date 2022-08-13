pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None;
        }

        Some(Player {
            health: 100,
            level: self.level,
            mana: if self.level >= 10 { Some(100) } else { None },
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(m) = self.mana {
            if m > mana_cost {
                self.mana = Some(m - mana_cost);
                return mana_cost * 2;
            }
            return 0;
        }

        self.health = self.health.saturating_sub(mana_cost);
        return 0;

    }
}
