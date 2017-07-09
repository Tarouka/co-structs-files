pub struct BaseItem {
	id:              u32,
	name:            String,
	requirements:    ItemRequirements,
	flags:           ItemFlags,
	combat_stats:    ItemCombatStats,
	weight:          u16,
	buy_price:       u32,
	action_id:       u32,
	hp_restored:     u16,
	mp_restored:     u16,
	amount:          u16,
	amount_limit:    u16,
	status:          u16,
	gem1:            u8,
	gem2:            u8,
	magic1:          u8,
	magic2:          u8,
	magic3:          u8,
	fray_mode:       u32,
	repair_mode:     u32,
	type_mask:       u32,
	buy_cps_price:   u16,
	type_name:       String,
	description:     String,
	unknown_1:       u8
}

impl BaseItem {
	// Gets unique item base ID
	pub fn get_id(&self) -> u32 { self.id }

	// Gets item name
	pub fn get_name(&self) -> &String { &self.name }

	// Gets item requirements
	pub fn get_requirements(&self) -> &ItemRequirements { &self.requirements }

	// Gets item flags
	pub fn get_flags(&self) -> &ItemFlags { &self.flags }

	// Gets item combat stats
	pub fn get_combat_stats(&self) -> &ItemCombatStats { &self.combat_stats }

	// Gets item weight
	pub fn get_weight(&self) -> u16 { self.weight }

	// Gets item silver buy price
	pub fn get_buy_price(&self) -> u32 { self.buy_price }

	// Gets item action ID
	pub fn get_action_id(&self) -> u32 { self.action_id }

	// Gets item total HP restored (e.g. health potions)
	pub fn get_hp_restored(&self) -> u16 { self.hp_restored }

	// Gets item total MP restored (e.g. mana potions)
	pub fn get_mp_restored(&self) -> u16 { self.mp_restored }

	// Gets item amount/quantity (notably used for arrow packs)
	pub fn get_amount(&self) -> u16 { self.amount }

	// Gets amount limit value
	pub fn get_amount_limit(&self) -> u16 { self.amount_limit }

	// Gets item status caused
	pub fn get_status(&self) -> u16 { self.status }

	// Gets item Gem1 attribute
	pub fn get_gem1(&self) -> u8 { self.gem1 }

	// Gets item Gem2 attribute
	pub fn get_gem2(&self) -> u8 { self.gem2 }

	// Gets item Magic1 attribute
	pub fn get_magic1(&self) -> u8 { self.magic1 }

	// Gets item Magic2 attribute
	pub fn get_magic2(&self) -> u8 { self.magic2 }

	// Gets item Magic3 attribute
	pub fn get_magic3(&self) -> u8 { self.magic3 }

	// Gets item FrayMode
	pub fn get_fray_mode(&self) -> u32 { self.fray_mode }

	// Gets item repair mode
	pub fn get_repair_mode(&self) -> u32 { self.repair_mode }

	// Gets item type mask
	pub fn get_type_mask(&self) -> u32 { self.type_mask }

	// Gets item CP price (for buying)
	pub fn get_buy_cps_price(&self) -> u16 { self.buy_cps_price }

	// Gets item type name
	pub fn get_type_name(&self) -> &String { &self.type_name }

	// Gets item description
	pub fn get_description(&self) -> &String { &self.description }

	// Gets item unknown flag
	pub fn get_unknown_1(&self) -> u8 { self.unknown_1 }
}

pub struct ItemFlags {
	sell_disabled:       bool,
	no_drop_on_death:    bool,
	important_sell_hint: bool,
	important_drop_hint: bool,
	is_unstoreable:      bool,
	is_untradeable:      bool
}

impl ItemFlags {
	pub fn get_flags_from_byte(flags: &i32) -> ItemFlags {
		ItemFlags {
			sell_disabled:       (flags & 0x20) > 0,
			no_drop_on_death:    (flags & 0x10) > 0,
			important_sell_hint: (flags & 0x08) > 0,
			important_drop_hint: (flags & 0x04) > 0,
			is_unstoreable:      (flags & 0x02) > 0,
			is_untradeable:      (flags & 0x01) > 0
		}
	}

	pub fn get_byte_from_flags(&self) -> u8 {
		let mut flag: u8 = 0;
		flag |= if self.is_untradeable { 0x01 } else { 0x00 };
		flag |= if self.is_unstoreable { 0x02 } else { 0x00 };
		flag |= if self.important_drop_hint { 0x04 } else { 0x00 };
		flag |= if self.important_sell_hint { 0x08 } else { 0x00 };
		flag |= if self.no_drop_on_death { 0x10 } else { 0x00 };
		flag |= if self.sell_disabled { 0x20 } else { 0x00 };

		flag
	}
}

pub struct ItemRequirements {
	profession:      u8,
	proficiency_lvl: u8,
	lvl:             u8,
	sex:             u8,
	str:             u16,
	agi:             u16,
	vit:             u16,
	spi:             u16
}

impl ItemRequirements {
	// Tests if profession level matches item requirement.
	pub fn is_profession_ok(&self, profession: &u8) -> bool { *profession - (*profession % 10) == self.profession }

	// Tests if profiency level matches item requirement.
	pub fn is_proficiency_lvl_ok(&self, proficiency_lvl: &u8) -> bool { *proficiency_lvl >= self.proficiency_lvl }

	// Tests if combat level matches item requirement.
	pub fn is_lvl_ok(&self, lvl: &u8) -> bool { *lvl >= self.lvl }

	// Tests if STR stat matches item requirement.
	pub fn is_str_ok(&self, str: &u16) -> bool { *str >= self.str }

	// Tests if AGI stat matches item requirement.
	pub fn is_agi_ok(&self, agi: &u16) -> bool { *agi >= self.agi }

	// Tests if VIT stat matches item requirement.
	pub fn is_vit_ok(&self, vit: &u16) -> bool { *vit >= self.vit }

	// Tests if SPI stat matches item requirement.
	pub fn is_spi_ok(&self, spi: &u16) -> bool { *spi >= self.spi }

	// Returns the sex ID required to equip the item.
	pub fn get_sex_required(&self) -> u8 { self.sex }
}

pub struct ItemCombatStats {
	pub max_phys_atk:    u16,
	pub min_phys_atk:    u16,
	pub phys_def:        u16,
	pub accuracy:        u8,
	pub dodge:           u8,
	pub magic_atk:       u16,
	pub magic_def:       u16,
	pub atk_range:       u8,
	pub atk_speed:       u16
}

impl ItemCombatStats {
	// Gets max phys ATK value
	pub fn get_max_phys_atk(&self) -> u16 { self.max_phys_atk }

	// Gets min phys ATK value
	pub fn get_min_phys_atk(&self) -> u16 { self.min_phys_atk }

	// Gets phys DEF value
	pub fn get_phys_def(&self) -> u16 { self.phys_def }

	// Gets accuracy value
	pub fn get_accuracy(&self) -> u8 { self.accuracy }

	// Gets dodge value
	pub fn get_dodge(&self) -> u8 { self.dodge }

	// Gets magic ATK value
	pub fn get_magic_atk(&self) -> u16 { self.magic_atk }

	// Gets Magic DEF value
	pub fn get_magic_def(&self) -> u16 { self.magic_def }

	// Gets ATK range value
	pub fn get_atk_range(&self) -> u8 {self.atk_range }

	// Gets ATK speed value
	pub fn get_atk_speed(&self) -> u16 { self.atk_speed }
}

mod serializer {
	use ::item::BaseItem;
	use ::serializer::{StringSerializableStructure, SpaceSeparatedSerializer};

	impl StringSerializableStructure for BaseItem {
		fn serialize(&self) -> String {
			let mut serializer = SpaceSeparatedSerializer::new();

			serializer.push_value(&self.id);
			serializer.push_value(&self.name);
			serializer.push_value(&self.requirements.profession);
			serializer.push_value(&self.requirements.proficiency_lvl);
			serializer.push_value(&self.requirements.lvl);
			serializer.push_value(&self.requirements.sex);
			serializer.push_value(&self.requirements.str);
			serializer.push_value(&self.requirements.agi);
			serializer.push_value(&self.requirements.vit);
			serializer.push_value(&self.requirements.spi);
			serializer.push_value(&self.flags.get_byte_from_flags());
			serializer.push_value(&self.weight);
			serializer.push_value(&self.buy_price);
			serializer.push_value(&self.action_id);
			serializer.push_value(&self.combat_stats.max_phys_atk);
			serializer.push_value(&self.combat_stats.min_phys_atk);
			serializer.push_value(&self.combat_stats.phys_def);
			serializer.push_value(&self.combat_stats.accuracy);
			serializer.push_value(&self.combat_stats.dodge);
			serializer.push_value(&self.hp_restored);
			serializer.push_value(&self.mp_restored);
			serializer.push_value(&self.amount);
			serializer.push_value(&self.amount_limit);
			serializer.push_value(&self.status);
			serializer.push_value(&self.gem1);
			serializer.push_value(&self.gem2);
			serializer.push_value(&self.magic1);
			serializer.push_value(&self.magic2);
			serializer.push_value(&self.magic3);
			serializer.push_value(&self.combat_stats.magic_atk);
			serializer.push_value(&self.combat_stats.magic_def);
			serializer.push_value(&self.combat_stats.atk_range);
			serializer.push_value(&self.combat_stats.atk_speed);
			serializer.push_value(&self.fray_mode);
			serializer.push_value(&self.repair_mode);
			serializer.push_value(&self.type_mask);
			serializer.push_value(&self.buy_cps_price);
			serializer.push_value(&self.type_name);
			serializer.push_value(&self.description);
			serializer.push_value(&self.unknown_1);

			serializer.get_buffer()
		}
	}
}