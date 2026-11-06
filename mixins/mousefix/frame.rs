// Sleeping: `play_mouse` ignores the buttons, and `startSleeping` clears
// left/right but not the toggles, so a toggled attack or use would flip off on
// the next click. Alt-Tab with no screen: `Focused(false)` calls
// `set_down(false)`, a no-op on a toggle, and clears left/right only, which
// leaves the toggle down. Drop both mappings for good; only the window's game.
fn mousefix_frame(game: &mut crate::game::Game<'_>) {
    use crate::game::mousefix as m;
    if !game.native_on(m::ID) || !game.native_window() {
        return;
    }
    let drop = m::should_drop_buttons(
        game.focused,
        game.control.is_some(),
        game.sleeping.is_some(),
        game.native_playing(),
    );
    if drop {
        game.toggles.0[m::ATTACK].reset();
        game.toggles.0[m::USE].reset();
        game.left = false;
        game.right = false;
    }
}
