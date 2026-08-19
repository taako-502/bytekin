use std::io::{self, Write};

struct Bitflora {
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

impl Bitflora {
    fn new(name: &str) -> Bitflora {
        Bitflora {
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
    println!("Hello, Bitflora!");
    let name = match read_name() {
        Ok(name) => name,
        Err(error) => {
            eprintln!("名前の読み取りに失敗しました: {}", error);
            return;
        }
    };
    let mut bitflora = Bitflora::new(&name);
    println!("{}が誕生しました！", bitflora.name);

    loop {
        println!();
        show_status(&bitflora);
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
            "1" => perform_action(&mut bitflora, Action::Train),
            "2" => perform_action(&mut bitflora, Action::Rest),
            "3" => show_history(&bitflora),
            "0" => {
                println!("またね、{}！", bitflora.name);
                break;
            }
            _ => println!("0から3の数字を入力してください。"),
        }
    }
}

fn read_name() -> io::Result<String> {
    print!("名前を入力してください（未入力ならMochi）: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(name_or_default(&input).to_string())
}

fn name_or_default(input: &str) -> &str {
    let name = input.trim();

    if name.is_empty() { "Mochi" } else { name }
}

fn perform_action(bitflora: &mut Bitflora, action: Action) {
    match bitflora.act(action) {
        Ok(()) => println!("行動に成功しました。"),
        Err(message) => println!("行動に失敗しました: {}", message),
    }
}

fn show_status(bitflora: &Bitflora) {
    println!(
        "{}: {} XP / Level {} / Energy {}",
        bitflora.name,
        bitflora.xp,
        bitflora.level(),
        bitflora.energy
    );

    match &bitflora.special_move {
        Some(move_name) => println!("Special Move: {}", move_name),
        None => println!("Special Move: Not learned"),
    }
}

fn show_history(bitflora: &Bitflora) {
    if bitflora.history.is_empty() {
        println!("行動履歴はまだありません。");
        return;
    }

    for (i, action) in bitflora.history.iter().enumerate() {
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
    fn new_bitflora_starts_with_name_and_zero_xp() {
        let bitflora = Bitflora::new("Mochi");
        assert_eq!(bitflora.name, "Mochi");
        assert_eq!(bitflora.xp, 0);
    }

    #[test]
    fn uses_default_name_when_input_is_empty() {
        assert_eq!(name_or_default("\n"), "Mochi");
        assert_eq!(name_or_default("   \n"), "Mochi");
    }

    #[test]
    fn trims_the_entered_name() {
        assert_eq!(name_or_default("  Flora  \n"), "Flora");
    }
}
