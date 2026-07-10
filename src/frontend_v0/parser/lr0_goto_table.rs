use crate::collections::stable_ordered_set::StableOrderedSet;
use crate::frontend_v0::parser::hydration::hydrate_state;
use crate::frontend_v0::parser::lr1_automata_builder::{LR1ParserAction, LR1ParserItem};
use crate::frontend_v0::parser::parser_grammar::ParserGrammar;
use std::collections::{HashMap, HashSet};

pub struct LR0GotoTable {
    pub(crate) maps: HashMap<usize, HashMap<String, LR1ParserAction>>,
}

impl LR0GotoTable {
    pub(crate) fn new() -> LR0GotoTable {
        LR0GotoTable {
            maps: HashMap::new(),
        }
    }
    pub(crate) fn add_entry(
        &mut self,
        start: usize,
        token: String,
        target: usize,
        grammar: &ParserGrammar,
        all_states: &StableOrderedSet<Vec<usize>>,
        all_items: &StableOrderedSet<LR1ParserItem>,
    ) {
        if !self.maps.contains_key(&start) {
            self.maps.insert(start, HashMap::new());
        }
        let available_connections = self.maps.get_mut(&start).unwrap();

        let result = available_connections.insert(token.clone(), LR1ParserAction::Shift(target));
        if result.is_some() {
            println!(
                "Got some shift overriding in {0} -({1})> {2}",
                start, token, target
            );
            println!("Source state ({}):", start);
            let hydrated_source_state = hydrate_state(&all_states[start], all_items);
            println!(
                "{}",
                hydrated_source_state
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
            println!("Target state ({}):", target);
            let hydrated_target_state = hydrate_state(&all_states[target], all_items);
            println!(
                "{}",
                hydrated_target_state
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
        }
    }
    pub fn add_reduces(
        &mut self,
        start: usize,
        lookaheads: &HashSet<String>,
        rule: usize,
        grammar: &ParserGrammar,
        all_states: &StableOrderedSet<Vec<usize>>,
        all_items: &StableOrderedSet<LR1ParserItem>,
    ) {
        if !self.maps.contains_key(&start) {
            self.maps.insert(start, HashMap::new());
        }
        let available_connections = self.maps.get_mut(&start).unwrap();
        for token in lookaheads {
            let result = available_connections.insert(token.clone(), LR1ParserAction::Reduce(rule));
            if result.is_some() {
                let some_result = result.unwrap();
                match some_result {
                    LR1ParserAction::Shift(target_state) => {
                        println!(
                            "Got some shift->reduce overriding in {0} -({1})> {2}",
                            start, token, rule
                        );

                        println!("Source state ({}):", start);
                        let hydrated_source_state = hydrate_state(&all_states[start], all_items)
                            .iter()
                            .map(|x| format!("{}", x))
                            .collect::<Vec<String>>()
                            .join("\n");
                        println!("{}", hydrated_source_state);
                        println!("Primary target state ({}):", start);
                        let hydrated_target_state =
                            hydrate_state(&all_states[target_state], all_items);
                        println!(
                            "{}",
                            hydrated_target_state
                                .iter()
                                .map(|x| format!("{}", x))
                                .collect::<Vec<String>>()
                                .join("\n")
                        );
                    }
                    LR1ParserAction::Reduce(rule_number) => {
                        if rule_number != rule {
                            println!(
                                "Got some reduce({0})->reduce({1}) overriding in {2} -({3})> {4}",
                                rule_number, rule, start, token, rule
                            );
                            println!("Source state ({}):", start);
                            let hydrated_source_state =
                                hydrate_state(&all_states[start], all_items);
                            println!(
                                "{}",
                                hydrated_source_state
                                    .iter()
                                    .map(|x| format!(" - {}", x))
                                    .collect::<Vec<String>>()
                                    .join("\n")
                            );
                            println!(
                                "Primary reduction rule ({}): {}",
                                rule_number, grammar.rules[rule_number]
                            );
                            println!("Final reduction rule ({}): {}", rule, grammar.rules[rule]);
                            println!();
                        }
                    }
                }
            }
        }
    }
}
