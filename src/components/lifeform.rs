use amethyst::{
    renderer::SpriteRender,
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, FlaggedStorage},
    renderer::palette::rgb::Srgba,
};

use serde::{Serialize, Deserialize};
use crate::{constants};
use crate::components::{Outfit, Skins, get_outfit, Monster};
use std::net::{SocketAddr};
use nalgebra::base::Vector3;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Orientation {
    South,
    West,
    East,
    North,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum LifeformType {
    Player,
    Monster,
    NPC,
}

/// Client Side player component
#[warn(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LifeformComponent {
    uid: u64,
    pub name: String,
    pub ip: Option<SocketAddr>, 
    pub room: String,
    pub x: f32,          
    pub y: f32, 
    pub skin: Outfit,
    pub orientation: Orientation,
    pub hp: f32,
    pub kind: LifeformType,
}

impl LifeformComponent {
    pub fn new_player(name: String, ip: SocketAddr, uid: u64) -> Self {
        Self {
            uid,
            name,
            ip: Some(ip),
            room: "resources/maps/town.tmx".to_string(),
            x: 8.0,
            y: 8.0,
            skin: get_outfit(&Skins::Female),
            orientation: Orientation::North,
            hp: 100.0,
            kind: LifeformType::Player,
        }
    }

    /// New Monster
    pub fn new_monster(
        uid: u64, 
        monster: &Monster,
        room: String,
        ) -> Self 
    {
        Self {
            uid,
            name: monster.name.clone(),
            ip: None,
            room,
            x: monster.x,
            y: monster.y,
            skin: monster.skin.clone(),
            orientation: Orientation::South,
            hp: monster.hp,
            kind: LifeformType::Monster,
        }
    }

   pub fn update_orientation(&mut self, or: Orientation) -> bool{
       let old = self.orientation.clone(); 
       self.orientation = or; 

       if old == self.orientation {
           return false
       }
       true
   }
    
    pub fn walk(&mut self) {
        match self.orientation {
            Orientation::North => self.y += constants::PLAYER_MOVE,
            Orientation::South => self.y -= constants::PLAYER_MOVE,
            Orientation::East  => self.x += constants::PLAYER_MOVE,
            Orientation::West  => self.x -= constants::PLAYER_MOVE,
        }
    }

    pub fn in_front(&self) -> Transform {
        let mut tr = self.trans();
        
        match self.orientation {
            Orientation::North => tr.move_up(constants::PLAYER_MOVE),  
            Orientation::South => tr.move_down(constants::PLAYER_MOVE),
            Orientation::East  => tr.move_right(constants::PLAYER_MOVE),
            Orientation::West  => tr.move_left(constants::PLAYER_MOVE),
        };
        tr
    }
    
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        1.0 
    }

    pub fn xyz(&self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, self.z()) 
    }

    pub fn hp(&mut self, amt: f32) {
        self.hp += amt;
    }

    pub fn trans(&self) -> Transform {
        let mut tr = Transform::default();
        tr.set_translation_xyz(self.x(), self.y(), self.z()); 
        tr
    }
    
    pub fn get_orientated(&self, sprites: &Vec<SpriteRender>) -> SpriteRender {
        match self.orientation {
            Orientation::North=> return sprites[self.skin.n].clone(),
            Orientation::South=> return sprites[self.skin.s].clone(),
            Orientation::East => return sprites[self.skin.e].clone(),
            Orientation::West => return sprites[self.skin.w].clone(),
        }
    }

    pub fn get_dir(&self) -> usize {
        match self.orientation {
            Orientation::North => self.skin.n,
            Orientation::South => self.skin.s,
            Orientation::East  => self.skin.e,
            Orientation::West  => self.skin.w,
        }
    }
    
    pub fn get_at(&self) -> usize {
        match self.orientation {
            Orientation::North => self.skin.at.n,
            Orientation::South => self.skin.at.s,
            Orientation::East  => self.skin.at.e,
            Orientation::West  => self.skin.at.w,
        }
    }

    pub fn get_sword(&self) -> usize {
        match self.orientation {
            Orientation::North => self.skin.at.s_n,
            Orientation::South => self.skin.at.s_s,
            Orientation::East  => self.skin.at.s_e,
            Orientation::West  => self.skin.at.s_w,
        }
    }
    
    /// Get the positions of the sword in space
    pub fn get_sword_pos(&self) -> Transform {
        match self.orientation {
            Orientation::North => Transform::default().move_forward(16.0).clone(),
            Orientation::South => Transform::default().move_backward(16.0).clone(),
            Orientation::East  => Transform::default().move_right(16.0).clone(),
            Orientation::West  => Transform::default().move_left(16.0).clone(),
        }
    }
    
    pub fn tint(&self) -> Srgba {
        Srgba::new((100.0 - self.hp)*0.05 + 1.0, 1.0, 1.0, 1.0)
    }

    pub fn id(&self) -> u64 { self.uid }
    
    pub fn ip(&self) -> SocketAddr { self.ip.unwrap() }
}

impl Component for LifeformComponent {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
