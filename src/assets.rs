#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
    Empty,
    WWall1, WWall2, NWall1, NWall2, SeAngle, SWall, SwAngle, EWall1, EWall2, S5, S4, Void,
    Floor1, Floor2, Floor3, Floor4, Floor5, Floor6, Floor7, Floor8,
    Spider1, Door, Spider2, LatDoor1, LatDoor2, DoorOpen1, DoorOpen2,
    NTorch, Skeleton, Chest, Pot
}

// tile coord: (x, y, blocking or not, printed after or before the player
//                                     0-> printed 1st layer
//                                     1-> depending on player pos
//                                     2-> printed last layer)
impl Tile {

    pub fn str_to_tile(index: &str) -> Tile {
        match index {
            "W"     => Tile::WWall1,
            "N1"    => Tile::NWall1,
            "N2"    => Tile::NWall2,
            "D"     => Tile::Door,
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

    pub fn coord(self) -> (f32, f32, f32, f32) {
        match self {
            Tile::Empty   => (0.0, 0.0, 1.0, 0.0),

            //walls
            Tile::WWall1  => (0.0, 0.0, 0.0, 2.0),
            Tile::WWall2  => (0.0, 1.0, 0.0, 2.0),
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
            Tile::DoorOpen1 => (7.0, 5.0, 1.0, 2.0),
            Tile::DoorOpen2 => (7.0, 5.0, 1.0, 2.0),
            Tile::LatDoor1 => (7.0, 5.0, 0.0, 2.0),
            Tile::LatDoor2 => (7.0, 4.0, 0.0, 2.0),
            Tile::NTorch   => (0.0, 9.0, 1.0, 1.0),
            Tile::Skeleton => (7.0, 7.0, 1.0, 0.0),
            Tile::Chest    => (0.0, 8.0, 0.0, 0.0),
            Tile::Pot      => (9.0, 8.0, 1.0, 0.0)
        }
    }
}