use crate::collections::stable_ordered_set::StableOrderedSet;
use crate::frontend_v0::parser::lr1_parser::LR1Parser;
use crate::frontend_v0::parser::parser_grammar::{ParserGrammar, read_parser_grammar};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::io::Error;

pub struct LR0ParserItem {
    source: String,
    marker_pos: usize,
    production: Vec<String>,
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

#[derive(Clone)]
pub struct LR1ParserItem {
    source: String,
    marker_pos: usize,
    production: Vec<String>,
    lookahead: HashSet<String>,
    rule_id: usize,
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
        if next_token? != token {
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
    }
}

impl Eq for LR1ParserItem {}

impl Hash for LR1ParserItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.source.hash(state);
        self.marker_pos.hash(state);
        self.production.hash(state);
    }
}

impl Debug for LR1ParserItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut production = self.production.clone();
        production.insert(self.marker_pos, MARKER_CHARACTER.to_string());
        write!(
            f,
            "{0} -> {1}, {2}",
            self.source,
            production.join::<_>(&String::from(", ")),
            self.lookahead
                .iter()
                .map(|x| x.clone())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl Display for LR1ParserItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut production = self.production.clone();
        production.insert(self.marker_pos, MARKER_CHARACTER.to_string());
        write!(
            f,
            "{0} -> {1}, {2}",
            self.source,
            production.join::<_>(&String::from(" ")),
            self.lookahead
                .iter()
                .map(|x| x.clone())
                .collect::<Vec<_>>()
                .join(",")
        )
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
    fn add_entry(
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
            let hydrated_source_state =
                Self::hydrate_state(start, all_states, all_items);
            println!(
                "{}",
                hydrated_source_state
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
            println!("Target state ({}):", target);
            let hydrated_target_state =
                Self::hydrate_state(target, all_states, all_items);
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
    fn add_reduces(
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
                        let hydrated_source_state =
                            Self::hydrate_state(start, all_states, all_items);
                        println!(
                            "{}",
                            hydrated_source_state
                                .iter()
                                .map(|x| format!("{}", x))
                                .collect::<Vec<String>>()
                                .join("\n")
                        );
                        println!("Primary target state ({}):", start);
                        let hydrated_target_state =
                            Self::hydrate_state(target_state, all_states, all_items);
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
                                Self::hydrate_state(start, all_states, all_items);
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

    fn hydrate_state(
        state_id: usize,
        all_states: &StableOrderedSet<Vec<usize>>,
        all_items: &StableOrderedSet<LR1ParserItem>,
    ) -> Vec<LR1ParserItem> {
        let state = all_states[state_id].clone();
        state
            .into_iter()
            .map(|x| all_items[x].clone())
            .collect::<Vec<_>>()
    }
}

pub struct LR1Automata {
    pub grammar: ParserGrammar,
    pub goto_table: Vec<HashMap<String, LR1ParserAction>>,
}

pub enum LR1ParserAction {
    // Shift(next state id)
    Shift(usize),
    // Reduce(reduction rule id)
    Reduce(usize),
}

pub fn build_lr1_parser(
    token_alphabet: &HashSet<String>,
    parser_grammar_filename: String,
) -> LR1Parser {
    let automata = build_automata_from_grammar(token_alphabet, parser_grammar_filename).unwrap();
    LR1Parser::new(automata)
}
pub fn build_automata_from_grammar(
    token_alphabet: &HashSet<String>,
    filename: String,
) -> Result<LR1Automata, Error> {
    let grammar = read_parser_grammar(&filename)?;
    let starting_rule = grammar.rules[0].name.clone();

    Ok(build_automata(token_alphabet, grammar, starting_rule))
}

pub fn build_automata(
    token_alphabet: &HashSet<String>,
    grammar: ParserGrammar,
    start_non_terminal: String,
) -> LR1Automata {
    let (all_states, all_items, mut goto_table) =
        build_automata_states(token_alphabet, &grammar, start_non_terminal.clone());

    for (state_id, state) in all_states.iter().enumerate() {
        let rules_for_reducing = state
            .iter()
            .filter(|x| {
                let item = all_items.get_index((*x).clone());
                if item.is_none() {
                    return false;
                }
                item.unwrap().get_next_token().is_none()
            })
            .collect::<Vec<_>>();
        for rule_index in rules_for_reducing {
            let rule_option = all_items.get_index(rule_index.clone());
            if rule_option.is_none() {
                continue;
            }
            let rule = rule_option.unwrap();
            goto_table.add_reduces(
                state_id,
                &rule.lookahead,
                rule.rule_id,
                &grammar,
                &all_states,
                &all_items,
            );
        }
    }
    let mut vector_goto_table = Vec::with_capacity(goto_table.maps.len());
    for _i in 0..goto_table.maps.len() {
        vector_goto_table.push(HashMap::new());
    }
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
    grammar: &ParserGrammar,
    start_non_terminal: String,
) -> (
    StableOrderedSet<Vec<usize>>,
    StableOrderedSet<LR1ParserItem>,
    LR0GotoTable,
) {
    let starting_state = create_starting_state(token_alphabet, grammar, start_non_terminal);
    let mut all_items = StableOrderedSet::<LR1ParserItem>::new();

    let mut all_states = StableOrderedSet::<Vec<usize>>::new(); //Vec::new();
    let remapped_state = remap_state(starting_state, &mut all_items);
    all_states.insert(remapped_state);

    let mut goto_table = LR0GotoTable::new();
    let mut queue = VecDeque::from([0]);

    while let Some(state_id) = queue.pop_front() {
        println!("Got state {}", state_id);
        let consumable_tokens = get_consumable_tokens(&all_states[state_id], &all_items);
        for token in consumable_tokens {
            let known_items = all_states[state_id]
                .iter()
                .filter_map(|x| {
                    all_items
                        .get_index(x.clone())
                        .and_then(|x| x.consume_token(token.clone()))
                })
                .collect::<Vec<LR1ParserItem>>();
            let new_state = remap_state(
                build_state_for_items(token_alphabet, grammar, known_items),
                &mut all_items,
            );

            let new_state_id = all_states
                .get_index_of(&new_state)
                .or(Some(all_states.len()))
                .unwrap();

            goto_table.add_entry(
                state_id,
                token.clone(),
                new_state_id,
                &grammar,
                &all_states,
                &all_items,
            );

            // If it is indeed a new state
            if new_state_id == all_states.len() {
                all_states.insert(new_state);
                queue.push_back(new_state_id);
            }
        }
    }
    (all_states, all_items, goto_table)
}

fn remap_state(
    state: Vec<LR1ParserItem>,
    all_items: &mut StableOrderedSet<LR1ParserItem>,
) -> Vec<usize> {
    let mut remapped_state = Vec::new();

    for item in state {
        if !all_items.contains(&item) {
            remapped_state.push(all_items.len());
            all_items.insert(item);
        } else {
            let item_index = all_items.get_index_of(&item).unwrap();
            all_items
                .get_index_mut(item_index)
                .unwrap()
                .lookahead
                .extend(item.lookahead.iter().cloned());
            remapped_state.push(item_index);
        }
    }

    remapped_state.sort();
    remapped_state
}

fn create_starting_state(
    token_alphabet: &HashSet<String>,
    grammar: &ParserGrammar,
    start_non_terminal: String,
) -> Vec<LR1ParserItem> {
    // Starting non-terminal items
    let known_items = grammar
        .rules
        .iter()
        .enumerate()
        .filter(|(_i, x)| x.name == start_non_terminal)
        .map(|(i, x)| LR1ParserItem {
            source: x.name.clone(),
            production: x.production.clone(),
            marker_pos: 0,
            lookahead: HashSet::new(),
            rule_id: i,
        })
        .collect::<Vec<LR1ParserItem>>();

    build_state_for_items(token_alphabet, grammar, known_items)
}

fn get_consumable_tokens(
    state: &Vec<usize>,
    all_items: &StableOrderedSet<LR1ParserItem>,
) -> StableOrderedSet<String> {
    state
        .iter()
        .filter_map(|item| {
            all_items
                .get_index(item.clone())
                .and_then(|x| x.get_next_token())
        })
        .collect::<StableOrderedSet<String>>()
}

fn build_state_for_items(
    token_alphabet: &HashSet<String>,
    grammar: &ParserGrammar,
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
    grammar: &ParserGrammar,
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
    let mut follow_changed = true;

    while follow_changed {
        follow_changed = false;

        for item in items {
            let next_token = item.get_next_token();
            if next_token.is_none() {
                continue;
            }
            let marked_token = next_token.unwrap();
            // Marker is on the last token of production
            // Then follow(B) U= follow(A)
            if item.marker_pos == item.production.len() - 1 {
                let source_follow =
                    get_or_default(&mut follow, item.source.clone(), HashSet::new()).clone();
                let current_follow =
                    get_mut_or_default(&mut follow, marked_token.clone(), HashSet::new());
                for follow_item in source_follow {
                    if !current_follow.contains(&follow_item) {
                        current_follow.insert(follow_item.to_string());
                        follow_changed = true;
                    }
                }
            } else {
                // A -> aBb, b is not empty
                // follow(B) U= first(b)
                let token_after_marked_one = item.production[item.marker_pos + 1].clone();
                let next_token_first = first
                    .get(&token_after_marked_one)
                    .or(Some(&HashSet::new()))
                    .unwrap()
                    .clone();
                let current_follow =
                    get_mut_or_default(&mut follow, marked_token.clone(), HashSet::new());
                for follow_item in next_token_first {
                    if !current_follow.contains(&follow_item) {
                        current_follow.insert(follow_item.to_string());
                        follow_changed = true;
                    }
                }
            }
        }
    }

    follow
}

fn get_or_default<'a, T>(hashmap: &'a mut HashMap<String, T>, key: String, default: T) -> &'a T {
    if hashmap.contains_key(&key) {
        return hashmap.get(&key).unwrap();
    }
    hashmap.insert(key.clone(), default);
    hashmap.get(&key).unwrap()
}
fn get_mut_or_default<T, K>(hashmap: &mut HashMap<K, T>, key: K, default: T) -> &mut T
where
    K: Eq + Hash + Clone,
{
    if hashmap.contains_key(&key) {
        return hashmap.get_mut(&key).unwrap();
    }
    hashmap.insert(key.clone(), default);
    hashmap.get_mut(&key).unwrap()
}

fn construct_first_for_state(
    token_alphabet: &HashSet<String>,
    items: &Vec<LR0ParserItem>,
) -> HashMap<String, HashSet<String>> {
    let mut first = HashMap::<String, HashSet<String>>::new();

    for terminal in token_alphabet {
        let mut terminal_first = HashSet::new();
        terminal_first.insert(terminal.clone());
        first.insert(terminal.clone(), terminal_first);
    }

    // Later non-terminals B may refer to earlier ones A making first(B) dependent on first(A) requiring at least one more iteration
    let mut first_changed = true;
    let empty_hashset = HashSet::new();
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
                let mut non_terminals = intermediate_first
                    .iter()
                    .filter(|x| !token_alphabet.contains(*x))
                    .map(|x| x.clone())
                    .collect::<HashSet<String>>();

                while non_terminals.len() > 0 {
                    let recursive_first = non_terminals
                        .iter()
                        .flat_map(|x| first.get(x).or(Some(&empty_hashset)).unwrap())
                        .map(|x| x.clone())
                        .collect::<HashSet<String>>();
                    intermediate_first = intermediate_first
                        .difference(&non_terminals)
                        .cloned()
                        .collect::<HashSet<String>>()
                        .union(&recursive_first)
                        .map(|x| x.clone())
                        .collect::<HashSet<_>>();
                    non_terminals = intermediate_first
                        .iter()
                        .filter(|x| !token_alphabet.contains(*x))
                        .map(|x| x.clone())
                        .collect::<HashSet<String>>();
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
