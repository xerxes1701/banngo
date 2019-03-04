use rand::{seq::SliceRandom, thread_rng, Rng};
use std::io;
use colored::Colorize;
use wana_kana::to_hiragana::*;

struct Category {
    pub name: &'static str,
    pub counters: [&'static str; 10]
}

const CATEGORIES: &[Category] = &[
    Category {
        name: "つ",
        counters: ["ひとつ", "ふたつ", "みっつ", "よっつ", "いつつ", "むっつ", "ななつ", "やっつ", "ここのつ", "とお"] 
    },
    Category {
        name: "日",
        counters: ["ついたち", "ふつか", "みっか", "よっか", "いつか", "むいか", "なのか", "ようか", "ここのか", "とおか"]
    },
    Category {
        name: "人",
        counters: ["ひとり", "ふたり", "さんにん", "よにん", "ごにん", "ろくにん", "しちにん", "はちにん", "きゅうにん", "じゅうにん"]
    },
    Category {
        name: "時",
        counters: ["いちじ", "にじ", "さんじ", "よじ", "ごじ", "ろくじ", "しちじ", "はちじ", "くじ", "じゅうじ"]
    },
    Category {
        name: "分",
        counters: ["いっぷん", "にふん", "さんぷん", "よんぷん", "ごふん", "ろっぷん", "しちふん", "はっぷん", "くふん", "じゅっぷん"]
    },
];

const NUMBERS: [&str; 10] = ["一", "二", "三", "四", "五", "六", "七", "八", "九", "十"];

fn main() {
    let mut rng = thread_rng();
    let mut buffer = String::new();

    loop {
        let category = CATEGORIES.choose(&mut rng).unwrap();

        let number = rng.gen_range(0, 10);

        println!("{}{}？", NUMBERS[number].cyan(), category.name.cyan());

        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let actual = to_hiragana(buffer.trim_end());

        let expected = category.counters[number];
        if expected == actual{
            println!("{}", "正しい！".green());
        } else {
            println!("{} {}", "違い!：".red(), expected);
        }
    }
}
