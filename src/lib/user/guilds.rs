use serde::{Deserialize, Serialize};

use super::profile::UserProfile;

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildMemberships {
    pub fishing: Guild,
    pub cooking: Guild,
    pub woodcutting: Guild,
    pub mining: Guild,
    pub smithing: Guild,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Guild {
    pub member: bool,
    pub member_price: u32,
}

pub enum PricedGuilds {
    Fishing,
    Cooking,
    Woodcutting,
    Mining,
    Smithing,
}

impl GuildMemberships {
    pub fn purchase(user: &mut UserProfile, guild_flag: PricedGuilds) {
        let guild: &mut Guild = match guild_flag {
            PricedGuilds::Fishing => &mut user.guild_memberships.fishing,
            PricedGuilds::Cooking => &mut user.guild_memberships.cooking,
            PricedGuilds::Woodcutting => &mut user.guild_memberships.woodcutting,
            PricedGuilds::Mining => &mut user.guild_memberships.mining,
            PricedGuilds::Smithing => &mut user.guild_memberships.smithing,
        };

        if user.gold < guild.member_price {
            println!("You do not have enough gold.");
            return;
        }

        user.gold -= guild.member_price;
        guild.member = true;
    }
}
