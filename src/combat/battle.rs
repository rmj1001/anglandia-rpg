use crate::{
    combat::enemy::{add_rewards_to_user, generate_rewards, Enemy, Rewards},
    data::{inventory::equipment::Equipment, player::Player, xp::XP},
    utils::{
        input,
        math::random_num,
        messages::out_of_bounds,
        tui::{page_header, press_enter_to_continue, sleep, HeaderSubtext},
    },
};

pub struct BattleSettings<'a> {
    pub header: &'static str,
    pub prompt: &'static str,
    pub player: &'a mut Player,
    pub enemy: Enemy,
    pub loops: usize,
    pub floor: usize,
    pub is_first_battle: bool,
    pub is_looped: bool,
    pub pause_seconds: u64,
    pub end_function: Option<fn(&mut Player)>,
}

use super::inventory::battle_inventory;

pub fn new_battle(battle: &mut BattleSettings) {
    // Prelude
    page_header(battle.header, HeaderSubtext::None);

    if battle.player.equipment.armor.is_none() || battle.player.equipment.weapon.is_none() {
        let confirm = input::confirm("Are you sure you want to fight without equipment? You'll die.");

        if !confirm {
            println!("Returning home.");
            press_enter_to_continue();

            crate::menus::game_menu::main(battle.player);
        }
    }

    if battle.loops > 0 {
        battle.floor += 1;
        battle.loops -= 1;
    }

    println!("{}", battle.prompt);
    sleep(battle.pause_seconds);

    if !battle.is_first_battle {
        battle.enemy = Enemy::new(battle.player.xp.combat, battle.player.health.hp);
    } else {
        battle.is_first_battle = false; // generate new enemy for subsequent battles
    }

    println!();
    println!("You are now fighting a {}.", battle.enemy.kind);
    sleep(battle.pause_seconds);
    battle_menu(battle);
}

pub fn battle_menu(battle: &mut BattleSettings) {
    page_header(
        format!("{} - {}", battle.header, battle.enemy.kind),
        HeaderSubtext::Keyboard,
    );

    if battle.is_looped {
        println!("Floor: {}", battle.floor);
        println!("Floors Left: {}", battle.loops);
        println!();
    }

    println!("Enemy: {}", battle.enemy.kind);
    println!("Enemy HP: {}", battle.enemy.hp);
    println!();

    println!("Player HP: {}", battle.player.health.hp);
    println!("Player Hunger: {}", battle.player.health.hunger);
    println!();

    let action = input::select_from_str_array(
        &[
            &format!("1. Attack the {}", battle.enemy.kind),
            "2. Inventory",
            "3. Retreat",
        ],
        None,
    );

    match action {
        0 => attack(battle),
        1 => {
            battle_inventory(battle.player);
            battle_menu(battle);
        }
        2 => retreat(battle.player),
        _ => out_of_bounds(),
    }
}

pub fn retreat(player: &mut Player) {
    page_header("Battle - Retreat", HeaderSubtext::None);

    println!("You have retreated from the battle.");
    press_enter_to_continue();

    crate::menus::game_menu::main(player);
}

pub fn attack(battle: &mut BattleSettings) {
    page_header(battle.header, HeaderSubtext::None);

    player_attack(battle);

    println!();

    enemy_attack(battle);

    println!();

    battle.player.health.heal();

    println!();

    press_enter_to_continue();

    battle_menu(battle);
}

fn player_attack(battle: &mut BattleSettings) {
    let enemy_type = battle.enemy.kind;

    println!("You attack the {}...", enemy_type);
    sleep(battle.pause_seconds);

    let hit = success_or_fail();

    if hit && battle.player.equipment.weapon.is_some() {
        let mut weapon = battle.player.equipment.weapon.clone().unwrap();
        let damage = weapon.damage;

        println!("You hit the {} for {} damage!", enemy_type, damage);

        weapon.decrease_durability();

        if !weapon.owns {
            battle.player.equipment.weapon = None;
        } else {
            battle.player.equipment.weapon = Some(weapon.clone());
        }

        Equipment::overwrite_inventory_weapon(weapon, battle.player);

        if battle.enemy.hp < damage {
            victory(battle);
        } else {
            battle.enemy.hp -= damage;
        }
    } else {
        println!("You missed the {}.", enemy_type);
    }

    sleep(battle.pause_seconds);
}

fn enemy_attack(battle: &mut BattleSettings) {
    let enemy_type = battle.enemy.kind;
    let mut damage: usize = battle.enemy.damage;

    if battle.player.equipment.armor.is_some() {
        let armor = battle.player.equipment.armor.clone().unwrap();

        if damage > armor.defense {
            damage -= armor.defense;
        } else {
            damage = 0
        }

        let mut new_armor = battle.player.equipment.armor.clone().unwrap();
        new_armor.decrease_durability();

        if !new_armor.owns {
            battle.player.equipment.armor = None;
        } else {
            battle.player.equipment.armor = Some(new_armor.clone());
        }

        Equipment::overwrite_inventory_armor(new_armor, battle.player);
    }

    println!("The {} attacks you...", enemy_type);
    sleep(battle.pause_seconds);

    let hit = success_or_fail();

    if hit && damage > 0 {
        println!("The {} hit you for {} damage!!", enemy_type, damage);

        if battle.player.health.hp < damage {
            defeat(battle);
        } else {
            battle.player.health.hp -= damage;
        }
    } else if damage == 0 {
        println!("The {} hit but the damage was negated by your armor!", enemy_type);
    } else {
        println!("The {} missed you.", enemy_type);
    }

    sleep(battle.pause_seconds);
}

fn success_or_fail() -> bool {
    let num = random_num(0, 1);

    num == 0
}

pub fn victory(battle: &mut BattleSettings) {
    page_header(format!("{} - Victory", battle.header), HeaderSubtext::None);

    println!("You successfully defeated the {}!", battle.enemy.kind);
    battle.player.health.restore();
    battle.player.achievements.monsters_killed += 1;
    println!();

    let rewards: Vec<Rewards> = generate_rewards(XP::get_level(battle.player.xp.total()));

    println!("Items Looted:");

    for reward in &rewards {
        println!("- {:?}", reward)
    }

    add_rewards_to_user(battle.player, rewards);
    println!();

    println!("Gained Combat XP: {}", battle.enemy.xp);
    battle.player.xp.combat += battle.enemy.xp;
    println!("Total Combat XP: {}", battle.player.xp.combat);
    println!();

    println!("Gained Gold: {}", battle.enemy.gold);
    battle.player.bank.wallet += battle.enemy.gold;
    println!("Total Gold: {}", battle.player.bank.wallet);
    println!();

    press_enter_to_continue();
    battle.player.save();

    if !battle.is_looped {
        crate::menus::game_menu::main(battle.player);
    }

    if battle.is_looped && battle.loops > 0 {
        new_battle(battle);
    }

    if battle.is_looped && battle.loops == 0 && battle.end_function.is_some() {
        battle.end_function.unwrap()(battle.player);
    }
}

pub fn defeat(battle: &mut BattleSettings) {
    page_header(format!("{} - Defeat", battle.header), HeaderSubtext::None);

    println!("You have been defeated in battle.");
    sleep(battle.pause_seconds);

    println!("You have been rushed to the local physician.");
    sleep(battle.pause_seconds);

    if battle.player.settings.hardmode {
        hardmode(battle);
    } else {
        revived(battle);
    }
}

pub fn revived(battle: &mut BattleSettings) {
    println!("You were successfully revived with 100 hp.");
    battle.player.health.reset();

    battle.player.save();
    press_enter_to_continue();
    crate::menus::game_menu::main(battle.player);
}

pub fn hardmode(battle: &mut BattleSettings) {
    let user_survives = random_num(0, 1);

    match user_survives {
        0 => {
            println!(
                "The {} stole all your gold and inventory, and you lost all your progress.",
                battle.enemy.kind
            );
            battle.player.die();
            sleep(battle.pause_seconds);

            revived(battle);
        }
        1 => {
            println!("You didn't survive. This profile will be deleted.");
            press_enter_to_continue();

            battle.player.delete();

            crate::menus::accounts::main();
        }
        _ => out_of_bounds(),
    }
}
