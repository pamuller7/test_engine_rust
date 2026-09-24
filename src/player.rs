struct Item {
    cost: u32,
    name: String,
}

struct ItemStack {
    item: Item,
    quantity: u32,
}

struct Inventory {
    size: u32,
    items: Vec<ItemStack>,
}

impl Inventory {
    fn new(size: u32) -> Inventory {
        Inventory {
            size,
            items: Vec::new(),
        }
    }

    fn add_item(&mut self, cost: u32, name: String) {
		for stack in &mut self.items {
			if stack.item.name == name {
				stack.quantity += 1;
				return;
			}
		}
		if self.items.len() < self.size as usize {
			self.items.push(ItemStack {
				item: Item { cost, name },
				quantity: 1,
			});
		}
	}
}

pub struct Entity {
    nom: String,
    inventory: Inventory,
    location: String,
	coord: (f32, f32),
	flip: bool,
}

impl Entity {
    pub fn nouvelle(nom: String, coord: (f32, f32)) -> Entity {
        Entity {
            nom,
            inventory: Inventory::new(10),
            location: String::from("Marché"),
			coord,
			flip: false,
        }
    }

	pub fn get_coord(&self) -> (f32, f32){
        return self.coord
    }

	pub fn get_flip(&self) -> bool {
		return self.flip
	}

    fn se_presenter(&self) {
        println!(
            "Je m'appelle {} et je suis au {}",
            self.nom, self.location,
        );
    }

	fn aff_inteventory(&self) {
		for x in &self.inventory.items {
			println! (
				"item name: {} quantity: {} cost: {}",
				x.item.name, x.quantity, x.item.cost,
			)
		}
	}

	pub fn moove(&mut self, movement: &str) {
		let (x2, y2) = match movement {
			"N" => (0.0, -0.1),
			"S" => (0.0, 0.1),
			"E" => {
				self.flip = false;
				(0.1, 0.0)
			}
			"W" => {
				self.flip = true;
				(-0.1, 0.0)
			}
			_ => (0.0, 0.0),
		};
		self.coord.0 += x2;
		self.coord.1 += y2;
	}
}