mod supplies;

use crate::supplies::{Dice, Sheet};

fn main() {
    let dice = Dice::new(6);

    let mut boss = Sheet::new(String::from("Blue Dragon"), 1000, 15, 10);

    let mut barbarian = Sheet::new(String::from("Barbarian"), 100, 5, 30);
    let mut warrior = Sheet::new(String::from("Warrior"), 100, 10, 20);
    let mut mage = Sheet::new(String::from("Mage"), 100, 5, 20);
    let mut players: Vec<&mut Sheet> = vec![&mut barbarian, &mut warrior, &mut mage];

    loop {
        for player in players.iter_mut() {
            let damage = dice.roll() + player.attack;
            boss.take_damage(damage);

            let boss_damage = dice.roll() + boss.attack;
            player.take_damage(boss_damage);
        }

        if end_battle_check(&mut players, &boss) {
            println!("---------");
            println!("Battle ended");

            for player in &players {
                println!("Player `{}` has {} health left", player.name, player.health);
            }
            println!("Boss has {} health left", boss.health);
            break;
        }
    }

}

fn end_battle_check(players: &mut Vec<&mut Sheet>, boss: &Sheet) -> bool {
    if players.iter().all(|player| !player.is_alive()) {
        return true;
    }

    if !boss.is_alive() {
        return true;
    }

    return false;
}
