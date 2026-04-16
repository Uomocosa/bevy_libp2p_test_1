use bevy::prelude::*;

use crate::clicker::component::ClickCounter;

pub fn update_counter(
    counter_query: Query<&ClickCounter>,
    mut text_query: Query<&mut Text, With<ClickCounter>>,
) {
    for counter in &counter_query {
        for mut text in &mut text_query {
            text.0 = format!("Clicks: {}", counter.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage() {
        let mut world = World::new();

        world.spawn((ClickCounter(5), Text::new("Clicks: 0")));

        let mut schedule = Schedule::default();
        schedule.add_systems(update_counter);
        schedule.run(&mut world);

        let mut query = world.query::<&Text>();
        let texts: Vec<_> = query.iter(&world).collect();
        assert!(!texts.is_empty());
        assert!(texts[0].0.contains("5"), "Text should contain click count");
    }
}
