pub struct BaseItem {
	pub id:              u32,
	pub name:            String,
	pub requirements:    ItemRequirements,
	pub flags:           ItemFlags,
	pub combat_stats:    ItemCombatStats,
	pub weight:          u16,
	pub buy_price:       u32,
	pub action_id:       u32,
	pub hp_restored:     u16,
	pub mp_restored:     u16,
	pub amount:          u16,
	pub amount_limit:    u16,
	pub status:          u16,
	pub gem1:            u8,
	pub gem2:            u8,
	pub magic1:          u8,
	pub magic2:          u8,
	pub magic3:          u8,
	pub fray_mode:       u32,
	pub repair_mode:     u32,
	pub type_mask:       u32,
	pub buy_cps_price:   u16,
	pub type_name:       String,
	pub description:     String,
	pub unknown_1:       u8
}

pub struct ItemFlags {
	pub sell_disabled:       bool,
	pub no_drop_on_death:    bool,
	pub important_sell_hint: bool,
	pub important_drop_hint: bool,
	pub is_unstoreable:      bool,
	pub is_untradeable:      bool
}

impl ItemFlags {
	pub fn from_byte(flags: &u8) -> ItemFlags {
		ItemFlags {
			sell_disabled:       (flags & 0x20) > 0,
			no_drop_on_death:    (flags & 0x10) > 0,
			important_sell_hint: (flags & 0x08) > 0,
			important_drop_hint: (flags & 0x04) > 0,
			is_unstoreable:      (flags & 0x02) > 0,
			is_untradeable:      (flags & 0x01) > 0
		}
	}

	pub fn get_as_byte(&self) -> u8 {
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
	pub profession:      u8,
	pub proficiency_lvl: u8,
	pub lvl:             u8,
	pub sex:             u8,
	pub str:             u16,
	pub agi:             u16,
	pub vit:             u16,
	pub spi:             u16
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
			serializer.push_value(&self.flags.get_as_byte());
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

mod parser {
	use super::*;
	use parser::*;

	impl SpaceSeparatedParseable<BaseItem> for BaseItem {
		fn from_line(line: &String) -> Result<BaseItem, SpaceSeparatedParserError> {
			let results = get_space_separated_values(line);

			if results.len() < 38 {
				return Result::Err(SpaceSeparatedParserError::MissingAtIndex(results.len()));
			}

			let item = BaseItem {
				id:            get_value_as!(results, 0, u32, "id"),
				name:          results[1].get_value().to_owned(),

				requirements:  ItemRequirements {
					profession:      get_value_as!(results, 1, u8, "profession requirement"),
					proficiency_lvl: get_value_as!(results, 2, u8, "proficiency level requirement"),
					lvl:             get_value_as!(results, 3, u8, "level requirement"),
					sex:             get_value_as!(results, 4, u8, "sex requirement"),
					str:             get_value_as!(results, 5, u16, "strength requirement"),
					agi:             get_value_as!(results, 6, u16, "agility requirement"),
					vit:             get_value_as!(results, 7, u16, "vitality requirement"),
					spi:             get_value_as!(results, 8, u16, "spirit requirement")
				},

				flags:         ItemFlags::from_byte(&get_value_as!(results, 9, u8, "item flags")),

				weight:        get_value_as!(results, 10, u16, "weight"),
				buy_price:     get_value_as!(results, 11, u32, "buy price (silver)"),
				action_id:     get_value_as!(results, 12, u32, "action id"),

				combat_stats:  ItemCombatStats {
					max_phys_atk:    get_value_as!(results, 13, u16, "max phys attack"),
					min_phys_atk:    get_value_as!(results, 14, u16, "min phys attack"),
					phys_def:        get_value_as!(results, 15, u16, "physical defense"),
					accuracy:        get_value_as!(results, 16, u8, "accuracy"),
					dodge:           get_value_as!(results, 17, u8, "dodge"),
					magic_atk:       get_value_as!(results, 28, u16, "magic atk"),
					magic_def:       get_value_as!(results, 29, u16, "magic def"),
					atk_range:       get_value_as!(results, 30, u8, "attack range"),
					atk_speed:       get_value_as!(results, 31, u16, "attack speed")
				},

				hp_restored:   get_value_as!(results, 18, u16, "hp restored"),
				mp_restored:   get_value_as!(results, 19, u16, "mp restored"),
				amount:        get_value_as!(results, 20, u16, "amount"),
				amount_limit:  get_value_as!(results, 21, u16, "amount limit"),
				status:        get_value_as!(results, 22, u16, "status"),
				gem1:          get_value_as!(results, 23, u8, "gem 1"),
				gem2:          get_value_as!(results, 24, u8, "gem 2"),
				magic1:        get_value_as!(results, 25, u8, "magic 1"),
				magic2:        get_value_as!(results, 26, u8, "magic 2"),
				magic3:        get_value_as!(results, 27, u8, "magic 3"),

				fray_mode:     get_value_as!(results, 32, u32, "fray mode"),
				repair_mode:   get_value_as!(results, 33, u32, "repair mode"),
				type_mask:     get_value_as!(results, 34, u32, "type mask"),
				buy_cps_price: get_value_as!(results, 35, u16, "buy price (cps)"),
				type_name:     results[36].get_value().to_owned(),
				description:   results[37].get_value().to_owned(),
				unknown_1:     get_value_as!(results, 38, u8, "unknown 1")
			};

			Result::Ok(item)
		}
	}
}