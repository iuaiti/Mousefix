# Mousefix

Stops the attack and use mouse buttons from sticking or flipping.

| Case | What went wrong | What the mod does |
|---|---|---|
| A mod cancels a button's **release** (`native_mouse_button_pre`) | the release never reaches `play_mouse`: mining/hitting/using goes on, or a toggle is out of step | RETURN hook on `native_mouse_button_pre` (sees every mod's final `canceled`): puts the logical button up, through `ToggleKeys::button` like `play_mouse` does |
| **Sleeping** | `play_mouse` ignores the mouse and `startSleeping` clears `left/right` only; a toggled attack/use stays down and the next click flips it **off** | each tick and frame while sleeping: `reset()` the attack and use toggles, `left = right = false` |
| **Alt-Tab** with no screen open | `Focused(false)` calls `set_down(false)`, a no-op on a toggle, then clears `left/right`: toggle down, button up | while unfocused, playing, and not under `--control`: same reset |

Deliberately **not** changed: the recapture click that only grabs the pointer, `miss_time = 10000` while a screen is open, and the remote `mem::take` of queued clicks while not ready. They match vanilla's `MouseHandler`/`Minecraft.tick` (a screen's `releaseAll` drops pending clicks too), and `InteractionInput`'s fields are private to `horizon-client-input`, so a mod cannot restore them. Holding use while an item is in use already retries in the local game (`use_delay` path).

## Build

```sh
python3 native/tools/native.py check mousefix                 # on the pinned commit
python3 native/tools/native.py check --horizon latest mousefix # on the newest Nightly
python3 native/tools/native.py package mousefix                # native/target/mousefix-0.1.0.hnmod
cargo test                                                     # the logic's unit tests
```

Or in the launcher: `horizon-launcher native-build native/mousefix/mixins/mousefix --trust-native-code`.

The mod reads private `Game` fields (`toggles`, `left`, `right`, `focused`, `sleeping`, `control`, `settings`); if a Nightly renames one, `check` fails with the Rust error and the helper needs the new name.

## Where it lives

This folder is standalone. To use `native.py`, copy `Cargo.toml`, `Cargo.lock`, `compatibility.json` and `mixins/` into a clone of [horizon-mods](https://github.com/horizon-rust/horizon-mods) as `native/mousefix/` and run the commands above from that clone's root. `mousefix-0.1.0.hnmod` here is the packaged bundle (sha256 `556f2b42b391f3d55dc07b9c7fbe2a5e763b141d42515a6dc0463623842f22f8`).
