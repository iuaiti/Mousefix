// A cancelled release skipped `play_mouse`: put the logical button up. For a
// toggled mapping a release is a no-op in `KeyState::set_down`, so this is
// exact in both modes.
fn mousefix_cancelled_release(
    _result: (),
    game: &mut crate::game::Game<'_>,
    button: i32,
    pressed: bool,
    canceled: &bool,
) {
    use crate::game::mousefix as m;
    if !game.native_on(m::ID) || !m::must_force_release(pressed, *canceled) {
        return;
    }
    match m::slot_of_glfw_button(button) {
        Some(m::ATTACK) => {
            let option = game.settings.toggle_attack;
            game.toggles.button(m::ATTACK, false, false, option);
            game.left = game.toggles.is_down(m::ATTACK);
        }
        Some(m::USE) => {
            let option = game.settings.toggle_use;
            game.toggles.button(m::USE, false, false, option);
            game.right = game.toggles.is_down(m::USE);
        }
        _ => {}
    }
}
