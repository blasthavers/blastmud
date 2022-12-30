use once_cell::sync::OnceCell;

struct PluralRule<'l> {
    match_suffix: &'l str,
    drop: usize,
    append_suffix: &'l str,
}

pub fn pluralise(input: &str) -> String {
    static PLURAL_RULES: OnceCell<Vec<PluralRule>> = OnceCell::new();
    let plural_rules = PLURAL_RULES.get_or_init(|| vec!(
        PluralRule { match_suffix: "foot", drop: 3, append_suffix: "eet" },
        PluralRule { match_suffix: "tooth", drop: 4, append_suffix: "eeth" },
        PluralRule { match_suffix: "man", drop: 2, append_suffix: "en" },
        PluralRule { match_suffix: "mouse", drop: 4, append_suffix: "ice" },
        PluralRule { match_suffix: "louse", drop: 4, append_suffix: "ice" },
        PluralRule { match_suffix: "fish", drop: 0, append_suffix: "" },
        PluralRule { match_suffix: "sheep", drop: 0, append_suffix: "" },
        PluralRule { match_suffix: "deer", drop: 0, append_suffix: "" },
        PluralRule { match_suffix: "pox", drop: 0, append_suffix: "" },
        PluralRule { match_suffix: "cis", drop: 2, append_suffix: "es" },
        PluralRule { match_suffix: "sis", drop: 2, append_suffix: "es" },
        PluralRule { match_suffix: "xis", drop: 2, append_suffix: "es" },
        PluralRule { match_suffix: "ss", drop: 0, append_suffix: "es" },
        PluralRule { match_suffix: "ch", drop: 0, append_suffix: "es" },
        PluralRule { match_suffix: "sh", drop: 0, append_suffix: "es" },
        PluralRule { match_suffix: "ife", drop: 2, append_suffix: "ves" },
        PluralRule { match_suffix: "lf", drop: 1, append_suffix: "ves" },
        PluralRule { match_suffix: "arf", drop: 1, append_suffix: "ves" },
        PluralRule { match_suffix: "ay", drop: 0, append_suffix: "s" },
        PluralRule { match_suffix: "ey", drop: 0, append_suffix: "s" },
        PluralRule { match_suffix: "iy", drop: 0, append_suffix: "s" },
        PluralRule { match_suffix: "oy", drop: 0, append_suffix: "s" },
        PluralRule { match_suffix: "uy", drop: 0, append_suffix: "s" },
        PluralRule { match_suffix: "y", drop: 1, append_suffix: "ies" },
        PluralRule { match_suffix: "ao", drop: 0, append_suffix: "s" },
        PluralRule { match_suffix: "eo", drop: 0, append_suffix: "s" },
        PluralRule { match_suffix: "io", drop: 0, append_suffix: "s" },
        PluralRule { match_suffix: "oo", drop: 0, append_suffix: "s" },
        PluralRule { match_suffix: "uo", drop: 0, append_suffix: "s" },
        // The o rule could be much larger... we'll add specific exceptions as
        // the come up.
        PluralRule { match_suffix: "o", drop: 0, append_suffix: "es" },
        // Lots of possible exceptions here.
        PluralRule { match_suffix: "ex", drop: 0, append_suffix: "es" },
    ));

    for rule in plural_rules {
        if input.ends_with(rule.match_suffix) {
            return input[0..(input.len() - rule.drop)].to_owned() + rule.append_suffix;
        }
    }
    input.to_owned() + "s"
}

#[cfg(test)]
mod test {
    #[test]
    fn pluralise_should_follow_english_rules() {
        for (word, plural) in vec!(
            ("cat", "cats"),
            ("wolf", "wolves"),
            ("scarf", "scarves"),
            ("volcano", "volcanoes"),
            ("canoe", "canoes"),
            ("pistachio", "pistachios"),
            ("match", "matches"),
            ("the fairest sex", "the fairest sexes"),
            ("loud hiss", "loud hisses"),
            ("evil axis", "evil axes"),
            ("death ray", "death rays"),
            ("killer blowfly", "killer blowflies"),
            ("house mouse", "house mice"),
            ("zombie sheep", "zombie sheep"),
        ) {
            assert_eq!(super::pluralise(word), plural);
        }
    }
}
