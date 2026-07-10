use crate::frontend_v0::parser::lr1_automata_builder::LR1ParserItem;
use crate::frontend_v0::parser::parser_grammar::ParserGrammarRule;
use std::fmt::{Debug, Formatter};

pub enum GotoTableError {
    ShiftShiftConflict {
        source_state_id: usize,
        token: String,
        primary_target_state_id: usize,
        final_target_state_id: usize,
        source_state: Vec<LR1ParserItem>,
        primary_target_state: Vec<LR1ParserItem>,
        final_target_state: Vec<LR1ParserItem>,
    },
    ShiftReduceConflict {
        source_state_id: usize,
        token: String,
        shift_target_state_id: usize,
        reduction_rule_id: usize,
        source_state: Vec<LR1ParserItem>,
        shift_target_state: Vec<LR1ParserItem>,
        reduction_rule: ParserGrammarRule,
    },
    ReduceReduceConflict {
        source_state_id: usize,
        token: String,
        primary_reduction_rule_id: usize,
        final_reduction_rule_id: usize,
        source_state: Vec<LR1ParserItem>,
        primary_reduction_rule: ParserGrammarRule,
        final_reduction_rule: ParserGrammarRule,
    },
}

impl GotoTableError {
    fn get_base_message(&self) -> String {
        match self {
            GotoTableError::ShiftShiftConflict {
                source_state_id,
                token,
                primary_target_state_id,
                final_target_state_id,
                ..
            } => {
                format!(
                    "Got shift-shift conflict {source_state_id} -({token})> shift({primary_target_state_id}) \
                    to {source_state_id} -({token})> shift({final_target_state_id})"
                )
            }
            GotoTableError::ShiftReduceConflict {
                source_state_id,
                token,
                shift_target_state_id,
                reduction_rule_id,
                ..
            } => {
                format!(
                    "Got shift-reduce conflict {source_state_id} -({token})> shift({shift_target_state_id}) \
                    to {source_state_id} -({token})> reduce({reduction_rule_id})"
                )
            }
            GotoTableError::ReduceReduceConflict {
                source_state_id,
                token,
                primary_reduction_rule_id,
                final_reduction_rule_id,
                ..
            } => {
                format!(
                    "Got reduce-reduce conflict {source_state_id} -({token})> reduce({primary_reduction_rule_id}) \
                    to {source_state_id} -({token})> reduce({final_reduction_rule_id})"
                )
            }
        }
    }

    fn format_state(state: Vec<LR1ParserItem>) -> String {
        state
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn get_additional_info(&self) -> String {
        match self {
            GotoTableError::ShiftShiftConflict {
                source_state_id,
                source_state,
                primary_target_state_id,
                primary_target_state,
                final_target_state_id,
                final_target_state,
                ..
            } => {
                let formatted_source_state = Self::format_state(source_state.clone());
                let formatted_primary_target_state =
                    Self::format_state(primary_target_state.clone());
                let formatted_final_target_state = Self::format_state(final_target_state.clone());

                format!(
                    "Source state ({source_state_id}):\n{formatted_source_state}\n\
                Primary target state({primary_target_state_id}:\n{formatted_primary_target_state})\n\
                Final target state({final_target_state_id}:\n{formatted_final_target_state})\n",
                )
            }
            GotoTableError::ShiftReduceConflict {
                source_state_id,
                source_state,
                shift_target_state_id,
                shift_target_state,
                reduction_rule_id,
                reduction_rule,
                ..
            } => {
                let formatted_source_state = Self::format_state(source_state.clone());
                let formatted_shift_target_state = Self::format_state(shift_target_state.clone());

                format!(
                    "Source state ({source_state_id}):\n{formatted_source_state}\n\
                Shift target state({shift_target_state_id}:\n{formatted_shift_target_state})\n\
                Reduction rule({reduction_rule_id}:\n{reduction_rule})\n",
                )
            }
            GotoTableError::ReduceReduceConflict {
                source_state_id,
                source_state,
                primary_reduction_rule_id,
                primary_reduction_rule,
                final_reduction_rule_id,
                final_reduction_rule,
                ..
            } => {
                let formatted_source_state = Self::format_state(source_state.clone());

                format!(
                    "Source state ({source_state_id}):\n{formatted_source_state}\n\
                    Primary reduction state({primary_reduction_rule_id}:\n{primary_reduction_rule})\n\
                    Final reduction rule({final_reduction_rule_id}:\n{final_reduction_rule})\n",
                )
            }
        }
    }
}

impl Debug for GotoTableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}",
            self.get_base_message(),
            self.get_additional_info()
        )
    }
}
