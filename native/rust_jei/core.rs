pub fn matches(query: &str, id: &str, name: &str, tags: &[String]) -> bool {
    query.split_whitespace().all(|term| {
        let term = term.to_lowercase();
        if let Some(namespace) = term.strip_prefix('@') {
            id.split(':').next().unwrap_or("").contains(namespace)
        } else if let Some(tag) = term.strip_prefix('#') {
            tags.iter().any(|t| t.to_lowercase().contains(tag))
        } else {
            id.to_lowercase().contains(&term) || name.to_lowercase().contains(&term)
        }
    })
}
#[cfg(test)]
mod tests {
    #[test]
    fn search_combines_name_namespace_and_tags() {
        let tags = vec!["minecraft:planks".into()];
        assert!(super::matches(
            "oak @minecraft #planks",
            "minecraft:oak_planks",
            "Tábuas de carvalho",
            &tags
        ));
        assert!(!super::matches(
            "@other",
            "minecraft:oak_planks",
            "Tábuas",
            &tags
        ));
        assert!(super::matches(
            "CARVALHO",
            "minecraft:oak_planks",
            "Tábuas de carvalho",
            &tags
        ));
    }
}
