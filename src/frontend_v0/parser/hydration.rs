use crate::collections::stable_ordered_set::StableOrderedSet;
use crate::frontend_v0::parser::lr1_automata_builder::LR1ParserItem;

pub fn dehydrate_state(
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

pub fn hydrate_state(
    state: &Vec<usize>,
    all_items: &StableOrderedSet<LR1ParserItem>,
) -> Vec<LR1ParserItem> {
    let state = state.clone();
    state
        .into_iter()
        .map(|x| all_items[x].clone())
        .collect::<Vec<_>>()
}
