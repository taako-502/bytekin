use std::io::{self, Write};

struct Bytekin {
    name: String,
    xp: u32,
    energy: u32,
    special_move: Option<String>,
    history: Vec<Action>,
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
            history: Vec::new(),
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
    let mut bytekin = Bytekin::new("Mochi");

    loop {
        println!();
        show_status(&bytekin);
        println!("\n行動を選んでください");
        println!("1: Train");
        println!("2: Rest");
        println!("3: History");
        println!("0: Quit");
        print!("> ");

        if let Err(error) = io::stdout().flush() {
            eprintln!("画面への出力に失敗しました: {}", error);
            break;
        }

        let mut input = String::new();
        if let Err(error) = io::stdin().read_line(&mut input) {
            eprintln!("入力の読み取りに失敗しました: {}", error);
            break;
        }

        match input.trim() {
            "1" => perform_action(&mut bytekin, Action::Train),
            "2" => perform_action(&mut bytekin, Action::Rest),
            "3" => show_history(&bytekin),
            "0" => {
                println!("またね、{}！", bytekin.name);
                break;
            }
            _ => println!("0から3の数字を入力してください。"),
        }
    }
}

fn perform_action(bytekin: &mut Bytekin, action: Action) {
    match bytekin.act(action) {
        Ok(()) => println!("行動に成功しました。"),
        Err(message) => println!("行動に失敗しました: {}", message),
    }
}

fn show_status(bytekin: &Bytekin) {
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

fn show_history(bytekin: &Bytekin) {
    if bytekin.history.is_empty() {
        println!("行動履歴はまだありません。");
        return;
    }

    for (i, action) in bytekin.history.iter().enumerate() {
        let action_str = match action {
            Action::Train => "Train",
            Action::Rest => "Rest",
        };
        println!("Action {}: {}", i + 1, action_str);
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
