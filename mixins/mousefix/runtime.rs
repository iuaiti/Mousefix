// Mouse Fix: keeps the attack and use buttons from sticking or flipping.
//
// * A mod cancels a button's release (`native_mouse_button_pre`): the release
//   never reaches `play_mouse`, so mining, hitting or using goes on.
// * Sleeping: `play_mouse` ignores the buttons and `startSleeping` clears
//   left/right but not the toggles, so a swallowed release leaves a toggled
//   attack or use down and the next click flips it off.
// * Alt-Tab with no pause screen: `set_down(false)` is a no-op on a toggled
//   mapping, so the toggle stays down while left/right are false.
//
// This file is both the mod's Cargo library (its tests) and the module the
// bundle adds to the game, so it holds the decisions and the helpers hold the
// field writes.

/// The mod's id (its settings folder), name and version.
pub const ID: &str = "mousefix";
pub const NAME: &str = "Mouse Fix";
pub const VERSION: &str = "0.1.0";
pub const DESCRIPTION: &str = "Stops the attack and use buttons from sticking or flipping after a mod cancels a release, after sleeping in a bed or after Alt-Tab.";

/// `ToggleKeys` slots of the two mouse mappings.
pub const ATTACK: usize = 2;
pub const USE: usize = 3;

/// The slot a GLFW button (0 left, 1 right) drives.
pub fn slot_of_glfw_button(button: i32) -> Option<usize> {
    match button {
        0 => Some(ATTACK),
        1 => Some(USE),
        _ => None,
    }
}

/// A button went up and a mod cancelled it: the game never saw the release.
pub fn must_force_release(pressed: bool, canceled: bool) -> bool {
    !pressed && canceled
}

/// Whether the attack and use state should be dropped now.
///
/// * `control`: a `--control` script holds buttons without the window's focus.
/// * `sleeping`: the game ignores the mouse in bed.
/// * `playing`: with a screen open the game's own `releaseAll` (and the restore
///   when it closes) owns the toggles, so they are left to it.
pub fn should_drop_buttons(focused: bool, control: bool, sleeping: bool, playing: bool) -> bool {
    if control {
        return false;
    }
    sleeping || (!focused && playing)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buttons_map_to_the_toggle_slots() {
        assert_eq!(slot_of_glfw_button(0), Some(ATTACK));
        assert_eq!(slot_of_glfw_button(1), Some(USE));
        assert_eq!(slot_of_glfw_button(2), None);
        assert_eq!(slot_of_glfw_button(7), None);
    }

    #[test]
    fn only_a_cancelled_release_is_forced() {
        assert!(must_force_release(false, true));
        assert!(!must_force_release(false, false));
        assert!(!must_force_release(true, true));
        assert!(!must_force_release(true, false));
    }

    #[test]
    fn sleeping_drops_the_buttons() {
        assert!(should_drop_buttons(true, false, true, true));
        assert!(should_drop_buttons(true, false, true, false));
    }

    #[test]
    fn alt_tab_drops_the_buttons_only_with_no_screen() {
        assert!(should_drop_buttons(false, false, false, true));
        assert!(!should_drop_buttons(false, false, false, false));
        assert!(!should_drop_buttons(true, false, false, true));
    }

    #[test]
    fn a_control_script_keeps_its_buttons() {
        assert!(!should_drop_buttons(false, true, true, true));
    }
}
