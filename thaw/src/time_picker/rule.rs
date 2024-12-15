use crate::{FieldValidationState, Rule};
use chrono::NaiveTime;
use leptos::prelude::*;
use std::ops::Deref;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum TimePickerRuleTrigger {
    #[default]
    Blur,
}

pub struct TimePickerRule(Rule<Option<NaiveTime>, TimePickerRuleTrigger>);

impl TimePickerRule {
    pub fn required(required: Signal<bool>) -> Self {
        Self::validator(move |value, name| {
            if required.get_untracked() && value.is_none() {
                let message = name.get_untracked().map_or_else(
                    || String::from("Please select!"),
                    |name| format!("Please select {name}!"),
                );
                Err(FieldValidationState::Error(message))
            } else {
                Ok(())
            }
        })
    }

    pub fn required_with_message(required: Signal<bool>, message: Signal<String>) -> Self {
        Self::validator(move |value, _| {
            if required.get_untracked() && value.is_none() {
                Err(FieldValidationState::Error(message.get_untracked()))
            } else {
                Ok(())
            }
        })
    }

    pub fn validator(
        f: impl Fn(&Option<NaiveTime>, Signal<Option<String>>) -> Result<(), FieldValidationState>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Rule::validator(f))
    }

    pub fn with_trigger(self, trigger: TimePickerRuleTrigger) -> Self {
        Self(Rule::with_trigger(self.0, trigger))
    }
}

impl Deref for TimePickerRule {
    type Target = Rule<Option<NaiveTime>, TimePickerRuleTrigger>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
