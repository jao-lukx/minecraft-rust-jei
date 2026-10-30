fn hook(game: &mut crate::game::Game<'_>, mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    game.rust_jei_list(mods);
}
