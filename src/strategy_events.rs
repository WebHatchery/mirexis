//! Projection and availability rules for persistent campaign events.

use crate::data::CharacterEventDef;
use crate::strategy::CharacterEventState;

pub(crate) fn from_definition(event: &CharacterEventDef) -> CharacterEventState {
    CharacterEventState {
        id: event.id.clone(),
        title: event.title.clone(),
        description: event.description.clone(),
        participants: event.participants.clone(),
        food_cost: event.food_cost,
        attention_change: event.attention_change,
        attention_faction: event.attention_faction.clone(),
        legacy_name: event.legacy_name.clone(),
        legacy_character_id: event.legacy_character_id.clone(),
        legacy_stat: event.legacy_stat.clone(),
        legacy_amount: event.legacy_amount,
        required_protocol: event.required_protocol.clone(),
        requires_contact_trace: event.requires_contact_trace,
        resolved: false,
    }
}

pub(crate) fn is_available(
    event: &CharacterEventState,
    protocol_id: &str,
    trace_complete: bool,
) -> bool {
    !event.resolved
        && (event.required_protocol.is_empty() || event.required_protocol == protocol_id)
        && (!event.requires_contact_trace || trace_complete)
}
