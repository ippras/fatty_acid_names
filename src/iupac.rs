use lipid::r#struct::fatty_acid::FattyAcid;
use std::fmt::{self, Formatter, from_fn};

// #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
// pub struct Unsaturated {
//     pub index: Option<u8>,
//     pub triple: Option<bool>,
//     pub parity: Option<bool>,
// }

pub fn format_iupac(fatty_acid: FattyAcid) -> String {
    // Сортируем связи по индексу (локанту) для правильного порядка
    fatty_acid.unsaturated.sort_by_key(|u| u.index);

    // Используем from_fn для формирования строки "на лету", без промежуточных Vec и String
    let display = from_fn(move |f| {
        // 1. Определяем корень
        let root = format_carbons(carbons);

        // Считаем количество двойных и тройных связей
        let ene_count = fatty_acid
            .unsaturated
            .iter()
            .filter(|unsaturation| unsaturation.triple.is_some_and(|triple| !triple))
            .count();
        let yne_count = fatty_acid
            .unsaturated
            .iter()
            .filter(|unsaturation| unsaturation.triple.is_some_and(|triple| triple))
            .count();

        // 2. Формируем префикс стереохимии (например: "(6Z,9Z,12Z)-")
        let mut stereo_iter = fatty_acid
            .unsaturated
            .iter()
            .filter_map(|u| Some((u.index?, u.parity?)))
            .peekable();

        if stereo_iter.peek().is_some() {
            write!(f, "(")?;
            let mut first = true;
            for (index, parity) in stereo_iter {
                if !first {
                    write!(f, ",")?;
                }
                let p = if parity { 'E' } else { 'Z' };
                write!(f, "{index}{p}")?;
                first = false;
            }
            write!(f, ")-")?;
        }

        // Если связей нет — это насыщенная кислота
        if ene_count == 0 && yne_count == 0 {
            return write!(f, "{root}anoic");
        }

        // 3. Добавляем корень и соединительную 'a'
        let a = if ene_count > 1 || (ene_count == 0 && yne_count > 1) {
            "a"
        } else {
            ""
        };
        write!(f, "{root}{a}")?;

        // Вспомогательное замыкание для форматирования локантов (например: "-6,9,12-")
        let write_locants = |f: &mut Formatter<'_>, is_triple: bool| -> fmt::Result {
            let mut locants_iter = fatty_acid
                .unsaturated
                .iter()
                .filter(|u| u.triple.unwrap_or(false) == is_triple)
                .filter_map(|u| u.index)
                .peekable();

            if locants_iter.peek().is_some() {
                write!(f, "-")?;
                let mut first = true;
                for idx in locants_iter {
                    if !first {
                        write!(f, ",")?;
                    }
                    write!(f, "{idx}")?;
                    first = false;
                }
                write!(f, "-")?;
            }
            Ok(())
        };

        // 4. Добавляем двойные связи (ene)
        if ene_count > 0 {
            write_locants(f, false)?;
            write!(f, "{}", format_multiplier(ene_count))?;
            if yne_count > 0 {
                write!(f, "en")?; // Если дальше идут тройные связи
            } else {
                write!(f, "enoic")?;
            }
        }

        // 5. Добавляем тройные связи (yne)
        if yne_count > 0 {
            write_locants(f, true)?;
            write!(f, "{}ynoic", format_multiplier(yne_count))?;
        }

        Ok(())
    });

    // Превращаем наш from_fn в итоговую строку за одну аллокацию
    display.to_string()
}

pub fn format_carbons(n: u8) -> &'static str {
    match n {
        1 => "meth",
        2 => "eth",
        3 => "prop",
        4 => "but",
        5 => "pent",
        6 => "hex",
        7 => "hept",
        8 => "oct",
        9 => "non",
        10 => "dec",
        11 => "undec",
        12 => "dodec",
        13 => "tridec",
        14 => "tetradec",
        15 => "pentadec",
        16 => "hexadec",
        17 => "heptadec",
        18 => "octadec",
        19 => "nonadec",
        20 => "icos",
        21 => "henicos",
        22 => "docos",
        23 => "tricos",
        24 => "tetracos",
        25 => "pentacos",
        26 => "hexacos",
        27 => "heptacos",
        28 => "octacos",
        29 => "nonacos",
        30 => "triacont",
        31 => "hentriacont",
        32 => "dotriacont",
        33 => "tritriacont",
        34 => "tetratriacont",
        35 => "pentatriacont",
        36 => "hexatriacont",
        37 => "heptatriacont",
        38 => "octatriacont",
        39 => "nonatriacont",
        40 => "tetracont",
        _ => unimplemented!(),
    }
}

/// Вспомогательная функция для множителей ИЮПАК (IUPAC P-14.2.1).
pub fn format_multiplier(n: usize) -> &'static str {
    match n {
        1 => "",
        2 => "di",
        3 => "tri",
        4 => "tetra",
        5 => "penta",
        6 => "hexa",
        7 => "hepta",
        8 => "octa",
        9 => "nona",
        10 => "deca",
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // assert_eq!("undeca", &multiplier(11));
    // assert_eq!("dodeca", &multiplier(12));
    // assert_eq!("henicosa", &multiplier(21));
    // assert_eq!("dotriaconta", &multiplier(32));
    // assert_eq!("dotriacontahecta", &multiplier(132));
    // assert_eq!("octatetracontapentacta", &multiplier(548));
    // assert_eq!("heptahexacontadictanonalia", &multiplier(9267));
    // #[test]
    // fn get_multiplier() {
    //     // assert_eq!(multiplier(1), "mono, hen");
    //     assert_eq!(format_multiplier(11), "undeca");
    //     assert_eq!(format_multiplier(101), "henhecta");
    //     assert_eq!(format_multiplier(1001), "henkilla");
    //     assert_eq!(format_multiplier(2), "di,do");
    //     assert_eq!(format_multiplier(20), "icosa");
    //     assert_eq!(format_multiplier(200), "dicta");
    //     assert_eq!(format_multiplier(2000), "dilia");
    //     assert_eq!(format_multiplier(3), "tri");
    //     assert_eq!(format_multiplier(30), "triaconta");
    //     assert_eq!(format_multiplier(300), "tricta");
    //     assert_eq!(format_multiplier(3000), "trilia");
    //     assert_eq!(format_multiplier(4), "tetra");
    //     assert_eq!(format_multiplier(40), "tetraconta");
    //     assert_eq!(format_multiplier(400), "tetracta");
    //     assert_eq!(format_multiplier(4000), "tetralia");
    //     assert_eq!(format_multiplier(5), "penta");
    //     assert_eq!(format_multiplier(50), "pentaconta");
    //     assert_eq!(format_multiplier(500), "pentacta");
    //     assert_eq!(format_multiplier(5000), "pentalia");
    //     assert_eq!(format_multiplier(6), "hexa");
    //     assert_eq!(format_multiplier(60), "hexaconta");
    //     assert_eq!(format_multiplier(600), "hexacta");
    //     assert_eq!(format_multiplier(6000), "hexalia");
    //     assert_eq!(format_multiplier(7), "hepta");
    //     assert_eq!(format_multiplier(70), "heptaconta");
    //     assert_eq!(format_multiplier(700), "heptacta");
    //     assert_eq!(format_multiplier(7000), "heptalia");
    //     assert_eq!(format_multiplier(8), "octa");
    //     assert_eq!(format_multiplier(80), "octaconta");
    //     assert_eq!(format_multiplier(800), "octacta");
    //     assert_eq!(format_multiplier(8000), "octalia");
    //     assert_eq!(format_multiplier(9), "nona");
    //     assert_eq!(format_multiplier(90), "nonaconta");
    //     assert_eq!(format_multiplier(900), "nonacta");
    //     assert_eq!(format_multiplier(9000), "nonalia");
    //     assert_eq!(format_multiplier(10), "deca");
    //     assert_eq!(format_multiplier(100), "hecta");
    //     assert_eq!(format_multiplier(1000), "kilia");

    //     // assert_eq!("tetradeca", multiplier(14));
    //     assert_eq!("henicosa", format_multiplier(21));
    //     // assert_eq!("docosa", multiplier(22));
    //     // assert_eq!("tricosa", multiplier(23));
    //     // assert_eq!("tetracosa", multiplier(24));
    //     // assert_eq!("hentetraconta", multiplier(41));
    //     // assert_eq!("dopentaconta", multiplier(52));
    //     // assert_eq!("undecahecta", multiplier(111));
    //     // assert_eq!("trihexacontatricta", multiplier(363));
    //     // assert_eq!("hexaoctacontatetracta", multiplier(486));
    // }

    #[test]
    fn c20u3c8c11c14() {
        assert_eq!(
            format_iupac(FattyAcid {
                carbon: 20,
                unsaturated: vec![
                    Unsaturated {
                        index: Some(8),
                        triple: Some(false),
                        parity: Some(false),
                    },
                    Unsaturated {
                        index: Some(11),
                        triple: Some(false),
                        parity: Some(false),
                    },
                    Unsaturated {
                        index: Some(14),
                        triple: Some(false),
                        parity: Some(false),
                    },
                ],
            }),
            "(8Z,11Z,14Z)-icosa-8,11,14-trienoic"
        );
    }

    #[test]
    fn c20u3c5c8c11c14() {
        assert_eq!(
            format_iupac(FattyAcid {
                carbon: 20,
                unsaturated: vec![
                    Unsaturated {
                        index: Some(5),
                        triple: Some(false),
                        parity: Some(false),
                    },
                    Unsaturated {
                        index: Some(8),
                        triple: Some(false),
                        parity: Some(false),
                    },
                    Unsaturated {
                        index: Some(11),
                        triple: Some(false),
                        parity: Some(false),
                    },
                    Unsaturated {
                        index: Some(14),
                        triple: Some(false),
                        parity: Some(false),
                    },
                ],
            }),
            "(5Z,8Z,11Z,14Z)-icosa-5,8,11,14-tetraenoic"
        );
    }

    #[test]
    fn c18u3c6c9c12() {
        assert_eq!(
            format_iupac(FattyAcid {
                carbon: 18,
                unsaturated: vec![
                    Unsaturated {
                        index: Some(6),
                        triple: Some(false),
                        parity: Some(false),
                    },
                    Unsaturated {
                        index: Some(9),
                        triple: Some(false),
                        parity: Some(false),
                    },
                    Unsaturated {
                        index: Some(12),
                        triple: Some(false),
                        parity: Some(false),
                    },
                ],
            }),
            "(6Z,9Z,12Z)-octadeca-6,9,12-trienoic"
        );
    }
}
