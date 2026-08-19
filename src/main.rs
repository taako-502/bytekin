struct Bytekin {
    name: String,
    xp: u32,
    energy: u32,
    special_move: Option<String>,
}

enum Action {
    Train,
    Rest,
}

impl Bytekin {
    fn new(name: &str) -> Bytekin {
        Bytekin {
            name: name.to_string(),
            xp: 0,
            energy: 100,
            special_move: None,
        }
    }

    fn level(&self) -> u32 {
        calculate_level(self.xp)
    }

    fn train(&mut self) -> Result<(), String> {
        if self.energy < 20 {
            return Err(String::from("体力が足りません。休息してください。"));
        }
        self.xp += 25;
        self.energy -= 20;

        if self.level() >= 2 && self.special_move.is_none() {
            self.special_move = Some(String::from("Byte Burst!!"));
        }

        Ok(())
    }

    fn rest(&mut self) {
        self.energy += 20;
    }

    fn act(&mut self, action: Action) -> Result<(), String> {
        match action {
            Action::Train => self.train(),
            Action::Rest => {
                self.rest();
                Ok(())
            }
        }
    }
}

fn main() {
    println!("Hello, Bytekin!");
    println!("Level for 0 XP: {}", calculate_level(0));

    let mut bytekin = Bytekin::new("Mochi");

    println!(
        "{}: {} XP / Level {} / Energy {}",
        bytekin.name,
        bytekin.xp,
        bytekin.level(),
        bytekin.energy
    );

    bytekin.train();

    println!(
        "{}: {} XP / Level {} / Energy {}",
        bytekin.name,
        bytekin.xp,
        bytekin.level(),
        bytekin.energy
    );

    let actions = [
        Action::Train,
        Action::Rest,
        Action::Train,
        Action::Train,
        Action::Train,
        Action::Train,
        Action::Train,
    ];

    for action in actions {
        match bytekin.act(action) {
            Ok(_) => println!("Action performed successfully."),
            Err(e) => println!("Error: {}", e),
        }
    }

    println!(
        "{}: {} XP / Level {} / Energy {}",
        bytekin.name,
        bytekin.xp,
        bytekin.level(),
        bytekin.energy
    );

    match &bytekin.special_move {
        Some(move_name) => println!("Special Move: {}", move_name),
        None => println!("Special Move: Not learned"),
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
    fn new_bytekin_starts_with_name_and_zero_xp() {
        let bytekin = Bytekin::new("Mochi");
        assert_eq!(bytekin.name, "Mochi");
        assert_eq!(bytekin.xp, 0);
    }
}
