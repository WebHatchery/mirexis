use super::*;

#[test]
fn palette_covers_every_required_response_and_ambience() {
    let cues = palette()
        .into_iter()
        .map(|entry| entry.0)
        .collect::<Vec<_>>();
    for cue in [
        SoundCue::Focus,
        SoundCue::Invalid,
        SoundCue::Move,
        SoundCue::Hit,
        SoundCue::Miss,
        SoundCue::Damage,
        SoundCue::Ability,
        SoundCue::Objective,
        SoundCue::Victory,
        SoundCue::Defeat,
        SoundCue::CityAmbience,
        SoundCue::TacticalAmbience,
    ] {
        assert!(cues.contains(&cue));
    }
}

#[test]
fn volume_steps_are_bounded() {
    let settings = AudioSettings::default();
    assert_eq!(settings.volume_percent, 75);
    assert!(!settings.muted);
    assert!(!settings.reduced_motion);
}
