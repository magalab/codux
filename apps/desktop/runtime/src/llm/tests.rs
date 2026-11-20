use super::*;

#[test]
fn selects_memory_provider_by_priority() {
    let settings = AISettings {
        providers: vec![
            provider("a", 10, true),
            provider("b", 1, true),
            provider("c", 0, false),
        ],
        ..AISettings::default()
    };

    let selected = select_provider(&settings, None, "memory").unwrap();

    assert_eq!(selected.id, "b");
}

fn provider(id: &str, priority: i32, use_for_memory_extraction: bool) -> AIProviderSettings {
    AIProviderSettings {
        id: id.to_string(),
        kind: "openAICompatible".to_string(),
        display_name: id.to_string(),
        is_enabled: true,
        model: "gpt-4.1-mini".to_string(),
        base_url: "https://api.openai.com/v1".to_string(),
        api_key: "key".to_string(),
        use_for_memory_extraction,
        priority,
    }
}
