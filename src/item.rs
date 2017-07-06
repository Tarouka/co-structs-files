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