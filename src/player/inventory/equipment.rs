use serde::{Deserialize, Serialize};

use crate::utils::{
    input::select_from_str_array,
    messages::out_of_bounds,
    tui::{page_header, press_enter_to_continue, print_table, HeaderSubtext},
};

use crate::player::profile::Player;

use super::{armor::Armor, weapons::Weapon};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Equipment {
    pub armor: Option<Armor>,
    pub weapon: Option<Weapon>,
}

impl Equipment {
    pub fn print_table(&self) {
        let mut weapon_string: String = String::new();
        let mut armor_string: String = String::new();

        if self.weapon.is_none() {
            weapon_string = String::from("Weapon: None,Damage: N/A,Durability: N/A");
        } else {
            let weapon = self.weapon.clone().unwrap();

            weapon_string = format!(
                "Weapon: {},Damage: {},Durability: {}",
                weapon.name, weapon.damage, weapon.durability
            );
        }

        if self.armor.is_none() {
            armor_string = String::from("Armor: None,Defense: N/A,Durability: N/A");
        } else {
            let armor = self.armor.clone().unwrap();

            armor_string = format!(
                "Armor: {},Defense: {},Durability: {}",
                armor.name, armor.defense, armor.durability
            )
        }

        print_table(vec![weapon_string, armor_string])
    }

    pub fn menu(player: &mut Player) {
        page_header("Equipment Manager", HeaderSubtext::Keyboard);

        player.equipment.print_table();

        let choice = select_from_str_array(
            &[
                "Weapon: Equip",
                "Weapon: Un-Equip",
                "Armor:  Equip",
                "Armor:  Un-Equip",
                "NAV: Go Back",
            ],
            None,
        );

        match choice {
            0 => {
                Self::equip_weapon(player);
                Self::menu(player);
            }
            1 => {
                Self::unequip_weapon(player);
                Self::menu(player);
            }
            2 => {
                Self::equip_armor(player);
                Self::menu(player);
            }
            3 => {
                Self::unequip_armor(player);
                Self::menu(player);
            }
            4 => player.save(), // goes back to whatever menu called it due to recursion
            _ => out_of_bounds(),
        }
    }

    pub fn equip_weapon(player: &mut Player) {
        let choices = [
            &player.weapons.wooden_sword.name[..],
            &player.weapons.bronze_sword.name[..],
            &player.weapons.iron_sword.name[..],
            &player.weapons.steel_sword.name[..],
            &player.weapons.mystic_sword.name[..],
            &player.weapons.wizard_staff.name[..],
        ];

        let choice: usize = select_from_str_array(&choices, None);

        let weapon_option: Option<&Weapon> = match choice {
            0 => Some(&player.weapons.wooden_sword),
            1 => Some(&player.weapons.bronze_sword),
            2 => Some(&player.weapons.iron_sword),
            3 => Some(&player.weapons.steel_sword),
            4 => Some(&player.weapons.mystic_sword),
            5 => Some(&player.weapons.wizard_staff),
            _ => None,
        };

        if weapon_option.is_none() {
            out_of_bounds();
        }

        let weapon = weapon_option.unwrap();

        if !weapon.owns {
            println!("You do not own this weapon.");
            press_enter_to_continue();
        } else {
            player.equipment.weapon = Some(weapon.clone());
            println!("Equipped the {}", weapon.name);
            press_enter_to_continue();
        }
    }

    pub fn unequip_weapon(player: &mut Player) {
        if player.equipment.weapon.is_none() {
            println!("You do not have a weapon equipped.");
            press_enter_to_continue();
            return;
        }

        Self::overwrite_inventory_weapon(player.equipment.weapon.clone().unwrap(), player);

        player.equipment.weapon = None;
        println!("Weapon successfully unequipped.");
        press_enter_to_continue();
    }

    pub fn overwrite_inventory_weapon(equipped: Weapon, player: &mut Player) {
        let name = equipped.name.clone();
        let weapons = &mut player.weapons;

        if name == weapons.wooden_sword.name {
            weapons.wooden_sword = equipped;
        } else if name == weapons.bronze_sword.name {
            weapons.bronze_sword = equipped;
        } else if name == weapons.iron_sword.name {
            weapons.iron_sword = equipped;
        } else if name == weapons.steel_sword.name {
            weapons.steel_sword = equipped;
        } else if name == weapons.mystic_sword.name {
            weapons.mystic_sword = equipped;
        } else if name == weapons.wizard_staff.name {
            weapons.wizard_staff = equipped;
        }
    }

    fn equip_armor(player: &mut Player) {
        let choices = [
            &player.armor.leather.name[..],
            &player.armor.bronze.name[..],
            &player.armor.iron.name[..],
            &player.armor.dragonhide.name[..],
            &player.armor.mystic.name[..],
        ];

        let choice: usize = select_from_str_array(&choices, None);

        let option: Option<&Armor> = match choice {
            0 => Some(&player.armor.leather),
            1 => Some(&player.armor.bronze),
            2 => Some(&player.armor.iron),
            3 => Some(&player.armor.dragonhide),
            4 => Some(&player.armor.mystic),
            _ => None,
        };

        if option.is_none() {
            out_of_bounds();
        }

        let armor: &Armor = option.unwrap();

        if !armor.owns {
            println!("You do not own this weapon.");
            press_enter_to_continue();
        } else {
            player.equipment.armor = Some(armor.clone());
            println!("Equipped the {}", armor.name);
            press_enter_to_continue();
        }
    }

    fn unequip_armor(player: &mut Player) {
        if player.equipment.armor.is_none() {
            println!("You do not have armor equipped.");
            press_enter_to_continue();
            return;
        }

        Self::overwrite_inventory_armor(player.equipment.armor.clone().unwrap(), player);

        player.equipment.armor = None;
        println!("Armor successfully unequipped.");
        press_enter_to_continue();
    }

    pub fn overwrite_inventory_armor(equipped: Armor, player: &mut Player) {
        let name = equipped.name.clone();
        let armor = &mut player.armor;

        if name == armor.leather.name {
            armor.leather = equipped;
        } else if name == armor.bronze.name {
            armor.bronze = equipped
        } else if name == armor.iron.name {
            armor.iron = equipped;
        } else if name == armor.dragonhide.name {
            armor.dragonhide = equipped;
        } else if name == armor.mystic.name {
            armor.mystic = equipped;
        }
    }
}