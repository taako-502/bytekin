use std::io::{self, Write};

use crate::game::{Action, Bitflora};

const BITFLORA_ART: &str = r#"          .-.
       .-(   )-.
      (___.-.___)
          \|/
       .-""""-.
      /  o  o  \
     |     ^     |
     |   \___/   |
      \  .:::.  /
       `-.___.-'
         /| |\
        /_| |_\
          / \
         /___\"#;

pub(crate) fn show_welcome() {
    println!("Hello, Bitflora!");
    println!("{}", BITFLORA_ART);
}

pub(crate) fn read_name() -> io::Result<String> {
    print!("名前を入力してください（未入力ならMochi）: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(name_or_default(&input).to_string())
}

pub(crate) fn read_menu_choice() -> io::Result<String> {
    println!("\n行動を選んでください");
    println!("1: Train");
    println!("2: Rest");
    println!("3: History");
    println!("0: Quit");
    print!("> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

pub(crate) fn show_birth(bitflora: &Bitflora) {
    println!("{}が誕生しました！", bitflora.name());
}

pub(crate) fn perform_action(bitflora: &mut Bitflora, action: Action) {
    match bitflora.act(action) {
        Ok(()) => println!("行動に成功しました。"),
        Err(message) => println!("行動に失敗しました: {}", message),
    }
}

pub(crate) fn show_status(bitflora: &Bitflora) {
    println!(
        "{}: {} XP / Level {} / Energy {}",
        bitflora.name(),
        bitflora.xp(),
        bitflora.level(),
        bitflora.energy()
    );

    match bitflora.special_move() {
        Some(move_name) => println!("Special Move: {}", move_name),
        None => println!("Special Move: Not learned"),
    }
}

pub(crate) fn show_history(bitflora: &Bitflora) {
    if bitflora.history().is_empty() {
        println!("行動履歴はまだありません。");
        return;
    }

    for (i, action) in bitflora.history().iter().enumerate() {
        let action_str = match action {
            Action::Train => "Train",
            Action::Rest => "Rest",
        };
        println!("Action {}: {}", i + 1, action_str);
    }
}

pub(crate) fn show_goodbye(bitflora: &Bitflora) {
    println!("またね、{}！", bitflora.name());
}

fn name_or_default(input: &str) -> &str {
    let name = input.trim();

    if name.is_empty() { "Mochi" } else { name }
}

#[cfg(test)]
mod tests {
    use super::*;

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
