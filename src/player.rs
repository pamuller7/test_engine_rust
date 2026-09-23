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
}

impl Entity {
    pub fn nouvelle(nom: String, coord: (f32, f32)) -> Entity {
        Entity {
            nom,
            inventory: Inventory::new(10),
            location: String::from("Marché"),
			coord,
        }
    }

	pub fn get_coord(&self) -> (f32, f32){
        return self.coord
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
		let deplacement:(f32, f32) = match movement {
			"N" => (0.0, -0.1),
			"S" => (0.0, 0.1),
			"E" => (0.1, 0.0),
			"W" => (-0.1, 0.0),
			_   => (0.0, 0.0)
		};
		let (x1, y1) = self.coord;
		let (x2, y2) = deplacement;
		self.coord = (x1 + x2, y1 + y2);
	}
}