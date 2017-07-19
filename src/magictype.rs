pub struct BaseMagicTypeEntry {
    pub id:                      u32,
    pub action_sort:             u8,
    pub skill_name:              String,
    pub is_offensive_on_use:     bool,
    pub is_ground_targeted:      bool,
    pub is_multi_target:         bool,
    pub targets_flags:           MagicTargetFlags,
    pub requirements:            MagicRequirements,
    pub effects:                 MagicEffects,
    pub costs:                   MagicCosts,
    pub skill_lvl:               u8,
    pub power:                   i32,
    pub intone_effect_duration:  u32,
    pub accuracy:                u8,
    pub time:                    u32,
    pub range:                   u32,
    pub max_distance:            u8,
    pub status:                  u64,
    pub skill_type:              u8,
    pub active_time:             u32,
    pub auto_active:             u16,
    pub floor_attribute:         u32,
    pub is_auto_learned:         bool,
    pub auto_learn_lvl:          u8,
    pub drop_weapon:             u32,
    pub hits_with_weapon:        u8,
    pub next_skill_id_auto_cast: u32,
    pub use_delay:               u32,
    pub use_item_num:            u8,
    pub sender_action:           u32,
    pub short_desc:              String,
    pub desc:                    String,
    pub target_delay:            u32,
    pub screen_represent:        bool,
    pub is_usable_in_market:     bool,
    pub is_staggering:           bool
}

pub struct MagicTargetFlags {
    pub is_body_target:          bool,
    pub is_passive_target:       bool,
    pub is_terrain_target:       bool,
    pub is_none_target:          bool,
    pub is_self_target:          bool
}

impl MagicTargetFlags {
    pub fn from_byte(flags: &u8) -> MagicTargetFlags {
        MagicTargetFlags {
            is_body_target:      (flags & 0x10) > 0,
            is_passive_target:   (flags & 0x08) > 0,
            is_terrain_target:   (flags & 0x04) > 0,
            is_none_target:      (flags & 0x02) > 0,
            is_self_target:      (flags & 0x01) > 0
        }
    }

    pub fn get_as_byte(&self) -> u8 {
        let mut flag: u8 = 0;

        flag |= if self.is_self_target { 0x01 } else { 0x00 };
        flag |= if self.is_none_target { 0x02 } else { 0x00 };
        flag |= if self.is_terrain_target { 0x04 } else { 0x00 };
        flag |= if self.is_passive_target { 0x08 } else { 0x00 };
        flag |= if self.is_body_target { 0x10 } else { 0x00 };

        flag
    }
}

pub struct MagicRequirements {
    pub job_required:            u32,
    pub xp_required:             u64,
    pub lvl_required:            u8,
    pub weapon_required:         u16,
    pub uses_item:               u8
}

pub struct MagicEffects {
    pub intone_effect:           String,
    pub intone_sfx:              String,
    pub sender_effect:           String,
    pub sender_sfx:              String,
    pub ground_effect:           String,
    pub trace_effect:            String,
    pub target_effect:           String,
    pub target_sfx:              String
}

pub struct MagicCosts {
    pub mp_cost:                 u16,
    pub stamina_cost:            u8
}

impl MagicCosts {
    pub fn is_mp_sufficient(&self, mp: &u16) -> bool { *mp >= self.mp_cost }
    pub fn is_stamina_sufficient(&self, stamina: &u8) -> bool { *stamina >= self.stamina_cost }
}

mod serializer {
    use ::magictype::BaseMagicTypeEntry;
    use ::serializer::{StringSerializableStructure, SpaceSeparatedSerializer};

    impl StringSerializableStructure for BaseMagicTypeEntry {
        fn serialize(&self) -> String {
            let mut serializer = SpaceSeparatedSerializer::new();

            serializer.push_value(&self.id);
            serializer.push_value(&self.action_sort);
            serializer.push_value(&self.skill_name);
            serializer.push_value(&self.is_offensive_on_use);
            serializer.push_value(&self.is_ground_targeted);
            serializer.push_value(&self.is_multi_target);
            serializer.push_value(&self.targets_flags.get_as_byte());
            serializer.push_value(&self.skill_lvl);
            serializer.push_value(&self.costs.mp_cost);
            serializer.push_value(&self.power);
            serializer.push_value(&self.intone_effect_duration);
            serializer.push_value(&self.accuracy);
            serializer.push_value(&self.time);
            serializer.push_value(&self.range);
            serializer.push_value(&self.max_distance);
            serializer.push_value(&self.status);
            serializer.push_value(&self.requirements.job_required);
            serializer.push_value(&self.requirements.xp_required);
            serializer.push_value(&self.requirements.lvl_required);
            serializer.push_value(&self.skill_type);
            serializer.push_value(&self.requirements.weapon_required);
            serializer.push_value(&self.active_time);
            serializer.push_value(&self.auto_active);
            serializer.push_value(&self.floor_attribute);
            serializer.push_value(&self.is_auto_learned);
            serializer.push_value(&self.auto_learn_lvl);
            serializer.push_value(&self.drop_weapon);
            serializer.push_value(&self.costs.stamina_cost);
            serializer.push_value(&self.hits_with_weapon);
            serializer.push_value(&self.requirements.uses_item);
            serializer.push_value(&self.next_skill_id_auto_cast);
            serializer.push_value(&self.use_delay);
            serializer.push_value(&self.use_item_num);
            serializer.push_value(&self.sender_action);
            serializer.push_value(&self.short_desc);
            serializer.push_value(&self.desc);
            serializer.push_value(&self.effects.intone_effect);
            serializer.push_value(&self.effects.intone_sfx);
            serializer.push_value(&self.effects.sender_effect);
            serializer.push_value(&self.effects.sender_sfx);
            serializer.push_value(&self.target_delay);
            serializer.push_value(&self.effects.target_effect);
            serializer.push_value(&self.effects.target_sfx);
            serializer.push_value(&self.effects.ground_effect);
            serializer.push_value(&self.effects.trace_effect);
            serializer.push_value(&self.screen_represent);
            serializer.push_value(&self.is_usable_in_market);
            serializer.push_value(&self.is_staggering);

            serializer.get_buffer()
        }
    }
}

mod parser {
    use super::*;
    use parser::*;

    impl SpaceSeparatedParseable<BaseMagicTypeEntry> for BaseMagicTypeEntry {
        fn from_line(line: &String) -> Result<BaseMagicTypeEntry, SpaceSeparatedParserError> {
            let results = get_space_separated_values(line);

            if results.len() < 48 {
                return Result::Err(SpaceSeparatedParserError::MissingAtIndex(results.len()));
            }

            let magic_type_entry = BaseMagicTypeEntry {
                id:                          get_value_as!(results, 0, u32, "id"),
                action_sort:                 get_value_as!(results, 1, u8, "action sort"),
                skill_name:                  results[2].get_value().to_owned(),
                is_offensive_on_use:         get_value_as!(results, 3, u8, "is offensive on use") == 1,
                is_ground_targeted:          get_value_as!(results, 4, u8, "is ground targeted") == 1,
                is_multi_target:             get_value_as!(results, 5, u8, "is multi target") == 1,

                targets_flags:               MagicTargetFlags::from_byte(&get_value_as!(results, 6, u8, "target flags")),

                costs:                       MagicCosts {
                    mp_cost:                   get_value_as!(results, 8, u16, "mp cost"),
                    stamina_cost:              get_value_as!(results, 27, u8, "stamina cost")
                },

                requirements:                MagicRequirements {
                    job_required:              get_value_as!(results, 16, u32, "job required"),
                    xp_required:               get_value_as!(results, 17, u64, "xp required"),
                    lvl_required:              get_value_as!(results, 18, u8, "level required"),
                    weapon_required:           get_value_as!(results, 20, u16, "weapon required"),
                    uses_item:                 get_value_as!(results, 29, u8, "uses item")
                },

                effects:                     MagicEffects {
                    intone_effect:             results[36].get_value().to_owned(),
                    intone_sfx:                results[37].get_value().to_owned(),
                    sender_effect:             results[38].get_value().to_owned(),
                    sender_sfx:                results[39].get_value().to_owned(),
                    target_effect:             results[41].get_value().to_owned(),
                    target_sfx:                results[42].get_value().to_owned(),
                    ground_effect:             results[43].get_value().to_owned(),
                    trace_effect:              results[44].get_value().to_owned()
                },

                skill_lvl:                       get_value_as!(results, 7, u8, "skill lvl"),
                power:                           get_value_as!(results, 9, i32, "power"),
                intone_effect_duration:          get_value_as!(results, 10, u32, "intone effect duration"),
                accuracy:                        get_value_as!(results, 11, u8, "accuracy"),
                time:                            get_value_as!(results, 12, u32, "time"),
                range:                           get_value_as!(results, 13, u32, "range"),
                max_distance:                    get_value_as!(results, 14, u8, "max distance"),
                status:                          get_value_as!(results, 15, u64, "status"),

                skill_type:                      get_value_as!(results, 19, u8, "skill type"),
                active_time:                     get_value_as!(results, 21, u32, "active time"),
                auto_active:                     get_value_as!(results, 22, u16, "auto active"),
                floor_attribute:                 get_value_as!(results, 23, u32, "floor attribute"),
                is_auto_learned:                 get_value_as!(results, 24, u8, "auto learned") == 1,
                auto_learn_lvl:                  get_value_as!(results, 25, u8, "auto learn level"),
                drop_weapon:                     get_value_as!(results, 26, u32, "drop weapon"),
                hits_with_weapon:                get_value_as!(results, 28, u8, "hits with weapon"),
                next_skill_id_auto_cast:         get_value_as!(results, 30, u32, "next skill id (auto casted)"),
                use_delay:                       get_value_as!(results, 31, u32, "use delay"),
                use_item_num:                    get_value_as!(results, 32, u8, "use item number"),
                sender_action:                   get_value_as!(results, 33, u32, "sender action"),
                short_desc:                      results[34].get_value().to_owned(),
                desc:                            results[35].get_value().to_owned(),

                target_delay:                    get_value_as!(results, 40, u32, "target delay"),

                screen_represent:                get_value_as!(results, 45, u8, "screen represent") == 1,
                is_usable_in_market:             get_value_as!(results, 46, u8, "is usable in market") == 1,
                is_staggering:                   get_value_as!(results, 47, u8, "is staggering") == 1
            };

            Result::Ok(magic_type_entry)
        }
    }
}