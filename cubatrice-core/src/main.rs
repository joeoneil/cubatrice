#![allow(unused)]

use std::{collections::HashMap, env, fs, io};

use cubatrice_core::{
    entity::{
        colony::{Colony, ColonyID, ColonyType},
        converter::{Arrow, Convert, Converter},
        cube::CubeType,
        faction::{FactionType, StartingResources},
        technology::{ConverterPrototype, TechID, Technology},
        Item, Token,
    },
    state::GameData,
    Fraction, DATA_DIR,
};

fn main() {
    let gd = GameData::preloaded().unwrap();

    for i in 0..6 {
        print!("\x1b[2J\x1b[1;1H");
        println!("\x1b[1mConfluence {}\x1b[0m\n", i + 1);
        let mut hm: HashMap<TechID, (Fraction, Fraction)> = HashMap::new();
        for (tid, p) in &gd.tech_prototype {
            hm.insert(
                *tid,
                (
                    p.input_value_adjusted(Fraction::new(7, 5), 6 - i),
                    p.output_value_adjusted(Fraction::new(7, 5), 6 - i),
                ),
            );
        }
        let mut ord = hm.into_iter().collect::<Vec<_>>();
        ord.sort_by(|b, a| {
            (a.1 .1 / a.1 .0)
                .value()
                .partial_cmp(&(b.1 .1 / b.1 .0).value())
                .unwrap()
        });
        for c in ord {
            let int = ((c.1 .1 / c.1 .0).value() - 1.0) * 100.0;
            let tech = gd
                .tech
                .get(&TechID(if c.0 .0 > 100 { c.0 .0 - 100 } else { c.0 .0 }))
                .unwrap();
            if c.0 .0 > 114 {
                continue;
            }
            let tn = tech.invents.as_ref().unwrap();
            let tier = tech.tier;
            println!(
                "[{}{}] {:40} | {:8} | {:.2} -> {:.2}",
                tier,
                if c.0 .0 > 100 { "+" } else { " " },
                if c.0 .0 > 100 {
                    format!("Upgraded {}", tn)
                } else {
                    format!("{}", tn)
                },
                format!("{}{:.2}%", if int > 0.0 { "+" } else { "" }, int),
                c.1 .0.value(),
                c.1 .1.value(),
            );
        }
        io::stdin().read_line(&mut String::new());
    }
}
