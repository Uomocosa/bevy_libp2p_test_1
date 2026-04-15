use bevy::prelude::*;

use crate::clicker::component::ClickCounter;

pub fn update_counter(
    _query: Query<&ClickCounter>,
    _text_query: Query<&mut Text, With<ClickCounter>>,
) {
    // TODO: Update text when Bevy 0.18 Text API is clarified
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage() {
        let counter = ClickCounter(10);
        assert_eq!(counter.0, 10);
    }
}
