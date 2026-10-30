use super::{Game, native_hooks::*};
use std::{cell::RefCell, sync::Arc};
const ID: &str = "rust_jei";
#[derive(Default)]
struct State {
    tables: Option<Arc<NativeRecipes>>,
    query: String,
    shown: Vec<usize>,
    recipes: Vec<usize>,
    page: usize,
}
thread_local! {static STATE:RefCell<State>=RefCell::new(State::default());}
impl Game<'_> {
    pub(super) fn rust_jei_list(&mut self, mods: &mut Vec<NativeMod>) {
        mods.push(NativeMod {
            id: ID.into(),
            title: "JEI Rust 0.2.0".into(),
            description: "Busca, receitas (R), usos (U) e preenchimento de receitas conhecidas."
                .into(),
            keys: vec![
                ["recipes".into(), "Receitas".into(), "key.keyboard.r".into()],
                ["uses".into(), "Usos".into(), "key.keyboard.u".into()],
            ],
        });
    }
    pub(super) fn rust_jei_frame(&mut self) {
        if !self.native_window() {
            return;
        }
        if !self.native_on(ID) || self.player.is_none() {
            STATE.with(|s| *s.borrow_mut() = State::default());
            self.native_panel_items(ID, vec![]);
            self.native_panel_recipes(ID, "", vec![], 0, vec![]);
            return;
        }
        let Some(tables) = self.native_recipes() else {
            return;
        };
        STATE.with(|state| {
            let mut s = state.borrow_mut();
            if s.tables.as_ref().is_some_and(|t| Arc::ptr_eq(t, &tables)) {
                return;
            }
            *s = State {
                tables: Some(tables),
                ..State::default()
            };
            self.rust_jei_items(&mut s);
        });
    }
    fn rust_jei_items(&mut self, s: &mut State) {
        let Some(t) = &s.tables else {
            return;
        };
        s.shown = t
            .items
            .iter()
            .enumerate()
            .filter(|(_, i)| super::rust_jei_core::matches(&s.query, &i.id, &i.name, &i.tags))
            .map(|(n, _)| n)
            .take(8192)
            .collect();
        let mut items: Vec<NativeStack> = s
            .shown
            .iter()
            .map(|&n| NativeStack {
                item: t.items[n].id.clone(),
                count: 1,
                components: String::new(),
            })
            .collect();
        // Keep the search box usable when nothing matches; the empty stack has no icon.
        if items.is_empty() {
            items.push(NativeStack::default());
        }
        self.native_panel_recipes(ID, "", vec![], 0, vec![]);
        self.native_panel_items(ID, items);
        s.recipes.clear();
        s.page = 0;
    }
    fn rust_jei_recipes(&mut self, s: &mut State, item: &str, uses: bool) {
        let Some(t) = &s.tables else {
            return;
        };
        let Some(index) = t.items.iter().position(|i| i.id == item) else {
            return;
        };
        s.recipes = t
            .recipes
            .iter()
            .enumerate()
            .filter(|(_, r)| {
                if uses {
                    r.slots.iter().flatten().any(|v| v.item as usize == index)
                } else {
                    r.results.iter().any(|v| v.item as usize == index)
                }
            })
            .map(|(n, _)| n)
            .take(256)
            .collect();
        s.page = 0;
        if s.recipes.is_empty() {
            self.native_chat("Nenhuma receita encontrada.");
            return;
        }
        let known = self.native_known_recipes();
        let stack = |v: &NativeRecipeStack| NativeStack {
            item: t
                .items
                .get(v.item as usize)
                .map_or_else(String::new, |i| i.id.clone()),
            count: v.count,
            components: v.components.clone(),
        };
        let views = s
            .recipes
            .iter()
            .map(|&n| {
                let r = &t.recipes[n];
                let layout = match r.kind {
                    NativeRecipeKind::CraftingShaped | NativeRecipeKind::CraftingShapeless => {
                        NativeRecipeLayout::Grid
                    }
                    NativeRecipeKind::Stonecutting => NativeRecipeLayout::Single,
                    NativeRecipeKind::Smithing => NativeRecipeLayout::Smithing,
                    NativeRecipeKind::Brewing => NativeRecipeLayout::Brewing,
                    _ => NativeRecipeLayout::Cooking,
                };
                let shaped = r.kind == NativeRecipeKind::CraftingShaped;
                let grid = layout == NativeRecipeLayout::Grid;
                let action = grid.then_some(r.book_id >= 0 && known.contains(&r.book_id));
                NativeRecipeView {
                    layout,
                    width: if shaped { r.width } else { 3 },
                    height: if shaped {
                        r.height
                    } else {
                        (r.slots.len() as u32).div_ceil(3).max(1)
                    },
                    slots: r
                        .slots
                        .iter()
                        .map(|v| v.iter().map(stack).collect())
                        .collect(),
                    results: r.results.iter().map(stack).collect(),
                    note: if r.cooking_time > 0 {
                        format!("{:.1}s | {} XP", r.cooking_time as f32 / 20.0, r.experience)
                    } else {
                        r.id.clone()
                    },
                    action,
                }
            })
            .collect();
        let prefix = if uses { "Usos" } else { "Receitas" };
        self.native_panel_recipes(
            ID,
            &format!("{prefix}: {}", t.items[index].name),
            vec![],
            0,
            views,
        );
    }
    pub(super) fn rust_jei_event(&mut self, mod_id: &str, event: &NativePanelEvent) {
        if mod_id != ID || !self.native_on(ID) {
            return;
        }
        STATE.with(|state| {
            let mut s = state.borrow_mut();
            match event {
                NativePanelEvent::Search(query) => {
                    s.query = query.clone();
                    self.rust_jei_items(&mut s);
                }
                NativePanelEvent::Back => self.rust_jei_items(&mut s),
                NativePanelEvent::Clicked { index, right } => {
                    let item = s
                        .shown
                        .get(*index as usize)
                        .and_then(|&n| s.tables.as_ref()?.items.get(n))
                        .map(|i| i.id.clone());
                    if let Some(item) = item {
                        self.rust_jei_recipes(&mut s, &item, *right);
                    }
                }
                NativePanelEvent::SlotClicked { stack, right, .. } => {
                    self.rust_jei_recipes(&mut s, &stack.item, *right)
                }
                NativePanelEvent::Key { key, stack } => {
                    if *key == self.native_mod_key(ID, "recipes", "key.keyboard.r") {
                        self.rust_jei_recipes(&mut s, &stack.item, false);
                    } else if *key == self.native_mod_key(ID, "uses", "key.keyboard.u") {
                        self.rust_jei_recipes(&mut s, &stack.item, true);
                    }
                }
                NativePanelEvent::Page(first) => s.page = *first as usize,
                NativePanelEvent::Action(index) => {
                    if let Some(r) = s
                        .recipes
                        .get(*index as usize)
                        .and_then(|&n| s.tables.as_ref()?.recipes.get(n))
                    {
                        if let Err(e) = self.native_place_recipe(r.book_id, self.has_shift_down()) {
                            self.native_chat(&e);
                        }
                    }
                }
                _ => {}
            }
        });
    }
}
