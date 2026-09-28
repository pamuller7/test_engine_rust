#[derive(Copy, Clone, PartialEq)]
pub enum Characters {
    PriestStick, Priest, PriestHat, PriestHatKnife,
    KnightStick, KnightKnife, Knight,
    BlueSoul, BlueSkull,
    VampireHair, Vampire,
    SquelettonKnife, SquelettonStick, SquelettonSickle
}

impl Characters {
    pub fn coord(self) -> (f32, f32) {
        match self {
            Characters::PriestStick      => (0.0, 0.0),
            Characters::Priest           => (1.0, 0.0),
            Characters::PriestHat        => (2.0, 0.0),
            Characters::PriestHatKnife   => (3.0, 0.0),
            Characters::KnightStick      => (4.0, 0.0),
            Characters::KnightKnife      => (5.0, 0.0),
            Characters::Knight           => (6.0, 1.0),
            Characters::BlueSoul         => (0.0, 1.0),
            Characters::BlueSkull        => (1.0, 1.0),
            Characters::VampireHair      => (2.0, 1.0),
            Characters::Vampire          => (3.0, 1.0),
            Characters::SquelettonKnife  => (4.0, 1.0),
            Characters::SquelettonStick  => (5.0, 1.0),
            Characters::SquelettonSickle => (6.0, 1.0),
        }
    }
}


#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
    Empty,
    WWall1, WWall2, NWall1, NWall2, SeAngle, SWall, SwAngle, EWall1, EWall2, S5, S4, Void,
    Floor1, Floor2, Floor3, Floor4, Floor5, Floor6, Floor7, Floor8,
    Spider1, Door, Spider2, LatDoor1, LatDoor2, DoorOpen1, DoorOpen2,Doorrev,DoorOpen3,DoorOpen0,
    NTorch, Skeleton, Chest, Pot, DoubleWall
}

// tile coord: (x, y, blocking or not, printed after or before the player
//                                     0-> printed 1st layer
//                                     1-> depending on player pos
//                                     2-> printed last layer)
impl Tile {
    pub fn str_to_tile(index: &str) -> Tile {
        match index {
            "DW"    => Tile::DoubleWall,
            "W"     => Tile::WWall1,
            "N1"    => Tile::NWall1,
            "N2"    => Tile::NWall2,
            "D"     => Tile::Door,
            "Dr"    => Tile::Doorrev,
            "E"     => Tile::EWall1,
            "SW"    => Tile::SwAngle,
            "SE"    => Tile::SeAngle,
            "S"     => Tile::SWall,
            "F1"    => Tile::Floor1,
            "F2"    => Tile::Floor2,
            "F3"    => Tile::Floor3,
            "F4"    => Tile::Floor4,
            "F5"    => Tile::Floor5,
            "F6"    => Tile::Floor6,
            "F7"    => Tile::Floor7,
            "F8"    => Tile::Floor8,
            "S5"    => Tile::S5,
            "S4"    => Tile::S4,
            "."     => Tile::Void,
            "T"     => Tile::NTorch,
            "Sp2"   => Tile::Spider2,
            "Sk"    => Tile::Skeleton,
            "DL1"   => Tile::LatDoor1,
            "DL2"   => Tile::LatDoor2,
            "DO1"   => Tile::DoorOpen1,
            "DO2"   => Tile::DoorOpen2,
            "Chest" => Tile::Chest,
            "Sp1"   => Tile::Spider1,
            "Pot"   => Tile::Pot,
            _       => Tile::Empty,
        }
    }

    pub fn activate_tile(tile: Tile) -> Tile {
        match tile {
            Tile::Door => Tile::DoorOpen1,
            Tile::DoorOpen1 => Tile::Door,

            Tile::Doorrev => Tile::DoorOpen0,
            Tile::DoorOpen0 => Tile::Doorrev,

            Tile::LatDoor1 => Tile::DoorOpen2,
            Tile::DoorOpen2 => Tile::LatDoor1,

            Tile::LatDoor2 => Tile::DoorOpen3,
            Tile::DoorOpen3 => Tile::LatDoor2,

            _ => Tile::Void,
        }
    }

    pub fn rotate_tile(tile: Tile) -> (f32, bool, bool) {
        match tile {
            Tile::Doorrev => {
                (0.0, true, false)
            }
            Tile::DoorOpen0 => {
                (0.0, false, false)
            }
            Tile::DoorOpen1 => {
                (std::f32::consts::PI, false, true)
            }
            Tile::DoorOpen2 => {
                (std::f32::consts::FRAC_PI_2, true, false)
            }
            Tile::DoorOpen3 => {
                (0.0, true, false)
            }

            _   => {
                (0.0, false, false)
            }
        }
    }

    pub fn coord(self) -> (f32, f32, f32, f32) {
        match self {
            Tile::Empty   => (0.0, 0.0, 1.0, 0.0),

            //walls
            Tile::WWall1  => (0.0, 0.0, 0.0, 2.0),
            Tile::WWall2  => (0.0, 1.0, 0.0, 2.0),
            Tile::DoubleWall  => (3.5, 5.0, 0.0, 2.0),
            Tile::NWall1  => (1.0, 0.0, 0.0, 1.0),
            Tile::NWall2  => (3.0, 0.0, 0.0, 1.0),
            Tile::SwAngle => (0.0, 4.0, 0.0, 2.0),
            Tile::S5 =>(0.0, 5.0, 0.0, 2.0),
            Tile::S4 =>(3.0, 5.0, 0.0, 2.0),
            Tile::SWall   => (1.0, 4.0, 0.0, 1.0),
            Tile::SeAngle => (5.0, 4.0, 0.0, 2.0),
            Tile::EWall1  => (5.0, 0.0, 0.0, 2.0),
            Tile::EWall2  => (5.0, 1.0, 0.0, 2.0),

            //floors
            Tile::Floor1  => (6.0, 0.0, 1.0, 0.0),
            Tile::Floor2  => (7.0, 0.0, 1.0, 0.0),
            Tile::Floor3  => (8.0, 0.0, 1.0, 0.0),
            Tile::Floor4  => (9.0, 0.0, 1.0, 0.0),
            Tile::Floor5  => (6.0, 1.0, 1.0, 0.0),
            Tile::Floor6  => (7.0, 1.0, 1.0, 0.0),
            Tile::Floor7  => (8.0, 1.0, 1.0, 0.0),
            Tile::Floor8  => (9.0, 1.0, 1.0, 0.0),

            Tile::Void     => (8.0, 7.0, 1.0, 0.0), // bg color

            //decorations
            Tile::Spider1 => (4.0, 6.0, 1.0, 0.0),
            Tile::Spider2 => (5.0, 6.0, 1.0, 0.0),
            Tile::Door      => (7.0, 3.0, 0.0, 1.0),
            Tile::Doorrev   => (7.0, 3.0, 0.0, 1.0),
            Tile::DoorOpen0 => (7.0, 5.0, 1.0, 1.0),
            Tile::DoorOpen1 => (7.0, 5.0, 1.0, 1.0),
            Tile::DoorOpen2 => (7.0, 5.0, 1.0, 1.0),
            Tile::DoorOpen3 => (7.0, 3.0, 1.0, 1.0),
            Tile::LatDoor1 => (7.0, 5.0, 0.0, 2.0),
            Tile::LatDoor2 => (7.0, 4.0, 0.0, 2.0),
            Tile::NTorch   => (0.0, 9.0, 1.0, 1.0),
            Tile::Skeleton => (7.0, 7.0, 1.0, 0.0),
            Tile::Chest    => (0.0, 8.0, 0.0, 0.0),
            Tile::Pot      => (9.0, 8.0, 1.0, 0.0)
        }
    }
}
