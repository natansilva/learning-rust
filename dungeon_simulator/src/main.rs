use dungeon_simulator::Dice;
use dungeon_simulator::Player;

fn main() {
    let dice = Dice::new(6);
    let mut boss = Player::new("Blue Dragon".to_string(), 1000, 15, 10);
    let mut barbarian = Player::new("Barbarian".to_string(), 100, 10, 20);
    let mut mage = Player::new("Mage".to_string(), 100, 5, 20);
    let mut players: Vec<&mut Player> = vec![&mut barbarian, &mut mage];

    loop {
        for player in &players {
            let damage = dice.roll() + player.get_attack();
    
            println!("Player `{}` attack with {}", player.get_name(), damage);        
            boss.take_damage(damage);
            println!("Now boss `{}` has {} health", boss.get_name(), boss.get_health());
        }
    
        boss_attack(&mut boss, &mut players, &dice);
    
        if end_battle_check(&mut players, &boss) {
            println!("Battle ended");
            break;
        }
    }

}

fn end_battle_check(players: &mut Vec<&mut Player>, boss: &Player) -> bool {
    if players.iter().all(|player| !player.is_alive()) {
        println!("All players are dead");
        return true;
    }

    if ! boss.is_alive() {
        println!("Boss is dead");
        return true;
    }

    return false;
}

fn boss_attack(boss: &mut Player, players: &mut Vec<&mut Player>, dice: &Dice) {
    let random_index = dice.roll() % players.len() as u32;
    let player = &mut players[random_index as usize];

    let damage = dice.roll() + boss.get_attack();
    println!("Boss `{}` attacks player `{}` with {}", boss.get_name(), player.get_name(), damage);
    println!("Now player `{}` has {} health", player.get_name(), player.get_health());
    player.take_damage(damage);
}
