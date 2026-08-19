pub(crate) struct Bitflora {
    name: String,
    xp: u32,
    energy: u32,
    special_move: Option<String>,
    history: Vec<Action>,
}

pub(crate) enum Action {
    Train,
    Rest,
}

impl Bitflora {
    pub(crate) fn new(name: &str) -> Bitflora {
        Bitflora {
            name: name.to_string(),
            xp: 0,
            energy: 100,
            special_move: None,
            history: Vec::new(),
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn xp(&self) -> u32 {
        self.xp
    }

    pub(crate) fn energy(&self) -> u32 {
        self.energy
    }

    pub(crate) fn special_move(&self) -> Option<&str> {
        self.special_move.as_deref()
    }

    pub(crate) fn history(&self) -> &[Action] {
        &self.history
    }

    pub(crate) fn level(&self) -> u32 {
        calculate_level(self.xp)
    }

    fn train(&mut self) -> Result<(), String> {
        if self.energy < 20 {
            return Err(String::from("体力が足りません。休息してください。"));
        }

        self.xp += 25;
        self.energy -= 20;
        self.history.push(Action::Train);

        if self.level() >= 2 && self.special_move.is_none() {
            self.special_move = Some(String::from("Byte Burst!!"));
        }

        Ok(())
    }

    fn rest(&mut self) {
        self.history.push(Action::Rest);
        self.energy = (self.energy + 20).min(100);
    }

    pub(crate) fn act(&mut self, action: Action) -> Result<(), String> {
        match action {
            Action::Train => self.train(),
            Action::Rest => {
                self.rest();
                Ok(())
            }
        }
    }
}

fn calculate_level(xp: u32) -> u32 {
    xp / 100 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_level_1_when_xp_is_0() {
        let actual = calculate_level(0);
        assert_eq!(actual, 1);
    }

    #[test]
    fn level_increases_every_100_xp() {
        assert_eq!(calculate_level(99), 1);
        assert_eq!(calculate_level(100), 2);
        assert_eq!(calculate_level(199), 2);
        assert_eq!(calculate_level(200), 3);
    }

    #[test]
    fn new_bitflora_starts_with_name_and_zero_xp() {
        let bitflora = Bitflora::new("Mochi");
        assert_eq!(bitflora.name(), "Mochi");
        assert_eq!(bitflora.xp(), 0);
    }
}
