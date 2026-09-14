// SPDX-FileCopyrightText: 2024 Softbear, Inc.

use crate::game::{ACTIVE_KEY, SURFACE_KEY};
use common::death_reason::DeathReason;
use common::entity::{EntityKind, EntitySubKind, EntityType};
use kodiak_client::{translate, PlayerAlias, RewardedAd, Translator};
use std::fmt::Display;

pub trait Mk48Phrases {
    fn entity_label(&self, entity_type: EntityType) -> String;
    fn entity_kind_name(&self, kind: EntityKind, sub_kind: EntitySubKind) -> String;
    fn entity_kind_hint(&self, kind: EntityKind, sub_kind: EntitySubKind) -> String;
    fn _rewarded_ad(&self, rewarded_ad: &RewardedAd) -> String;
    fn death_reason(&self, death_reason: &DeathReason) -> String;
    fn death_reason_collision(&self, thing: impl Display) -> String;
    fn death_reason_obstacle(&self, entity_type: EntityType) -> String;
    fn death_reason_weapon(&self, alias: PlayerAlias, entity_type: EntityType) -> String;
    //fn level(&self, level: u8) -> String;
    fn sensor_active_label(&self) -> String;
    fn sensor_active_hint(&self, sensors: &str) -> String;
    fn sensor_radar_label(&self) -> String;
    fn sensor_sonar_label(&self) -> String;
    fn ship_surface_label(&self) -> String;
    fn ship_surface_hint(&self) -> String;
    fn team_fleet_label(&self) -> String;
    fn team_fleet_name_placeholder(&self) -> String;
}

impl Mk48Phrases for Translator {
    fn entity_label(&self, entity_type: EntityType) -> String {
        let (id, fallback) = match entity_type {
            EntityType::Avenger => ("entity_avenger", "Мститель Ваня"),
            EntityType::E4N => ("entity_e4n", "Е4Н Егор"),
            EntityType::Harbin => ("entity_harbin", "Харбин Харитон"),
            EntityType::Ka25 => ("entity_ka25", "Кеша Ка-25"),
            EntityType::Kingfisher => ("entity_kingfisher", "Зимородок Зина"),
            EntityType::Seahawk => ("entity_seahawk", "Морской Ястреб Сева"),
            EntityType::SuperEtendard => ("entity_super_etendard", "Супер Эдик"),
            EntityType::SuperFrelon => ("entity_super_frelon", "Фрёля Супер"),
            EntityType::Akula => ("entity_akula", "Орешник"),
            EntityType::ArleighBurke => ("entity_arleigh_burke", "Арли Бурк Аркадий"),
            EntityType::Bismarck => ("entity_bismarck", "Бисмарк Борис"),
            EntityType::Buyan => ("entity_buyan", "Буян Боря"),
            EntityType::Clemenceau => ("entity_clemenceau", "Клеман Клементий"),
            EntityType::Dreadnought => ("entity_dreadnought", "Дредноут Дима"),
            EntityType::Dredger => ("entity_dredger", "Земснаряд Зина"),
            EntityType::Espana => ("entity_espana", "Эспаньола Елена"),
            EntityType::Essex => ("entity_essex", "Эссекс Эдик"),
            EntityType::FairmileD => ("entity_fairmile_d", "Фэйрмайл Даша"),
            EntityType::Fletcher => ("entity_fletcher", "Флетчер Фёдор"),
            EntityType::Freccia => ("entity_freccia", "Фречча Федя"),
            EntityType::Freedom => ("entity_freedom", "Фридом Фрося"),
            EntityType::G5 => ("entity_g5", "Г-5 Гена"),
            EntityType::Golf => ("entity_golf", "Гольф Гоша"),
            EntityType::Indiaman => ("entity_indiaman", "Индиец Иннокентий"),
            EntityType::Iowa => ("entity_iowa", "Айова Игорь"),
            EntityType::Kirov => ("entity_kirov", "Киров Кирилл"),
            EntityType::Kolkata => ("entity_kolkata", "Калькутта Костя"),
            EntityType::Komar => ("entity_komar", "Комар Коля"),
            EntityType::Leander => ("entity_leander", "Леандр Лёня"),
            EntityType::Lublin => ("entity_lublin", "Люблин Люба"),
            EntityType::Momi => ("entity_momi", "Моми Митя"),
            EntityType::Montana => ("entity_montana", "Монтана Мотя"),
            EntityType::Moskva => ("entity_moskva", "Москва Маша"),
            EntityType::Oberon => ("entity_oberon", "Оберон Олег"),
            EntityType::Ohio => ("entity_ohio", "Огайо Ося"),
            EntityType::Olympias => ("entity_olympias", "Олимпиас Оля"),
            EntityType::Osa => ("entity_osa", "Оса Оксана"),
            EntityType::Pt34 => ("entity_pt34", "ПТ-34 Паша"),
            EntityType::Seawolf => ("entity_seawolf", "Морской Волк Вова"),
            EntityType::Skipjack => ("entity_skipjack", "Скипджек Слава"),
            EntityType::Skjold => ("entity_skjold", "Шкильд Шура"),
            EntityType::Tanker => ("entity_tanker", "Танкер Толик"),
            EntityType::TerryFox => ("entity_terry_fox", "Терри Фокс Федя"),
            EntityType::TuoChiang => ("entity_tuo_chiang", "Туо-Чианг Толя"),
            EntityType::Town => ("entity_town", "Таун Тимур"),
            EntityType::Type055 => ("entity_type_055", "Тип-055 Тёма"),
            EntityType::Type212A => ("entity_type_212a", "Тип-212 Артём"),
            EntityType::TypeViic => ("entity_type_viic", "Тип VII-C Витя"),
            EntityType::Visby => ("entity_visby", "Висби Вася"),
            EntityType::Yamato => ("entity_yamato", "Ямато Яша"),
            EntityType::Yasen => ("entity_yasen", "Ясень Яна"),
            EntityType::Zubr => ("entity_zubr", "Зубр Захар"),
            EntityType::Zumwalt => ("entity_zumwalt", "Зумвальт Зоя"),
            EntityType::Barrel => ("entity_barrel", "Бочка Бодя"),
            EntityType::Coin => ("entity_coin", "Монетка Кузя"),
            EntityType::Crate => ("entity_crate", "Ящик Петрович"),
            EntityType::Scrap => ("entity_scrap", "Железяка Жорик"),
            EntityType::Brosok => ("entity_brosok", "Бросокыч"),
            EntityType::Mk70 => ("entity_mk70", "Моссик"),
            EntityType::Acacia => ("entity_acacia", "Куст Акакий"),
            EntityType::Hq => ("entity_hq", "Штаб Дядя Витя"),
            EntityType::OilPlatform => ("entity_oil_platform", "Нефтяная Тётя Нюра"),
            EntityType::_100Mm => ("entity_100mm", "Соточка"),
            EntityType::_2M3M => ("entity_2m3m", "Две-Три Миша"),
            EntityType::_38CmSkc34 => ("entity_38cm_skc34", "Тридцать восьмой Сергей"),
            EntityType::_45Type94 => ("entity_45_type94", "Сорокапятка Соня"),
            EntityType::_6Pounder => ("entity_6_pounder", "Шестипундик Фима"),
            EntityType::_88CmSkc35 => ("entity_88cm_skc35", "Восемь-восемь Олег"),
            EntityType::A190 => ("entity_a190", "Андрей-190"),
            EntityType::Ak130 => ("entity_ak130", "АК-130 Ксюша"),
            EntityType::Ansaldo => ("entity_ansaldo", "Ансальдо Аня"),
            EntityType::Bl6MkXxiii => ("entity_bl6_mk_xxiii", "Би-Эль Шестёрка"),
            EntityType::Bl6MkXxiiiX3 => ("entity_bl6_mk_xxiii_x3", "Би-Эль Тройняшка"),
            EntityType::Bofors57MmMk3 => ("entity_bofors_57_mk3", "Бофорс Боря"),
            EntityType::Crotale => ("entity_crotale", "Кроталь Костя"),
            EntityType::Hpj38 => ("entity_hpj38", "Эйч-Пи-Джей Женя"),
            EntityType::Mark12 => ("entity_mark12", "Марик Двенадцатый"),
            EntityType::Mark12X2 => ("entity_mark12_x2", "Марик Двойной"),
            EntityType::Mark49 => ("entity_mark49", "Марик ПВО"),
            EntityType::Mark51 => ("entity_mark51", "Марик Пятьдесят Первый"),
            EntityType::Mark7 => ("entity_mark7", "Марик Седьмой"),
            EntityType::MarkBViii => ("entity_mark_bviii", "Марик Восьмой"),
            EntityType::Ogon => ("entity_ogon", "Огонь Олег"),
            EntityType::OtoMelara76Mm => ("entity_oto_melara_76", "Отто Олегович"),
            EntityType::RatepKomar => ("entity_ratep_komar", "Комар-Капитан"),
            EntityType::Shtorm => ("entity_shtorm_turret", "Шторм Шурик"),
            EntityType::VickersMkH12In => ("entity_vickers_mkh12", "Виктор Двенадцатый"),
            EntityType::_127X680MmR => ("entity_127x680", "Стодвадцатьседьмой"),
            EntityType::_130X720MmR => ("entity_130x720", "Сто-тридцать Ксюша"),
            EntityType::_25X129MmR => ("entity_25x129", "Двадцатьпятка"),
            EntityType::_300X1400MmR => ("entity_300x1400", "Трёхсотка"),
            EntityType::_380X1700MmR => ("entity_380x1700", "Триста-восемьдесят"),
            EntityType::_458X1980MmR => ("entity_458x1980", "Четыреста-пятьдесят-восьмой"),
            EntityType::_57X441MmR => ("entity_57x441", "Пятьдесятседьмой"),
            EntityType::_76X636MmR => ("entity_76x636", "Семьдесятшестой"),
            EntityType::_82R => ("entity_82r", "Восемьдесят-два Рома"),
            EntityType::Asroc => ("entity_asroc", "Асрок Аркаша"),
            EntityType::Barak8 => ("entity_barak8", "Барак Боб"),
            EntityType::BrahMos => ("entity_brahmos", "Брамос Бронислав"),
            EntityType::CannonBall => ("entity_cannon_ball", "Чугунный Коля"),
            EntityType::Depositor => ("entity_depositor", "Закладчик Захар"),
            EntityType::Dm2A4 => ("entity_dm2a4", "ДМ-2 Даша"),
            EntityType::Essm => ("entity_essm", "Эсэм Эдик"),
            EntityType::Exocet => ("entity_exocet", "Экзосет Эмиль"),
            EntityType::Harpoon => ("entity_harpoon", "Гарпун Гриша"),
            EntityType::Hq9 => ("entity_hq9", "Эйч-Кью Девять"),
            EntityType::HsiungFengII => ("entity_hsiung_feng_ii", "Сюн-Фэн Двойка"),
            EntityType::HsiungFengIII => ("entity_hsiung_feng_iii", "Сюн-Фэн Тройка"),
            EntityType::Idas => ("entity_idas", "Идас Илья"),
            EntityType::Igla => ("entity_igla", "Игла Игорь"),
            EntityType::Kalibr => ("entity_kalibr", "Калибр Костя"),
            EntityType::Lrlap => ("entity_lrlap", "Элэрлап Лёша"),
            EntityType::Magic => ("entity_magic", "Мэджик Маша"),
            EntityType::Mark18 => ("entity_mark18", "Марик Торпедный"),
            EntityType::Mark48 => ("entity_mark48", "Марик Орешниковый"),
            EntityType::Mark54 => ("entity_mark54", "Марик Пятьдесят Четыре"),
            EntityType::Mark8 => ("entity_mark8", "Марик Снаряд"),
            EntityType::Mark9 => ("entity_mark9", "Марик Глубинный"),
            EntityType::Mistral => ("entity_mistral", "Мистраль Миша"),
            EntityType::Nsm => ("entity_nsm", "Морской НСМ Николай"),
            EntityType::Of45 => ("entity_of45", "ОФ-45 Олег"),
            EntityType::P15 => ("entity_p15", "Пятнадцатый Пётр"),
            EntityType::P700 => ("entity_p700", "Гранит Григорий"),
            EntityType::Rbs15 => ("entity_rbs15", "Эрбээс Роман"),
            EntityType::Rim116 => ("entity_rim116", "Рим-116 Рита"),
            EntityType::Rpk6 => ("entity_rpk6", "Водопад Витя"),
            EntityType::S300 => ("entity_s300", "С-300 Сергей"),
            EntityType::Set65 => ("entity_set65", "Сет-65 Степан"),
            EntityType::Tau2000 => ("entity_tau2000", "Тау-2000 Таня"),
            EntityType::TC2N => ("entity_tc2n", "ТеСэ-2 Никита"),
            EntityType::Tomahawk => ("entity_tomahawk", "Томагавк Тома"),
            EntityType::Torped45 => ("entity_torped45", "Торпедо Трофим"),
            EntityType::Type53 => ("entity_type53", "Тип-53 Тимофей"),
            EntityType::V611 => ("entity_v611", "Штормовой Ваня"),
            EntityType::Vt1 => ("entity_vt1", "Кроталь Витя"),
            EntityType::Wz0839 => ("entity_wz0839", "Мина Восьмого Года"),
            EntityType::Yj18 => ("entity_yj18", "Икс-Же-18 Яша"),
        };
        self.translate_phrase(Some(id), fallback, &[])
    }

    fn entity_kind_name(&self, kind: EntityKind, sub_kind: EntitySubKind) -> String {
        match (kind, sub_kind) {
            (EntityKind::Aircraft, EntitySubKind::Heli) => {
                translate!(self, "helicopter")
            }
            (EntityKind::Aircraft, EntitySubKind::Plane) => {
                translate!(self, "plane")
            }
            (EntityKind::Boat, EntitySubKind::Battleship) => {
                translate!(self, "battleship")
            }
            (EntityKind::Boat, EntitySubKind::Carrier) => {
                translate!(self, "aircraft carrier")
            }
            (EntityKind::Boat, EntitySubKind::Corvette) => {
                translate!(self, "corvette")
            }
            (EntityKind::Boat, EntitySubKind::Cruiser) => {
                translate!(self, "cruiser")
            }
            (EntityKind::Boat, EntitySubKind::Destroyer) => {
                translate!(self, "destroyer")
            }
            (EntityKind::Boat, EntitySubKind::Dreadnought) => {
                translate!(self, "dreadnought")
            }
            (EntityKind::Boat, EntitySubKind::Dredger) => {
                translate!(self, "dredger")
            }
            (EntityKind::Boat, EntitySubKind::Hovercraft) => {
                translate!(self, "hovercraft")
            }
            (EntityKind::Boat, EntitySubKind::Icebreaker) => {
                translate!(self, "icebreaker")
            }
            (EntityKind::Boat, EntitySubKind::Lcs) => {
                translate!(self, "littoral combat ship")
            }
            (EntityKind::Boat, EntitySubKind::Minelayer) => {
                translate!(self, "minelayer")
            }
            (EntityKind::Boat, EntitySubKind::Mtb) => {
                translate!(self, "motor-torpedo boat")
            }
            (EntityKind::Boat, EntitySubKind::MissileBoat) => {
                translate!(self, "missile boat")
            }
            (EntityKind::Boat, EntitySubKind::Pirate) => {
                translate!(self, "pirate")
            }
            (EntityKind::Boat, EntitySubKind::Ram) => {
                translate!(self, "ram")
            }
            (EntityKind::Boat, EntitySubKind::Submarine) => {
                translate!(self, "submarine")
            }
            (EntityKind::Boat, EntitySubKind::Tanker) => {
                translate!(self, "tanker")
            }
            (EntityKind::Decoy, EntitySubKind::Sonar) => {
                translate!(self, "sonar decoy")
            }
            (EntityKind::Collectible, EntitySubKind::Score) => {
                translate!(self, "entity_kind_score", "трофей")
            }
            (EntityKind::Obstacle, EntitySubKind::Tree) => {
                translate!(self, "entity_kind_tree", "дерево")
            }
            (EntityKind::Obstacle, EntitySubKind::Structure) => {
                translate!(self, "structure")
            }
            (EntityKind::Turret, EntitySubKind::Gun) => {
                translate!(self, "entity_kind_gun", "орудие")
            }
            (EntityKind::Turret, EntitySubKind::Missile) => {
                translate!(self, "entity_kind_missile_turret", "ракетная установка")
            }
            (EntityKind::Turret, EntitySubKind::Rocket) => {
                translate!(self, "entity_kind_rocket_turret", "ракетная установка")
            }
            (EntityKind::Turret, EntitySubKind::Sam) => {
                translate!(self, "entity_kind_sam_turret", "зенитная установка")
            }
            (EntityKind::Weapon, EntitySubKind::Depositor) => {
                translate!(self, "depositor")
            }
            (EntityKind::Weapon, EntitySubKind::DepthCharge) => {
                translate!(self, "depth charge")
            }
            (EntityKind::Weapon, EntitySubKind::Mine) => {
                translate!(self, "mine")
            }
            (EntityKind::Weapon, EntitySubKind::Missile) => {
                translate!(self, "missile")
            }
            (EntityKind::Weapon, EntitySubKind::RocketTorpedo) => {
                translate!(self, "rocket torpedo")
            }
            (EntityKind::Weapon, EntitySubKind::Rocket) => {
                translate!(self, "rocket")
            }
            (EntityKind::Weapon, EntitySubKind::Sam) => {
                translate!(self, "surface-to-air missile")
            }
            (EntityKind::Weapon, EntitySubKind::Shell) => {
                translate!(self, "shell")
            }
            (EntityKind::Weapon, EntitySubKind::Torpedo) => {
                translate!(self, "torpedo")
            }
            _ => {
                debug_assert!(false, "missing name for {:?}/{:?}", kind, sub_kind);
                "???".to_string()
            }
        }
    }

    fn entity_kind_hint(&self, kind: EntityKind, sub_kind: EntitySubKind) -> String {
        match (kind, sub_kind) {
            (EntityKind::Boat, EntitySubKind::Battleship) => {
                translate!(self, "Your ship has powerful guns and plenty of armor!")
            }
            (EntityKind::Boat, EntitySubKind::Carrier) => translate!(
                self,
                "Your ship can launch aircraft with weapons of their own!"
            ),
            (EntityKind::Boat, EntitySubKind::Corvette) => {
                translate!(self, "Your ship is small and difficult to hit!")
            }
            (EntityKind::Boat, EntitySubKind::Cruiser) => translate!(
                self,
                "Your ship is equipped with anti-ship and anti-submarine weapons!"
            ),
            (EntityKind::Boat, EntitySubKind::Destroyer) => {
                translate!(self, "Your ship is equipped with a variety of weapons!")
            }
            (EntityKind::Boat, EntitySubKind::Dreadnought) => {
                translate!(self, "Your ship has powerful cannons!")
            }
            (EntityKind::Boat, EntitySubKind::Dredger) => {
                translate!(self, "Your ship can create and destroy land!")
            }
            (EntityKind::Boat, EntitySubKind::Hovercraft) => {
                translate!(self, "Your boat can travel on both land and water!")
            }
            (EntityKind::Boat, EntitySubKind::Icebreaker) => {
                translate!(self, "Your ship can plow through ice sheets!")
            }
            (EntityKind::Boat, EntitySubKind::Lcs) => translate!(
                self,
                "Your boat can unleash deadly weapons from within small island groups!"
            ),
            (EntityKind::Boat, EntitySubKind::Minelayer) => {
                translate!(self, "Your boat can lay deadly magnetic mines")
            }
            (EntityKind::Boat, EntitySubKind::Mtb) => {
                translate!(self, "Your boat has torpedoes to sink other boats!")
            }
            (EntityKind::Boat, EntitySubKind::MissileBoat) => {
                translate!(self, "Your boat has missiles to sink other boats!")
            }
            (EntityKind::Boat, EntitySubKind::Ram) => {
                translate!(self, "Your boat is designed to ram other boats!")
            }
            (EntityKind::Boat, EntitySubKind::Submarine) => {
                translate!(self, "Your boat can deliver weapons from underwater!")
            }
            (EntityKind::Boat, EntitySubKind::Tanker) => {
                translate!(self, "Your boat gets double the value from oil barrels!")
            }
            (EntityKind::Boat, EntitySubKind::Pirate) => {
                translate!(self, "Yer ship be a sittin' duck")
            }
            _ => {
                debug_assert!(false, "missing hint for {:?}/{:?}", kind, sub_kind);
                "???".to_string()
            }
        }
    }

    fn _rewarded_ad(&self, rewarded_ad: &RewardedAd) -> String {
        match rewarded_ad {
            RewardedAd::Available { .. } => {
                translate!(self, "Unlock bonus content")
            }
            RewardedAd::Watching { .. } => {
                translate!(self, "Requesting ad...")
            }
            RewardedAd::Watched { .. } => translate!(self, "Unlocked!"),
            _ => translate!(self, "Ad error"),
        }
    }

    fn death_reason(&self, death_reason: &DeathReason) -> String {
        match death_reason {
            &DeathReason::Boat { killer_alias, .. } => self.death_reason_collision(killer_alias),
            DeathReason::Border => {
                translate!(self, "Crashed into the border!")
            }
            &DeathReason::Obstacle(entity_type) => self.death_reason_obstacle(entity_type),
            &DeathReason::Ram {
                killer_alias: alias,
                ..
            } => translate!(self, "Rammed by {alias}!"),
            DeathReason::Terrain => {
                translate!(self, "Crashed into the ground!")
            }
            &DeathReason::Weapon {
                killer_alias,
                weapon_type,
                ..
            } => self.death_reason_weapon(killer_alias, weapon_type),
            _ => {
                debug_assert!(false, "unexpected {:?}", death_reason);
                String::from("Died of unexplained causes.")
            }
        }
    }

    fn death_reason_collision(&self, thing: impl Display) -> String {
        translate!(self, "Crashed into {thing}!")
    }

    fn death_reason_obstacle(&self, entity_type: EntityType) -> String {
        self.death_reason_collision(self.entity_label(entity_type))
    }

    fn death_reason_weapon(&self, alias: PlayerAlias, entity_type: EntityType) -> String {
        let data = entity_type.data();
        let weapon = self.entity_kind_name(data.kind, data.sub_kind);
        translate!(self, "Sunk by {alias} with a {weapon}!")
    }

    /*
    fn level(&self, level: u8) -> String {
        translate!(self, "Level {level}")
    }
    */

    fn sensor_active_label(&self) -> String {
        translate!(self, "Active sensors")
    }

    fn sensor_active_hint(&self, sensors: &str) -> String {
        let key = ACTIVE_KEY;
        translate!(
            self,
            "({key}) Active {sensors} helps you see more, but may also give away your position"
        )
    }

    fn sensor_radar_label(&self) -> String {
        translate!(self, "Radar")
    }

    fn sensor_sonar_label(&self) -> String {
        translate!(self, "Sonar")
    }

    fn ship_surface_label(&self) -> String {
        translate!(self, "Surface")
    }

    fn ship_surface_hint(&self) -> String {
        let key = SURFACE_KEY;
        translate!(self, "({key}) You can surface your ship whenever you want, but diving is sometimes limited by the depth of the water")
    }

    fn team_fleet_label(&self) -> String {
        translate!(self, "Fleet")
    }

    fn team_fleet_name_placeholder(&self) -> String {
        translate!(self, "Fleet name")
    }
}
