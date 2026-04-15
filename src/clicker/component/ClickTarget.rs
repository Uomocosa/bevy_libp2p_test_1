use bevy::prelude::*;

#[derive(Component)]
pub struct ClickTarget;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage() {
        let target = ClickTarget;
        let _ = target;
    }
}
