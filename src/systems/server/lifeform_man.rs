use amethyst::{
    derive::SystemDesc,
    ecs::{Write, Read, System, SystemData},
};
use log::info;

use crate::{
    network::{Pack, Cmd, Dest},
    components::{Action, get_outfit, LifeformComponent},
    resources::{LifeformList, IO, MapList},
};

/// A simple system that receives a ton of network events.
#[derive(SystemDesc)]
pub struct LifeformManSystem;

impl<'a> System<'a> for LifeformManSystem {
    type SystemData = (
        Write <'a, IO>,
        Write<'a, LifeformList>,
        Read <'a, MapList>,
    );

    fn run(&mut self, (mut io, mut pl, maps): Self::SystemData) {
        for element in io.i.pop() {
            match &element.cmd {
                Cmd::Action(act) => {
                    info!("Action from Address: {:?}, Action: {:?}", element.ip(), element.cmd);
                    let acting_player = pl.get_from_ip(element.ip().unwrap()).unwrap(); 
                    info!("player gotten from IP is: {:?}", acting_player);
                    let packs_players = self.act(acting_player, act, &maps, &pl);
                    
                    // If packs come out of the action
                    for pack in packs_players.0 {
                        info!("{:?}", pack);
                        io.o.push(pack) 
                    }

                    // If a player needs to be replacd  
                    for player in packs_players.1 {
                        info!("{:?}", player);
                        pl.replace(player); 
                    }
                },
                Cmd::RemovePlayer(uid) => pl.remove_with_id(*uid), 
                _ => (io.i.push(element)), 
            }
        }
    }
}

impl LifeformManSystem {
    fn act(&mut self, 
           mut player: LifeformComponent, 
           act: &Action, 
           maps: &MapList, 
           pl: &LifeformList) 
           -> (Vec<Pack>, Vec<LifeformComponent>) 
        {
        let mut out = Vec::<Pack>::new();
        let mut players = Vec::<LifeformComponent>::new();

        match act {
            Action::Move(dir) => {
                player.orientation = dir.clone();
                info!("Checking to see if walk is allowed"); 
                if maps.get(&player.room).unwrap().allowed_move(&player.trans(), &player.orientation) {
                    info!("Player Walking"); 
                    player.walk();
                    players.push(player.clone());
                    let rm = player.room.clone();
                    out.push(Pack::new(Cmd::UpdatePlayer(player), Dest::Room(rm)));
                }
            },
            
            Action::ChangeOutfit(skin) => {
                player.skin = get_outfit(&skin);
                //TODO: Make sure skin in legal!
                players.push(player.clone());
                let rm = player.room.clone();
                out.push(Pack::new(Cmd::UpdatePlayer(player), Dest::Room(rm)));
            },

            Action::Melee => {
                let victom = pl.get_from_transform(player.in_front()); // Anyone in front of the player???
                info!("Swing!"); 
                match victom{
                    Some(mut victom) => {
                        info!("Direct Hit!");
                        victom.hp(-10.0); // Oh shit
                        players.push(victom.clone());
                        let rm = player.room.clone();
                        out.push(Pack::new(Cmd::UpdatePlayer(victom), Dest::Room(rm)));
                    },
                    None => info!("And a miss!"), 
                }
            },
            
            Action::Rotate(dir) => {
                player.orientation = dir.clone();
                players.push(player.clone());
                let rm = player.room.clone();
                out.push(Pack::new(Cmd::UpdatePlayer(player), Dest::Room(rm)));
            },
            _ => (), 
        };
        (out, players)
    }
}
