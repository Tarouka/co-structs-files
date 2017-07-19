pub struct BaseMonster {
	pub name:           String,
	pub size_add:       u8,
	pub zoom_pct:       u16,
	pub max_life:       u32,
	pub lvl:            u16,
	pub born_action:    u16,
	pub act_res_ctrl:   u8,
	pub asb:            u8,
	pub adb:            u8,
	pub body_type:      u8,
	pub type_id:        String,
	pub anti_type:      u8,
	pub armet:          u8,
	pub armet_color:    u8,
	pub r_weapon:       u8,
	pub l_weapon:       u8,
	pub l_weapon_color: u8,
	pub misc:           u8,
	pub mount:          u8,
	pub battle_lvl:     u8,
	pub extra_xp:       u8,
	pub stc_type:       u8,
	pub born_effect:    String,
	pub born_sound:     String
}

mod serializer {
    use super::BaseMonster;
    use serializer::{IniEntrySerializer, StringSerializableStructure};

    
    impl StringSerializableStructure for BaseMonster {
		fn serialize(&self) -> String {
			let mut serializer: IniEntrySerializer = IniEntrySerializer::new();

			serializer.add_section(&self.name);
			serializer.add_entry("SizeAdd", &self.size_add);
			serializer.add_entry("ZoomPercent", &self.zoom_pct);
			serializer.add_entry("MaxLife", &self.max_life);
			serializer.add_entry("Level", &self.lvl);
			serializer.add_entry("BornAction", &self.born_action);
			serializer.add_entry("ActResCtrl", &self.act_res_ctrl);
			serializer.add_entry("ASB", &self.asb);
			serializer.add_entry("ADB", &self.adb);
			serializer.add_entry("BodyType", &self.body_type);
			serializer.add_entry("TypeID", &self.type_id);
			serializer.add_entry("AntiType", &self.anti_type);
			serializer.add_entry("Armet", &self.armet);
			serializer.add_entry("ArmetColor", &self.armet_color);
			serializer.add_entry("RWeapon", &self.r_weapon);
			serializer.add_entry("LWeapon", &self.l_weapon);
			serializer.add_entry("LWeaponColor", &self.l_weapon_color);
			serializer.add_entry("Misc", &self.misc);
			serializer.add_entry("Mount", &self.mount);
			serializer.add_entry("BattleLev", &self.battle_lvl);
			serializer.add_entry("ExtraExp", &self.extra_xp);
			serializer.add_entry("StcType", &self.stc_type);
			serializer.add_entry("BornEffect", &self.born_effect);
			serializer.add_entry("BornSound", &self.born_sound);

			serializer.get_buffer()
		}
    }
}

mod parser {
	use super::BaseMonster;
	use parser::{IniEntryParser, IniEntryParserError, IniEntryParseable};

	impl IniEntryParseable<BaseMonster> for BaseMonster {
		fn from_section_string(section: &String) -> Result<BaseMonster, IniEntryParserError> {
			match IniEntryParser::new_from_str(section.clone()) {
				Result::Ok(parser) => {
					let monster = BaseMonster {
						name:             parser.get_section(),

						size_add:         parser.get_entry_as("SizeAdd")?,
						zoom_pct:         parser.get_entry_as("ZoomPercent")?,
						max_life:         parser.get_entry_as("MaxLife")?,
						lvl:              parser.get_entry_as("Level")?,
						born_action:      parser.get_entry_as("BornAction")?,
						act_res_ctrl:     parser.get_entry_as("ActResCtrl")?,
						asb:              parser.get_entry_as("ASB")?,
						adb:              parser.get_entry_as("ADB")?,
						body_type:        parser.get_entry_as("BodyType")?,
						type_id:          parser.get_entry("TypeID")?,
						anti_type:        parser.get_entry_as("AntiType")?,
						armet:            parser.get_entry_as("Armet")?,
						armet_color:      parser.get_entry_as("ArmetColor")?,
						r_weapon:         parser.get_entry_as("RWeapon")?,
						l_weapon:         parser.get_entry_as("LWeapon")?,
						l_weapon_color:   parser.get_entry_as("LWeaponColor")?,
						misc:             parser.get_entry_as("Misc")?,
						mount:            parser.get_entry_as("Mount")?,
						battle_lvl:       parser.get_entry_as("BattleLev")?,
						extra_xp:         parser.get_entry_as("ExtraExp")?,
						stc_type:         parser.get_entry_as("StcType")?,
						born_effect:      parser.get_entry("BornEffect")?,
						born_sound:       parser.get_entry("BornSound")?
					};

					return Result::Ok(monster);
				},

				Result::Err(err) => {
					return Result::Err(err);
				}
			}
		}
	}
}