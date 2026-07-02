use crate::parser::parser_raw_grammar::ParserRawGrammar;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

pub struct LR0ParserItem {
    source: String,
    marker_pos: usize,
    production: Vec<String>,
    rule_id: usize,
}

pub struct LR1ParserItem {
    source: String,
    marker_pos: usize,
    production: Vec<String>,
    lookahead: HashSet<String>,
    rule_id: usize,
}

impl LR0ParserItem {
    fn to_lr1(&self, lookaheads: HashSet<String>) -> LR1ParserItem {
        LR1ParserItem {
            source: self.source.clone(),
            marker_pos: self.marker_pos,
            production: self.production.clone(),
            rule_id: self.rule_id.clone(),
            lookahead: lookaheads.clone(),
        }
    }
    fn get_next_token(&self) -> Option<String> {
        if self.marker_pos < self.production.len() {
            return Some(self.production[self.marker_pos].clone());
        }
        None
    }
}

impl Clone for LR0ParserItem {
    fn clone(&self) -> Self {
        LR0ParserItem {
            source: self.source.clone(),
            marker_pos: self.marker_pos,
            production: self.production.clone(),
            rule_id: self.rule_id.clone(),
        }
    }
}

impl PartialEq<Self> for LR0ParserItem {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
            && self.marker_pos == other.marker_pos
            && self.production == other.production
    }
}

impl Hash for LR0ParserItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.source.hash(state);
        self.marker_pos.hash(state);
        self.production.hash(state);
    }
}

impl LR1ParserItem {
    fn to_lr0(&self) -> LR0ParserItem {
        LR0ParserItem {
            source: self.source.clone(),
            marker_pos: self.marker_pos,
            production: self.production.clone(),
            rule_id: self.rule_id.clone(),
        }
    }

    fn get_next_token(&self) -> Option<String> {
        if self.marker_pos < self.production.len() {
            return Some(self.production[self.marker_pos].clone());
        }
        None
    }

    fn consume_token(&self, token: String) -> Option<LR1ParserItem> {
        let next_token = self.get_next_token();
        if (next_token? != token) {
            return None;
        }

        Some(LR1ParserItem {
            source: self.source.clone(),
            production: self.production.clone(),
            rule_id: self.rule_id.clone(),
            lookahead: self.lookahead.clone(),
            marker_pos: self.marker_pos + 1,
        })
    }
}

impl PartialEq for LR1ParserItem {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
            && self.marker_pos == other.marker_pos
            && self.production == other.production
            && self.lookahead == other.lookahead
    }
}

const MARKER_CHARACTER: char = '•';

struct LR0GotoTable {
    maps: HashMap<usize, HashMap<String, LR1ParserAction>>,
}

impl LR0GotoTable {
    fn new() -> LR0GotoTable {
        LR0GotoTable {
            maps: HashMap::new(),
        }
    }
    fn add_entry(&mut self, start: usize, token: String, target: usize) {
        if (!self.maps.contains_key(&start)) {
            self.maps.insert(start, HashMap::new());
        }
        let available_connections = self.maps.get_mut(&start).unwrap();

        let result = available_connections.insert(token.clone(), LR1ParserAction::Shift(target));
        if result.is_some() {
            println!(
                "Got some shift overriding in {0} -({1})> {2}",
                start, token, target
            );
        }
    }
    fn add_reduces(&mut self, start: usize, lookaheads: &HashSet<String>, rule: usize) {
        if (!self.maps.contains_key(&start)) {
            self.maps.insert(start, HashMap::new());
        }
        let available_connections = self.maps.get_mut(&start).unwrap();
        for token in lookaheads {
            let result = available_connections.insert(token.clone(), LR1ParserAction::Reduce(rule));
            if result.is_some() {
                println!(
                    "Got some reduce overriding in {0} -({1})> {2}",
                    start, token, rule
                );
            }
        }
    }
}

pub struct LR1Automata {
    grammar: ParserRawGrammar,
    goto_table: Vec<HashMap<String, LR1ParserAction>>,
}

pub enum LR1ParserAction {
    // Shift(next state id)
    Shift(usize),
    // Reduce(reduction rule id)
    Reduce(usize),
}

pub fn build_automata(
    token_alphabet: &HashSet<String>,
    grammar: ParserRawGrammar,
    start_non_terminal: String,
) -> LR1Automata {
    let (all_states, mut goto_table) =
        build_automata_states(token_alphabet, &grammar, start_non_terminal);

    for (state_id, state) in all_states.iter().enumerate() {
        let rules_for_reducing = state.iter().enumerate().filter(|(i, x)| x.get_next_token().is_none()).collect::<Vec<_>>();
        for (reduction_rule_id, rule) in rules_for_reducing {
            goto_table.add_reduces(state_id, &rule.lookahead, reduction_rule_id);
        }
    }
    let mut vector_goto_table = Vec::with_capacity(goto_table.maps.len());
    for (state_id, available_connections) in goto_table.maps {
        vector_goto_table[state_id] = available_connections;
    }


    LR1Automata {
        goto_table: vector_goto_table,
        grammar,
    }
}

fn build_automata_states(
    token_alphabet: &HashSet<String>,
    grammar: &ParserRawGrammar,
    start_non_terminal: String,
) -> (Vec<Vec<LR1ParserItem>>, LR0GotoTable) {
    let starting_state = create_starting_state(token_alphabet, grammar, start_non_terminal);
    let mut all_states = Vec::new();
    all_states.push(starting_state);

    let mut goto_table = LR0GotoTable::new();
    let mut queue = VecDeque::from([0]);

    while let Some(state_id) = queue.pop_front() {
        let consumable_tokens = get_consumable_tokens(&all_states[state_id]);
        for token in consumable_tokens {
            let known_items = all_states[state_id]
                .iter()
                .filter_map(|x| x.consume_token(token.clone()))
                .collect::<Vec<LR1ParserItem>>();
            let new_state = build_state_for_items(token_alphabet, grammar, known_items);

            let new_state_id = all_states
                .iter()
                .position(|x| x == &new_state)
                .or(Some(all_states.len()))
                .unwrap();

            goto_table.add_entry(state_id, token.clone(), new_state_id);

            // If it is indeed a new state
            if new_state_id == all_states.len() {
                all_states.push(new_state);
                queue.push_back(new_state_id);
            }
        }
    }
    (all_states, goto_table)
}

fn create_starting_state(
    token_alphabet: &HashSet<String>,
    grammar: &ParserRawGrammar,
    start_non_terminal: String,
) -> Vec<LR1ParserItem> {
    // Starting non-terminal items
    let known_items = grammar
        .rules
        .iter()
        .enumerate()
        .filter(|(i, x)| x.name == start_non_terminal)
        .map(|(i, x)| LR1ParserItem {
            source: x.name.clone(),
            production: x.production.clone(),
            marker_pos: 0,
            lookahead: HashSet::from(["eof".to_string()]),
            rule_id: i,
        })
        .collect::<Vec<LR1ParserItem>>();

    build_state_for_items(token_alphabet, grammar, known_items)
}

fn get_consumable_tokens(state: &Vec<LR1ParserItem>) -> HashSet<String> {
    state
        .iter()
        .filter_map(|item| item.get_next_token())
        .collect::<HashSet<String>>()
}

fn build_state_for_items(
    token_alphabet: &HashSet<String>,
    grammar: &ParserRawGrammar,
    known_items: Vec<LR1ParserItem>,
) -> Vec<LR1ParserItem> {
    let mut known_follow = HashMap::<String, HashSet<String>>::new();

    for item in &known_items {
        if !known_follow.contains_key(&item.source) {
            known_follow.insert(item.source.clone(), HashSet::new());
        }
        for lookahead in &item.lookahead {
            known_follow
                .get_mut(&item.source)
                .unwrap()
                .insert(lookahead.clone());
        }
    }

    let known_lr0_state = known_items
        .iter()
        .map(|x| x.to_lr0())
        .collect::<Vec<LR0ParserItem>>();
    let lr0_state = construct_lr0_state(token_alphabet, grammar, known_lr0_state);
    let follow = construct_follow_for_state(&token_alphabet, known_follow, &lr0_state);
    let lr1_state = lr0_state
        .iter()
        .map(|x| x.to_lr1(follow.get(&x.source).unwrap().clone()))
        .collect();

    lr1_state
}

fn construct_lr0_state(
    token_alphabet: &HashSet<String>,
    grammar: &ParserRawGrammar,
    known_items: Vec<LR0ParserItem>,
) -> Vec<LR0ParserItem> {
    let mut state = known_items;
    let mut prev_size = 0;
    let mut seen_tokens = HashSet::<String>::new();

    while prev_size != state.len() {
        prev_size = state.len();
        let mut queue = HashSet::<String>::new();

        for item in &state {
            let next_token = item.get_next_token();
            if next_token.is_none() {
                continue;
            }
            let current_token = next_token.unwrap();
            if !token_alphabet.contains(&current_token) && !seen_tokens.contains(&current_token) {
                queue.insert(current_token.clone());
                seen_tokens.insert(current_token.clone());
            }
        }

        for item in queue {
            let non_terminal_rules = grammar
                .rules
                .iter()
                .enumerate()
                .filter(|(_, x)| x.name == item)
                .map(|(i, x)| LR0ParserItem {
                    source: x.name.clone(),
                    production: x.production.clone(),
                    marker_pos: 0,
                    rule_id: i,
                })
                .collect::<Vec<LR0ParserItem>>();

            for item in non_terminal_rules {
                state.push(item);
            }
        }
    }

    state
}

fn construct_follow_for_state(
    token_alphabet: &HashSet<String>,
    known_follow: HashMap<String, HashSet<String>>,
    items: &Vec<LR0ParserItem>,
) -> HashMap<String, HashSet<String>> {
    let first = construct_first_for_state(token_alphabet, items);
    let mut follow = known_follow;

    for item in items {
        let next_token = item.get_next_token();
        if (next_token.is_none()) {
            continue;
        }
        let marked_token = next_token.unwrap();
        // Marker is on the last token of production
        // Then follow(B) U= follow(A)
        if (item.marker_pos == item.production.len() - 1) {
            let source_follow = follow.get(&item.source).unwrap().clone();
            let current_follow = follow.get_mut(&marked_token).unwrap();
            for follow_item in source_follow {
                current_follow.insert(follow_item.to_string());
            }
        } else {
            // A -> aBb, b is not empty
            // follow(B) U= first(b)
            let token_after_marked_one = item.production[item.marker_pos + 1].clone();
            let next_token_first = first.get(&token_after_marked_one).unwrap().clone();
            let current_follow = follow.get_mut(&marked_token).unwrap();
            for follow_item in next_token_first {
                current_follow.insert(follow_item.to_string());
            }
        }
    }

    follow
}

fn construct_first_for_state(
    token_alphabet: &HashSet<String>,
    items: &Vec<LR0ParserItem>,
) -> HashMap<String, HashSet<String>> {
    let mut first = HashMap::<String, HashSet<String>>::new();

    // Later non-terminals B may refer to earlier ones A making first(B) dependent on first(A) requiring at least one more iteration
    let mut first_changed = true;
    while first_changed {
        first_changed = false;

        for item in items.iter().rev() {
            if !first.contains_key(&item.source) {
                first.insert(item.source.clone(), HashSet::new());
            }

            let first_letter = item.production[0].clone();
            if token_alphabet.contains(&first_letter) {
                let current_first = first.get_mut(&item.source).unwrap();
                if !current_first.contains(&first_letter) {
                    current_first.insert(first_letter.clone());
                    first_changed = true;
                }
            } else {
                let mut intermediate_first = HashSet::from([first_letter.clone()]);
                let mut non_terminals = (&intermediate_first)
                    .iter()
                    .filter(|x| !token_alphabet.contains(&first_letter))
                    .map(|x| x.clone())
                    .collect::<Vec<String>>();

                while non_terminals.len() > 0 {
                    let recursive_first = non_terminals
                        .iter()
                        .flat_map(|x| first.get(x).unwrap())
                        .map(|x| x.clone())
                        .collect::<HashSet<String>>();
                    intermediate_first = intermediate_first
                        .union(&recursive_first)
                        .map(|x| x.clone())
                        .collect::<HashSet<_>>();
                    non_terminals = (&intermediate_first)
                        .iter()
                        .filter(|x| !token_alphabet.contains(&first_letter))
                        .map(|x| x.clone())
                        .collect::<Vec<String>>();
                }
                let current_first = first.get_mut(&item.source).unwrap();

                for x in intermediate_first {
                    if !current_first.contains(&x) {
                        current_first.insert(x);
                        first_changed = true;
                    }
                }
            }
        }
    }
    first
}
