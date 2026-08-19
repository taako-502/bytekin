mod game;
mod terminal;

use game::{Action, Bitflora};

fn main() {
    terminal::show_welcome();

    let name = match terminal::read_name() {
        Ok(name) => name,
        Err(error) => {
            eprintln!("名前の読み取りに失敗しました: {}", error);
            return;
        }
    };

    let mut bitflora = Bitflora::new(&name);
    terminal::show_birth(&bitflora);

    loop {
        println!();
        terminal::show_status(&bitflora);

        let choice = match terminal::read_menu_choice() {
            Ok(choice) => choice,
            Err(error) => {
                eprintln!("入力の読み取りに失敗しました: {}", error);
                break;
            }
        };

        match choice.as_str() {
            "1" => terminal::perform_action(&mut bitflora, Action::Train),
            "2" => terminal::perform_action(&mut bitflora, Action::Rest),
            "3" => terminal::show_history(&bitflora),
            "0" => {
                terminal::show_goodbye(&bitflora);
                break;
            }
            _ => println!("0から3の数字を入力してください。"),
        }
    }
}
