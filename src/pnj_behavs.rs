use crate::player::Entity;

pub fn pnj_behaviour(entity: &mut Entity, player: &mut Entity) -> String {
	match entity.get_name().as_str() {
		"Vampire" => {
			vamp_behaviour(entity, player)
		}
		_     => {
			format!("{}: {}", entity.get_name(), entity.talk())
		}
	}
}

fn vamp_behaviour(entity: &mut Entity, player: &mut Entity) -> String {
	for (name, _quantity, _cost) in player.get_inventory() {
		if name == "Silver key" {
			return format!("NOOOO {} GET THIS SILVER AWAY FROM ME", player.get_name())
		}
		if name == "Potion" {
			return format!("PLEASE {} GIVE THIS POTION TO ME", player.get_name())
		}
	}
	format!("{}: {}", entity.get_name(), entity.talk())	
}