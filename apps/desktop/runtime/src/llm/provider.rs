fn select_provider<'a>(
    settings: &'a AISettings,
    provider_id: Option<&str>,
    purpose: &str,
) -> Option<&'a AIProviderSettings> {
    let requested = provider_id
        .map(str::trim)
        .filter(|value| !value.is_empty() && *value != "automatic");
    if let Some(id) = requested
        && let Some(provider) = settings.providers.iter().find(|provider| {
            provider.id == id && provider.is_enabled && supports_completion(&provider.kind)
        })
    {
        return Some(provider);
    }
    let selector = match purpose {
        "memory" => settings.memory.default_extractor_provider_id.as_str(),
        "gitCommitMessage" => "automatic",
        _ => "automatic",
    };
    if selector != "automatic"
        && let Some(provider) = settings.providers.iter().find(|provider| {
            provider.id == selector && provider.is_enabled && supports_completion(&provider.kind)
        })
    {
        return Some(provider);
    }
    settings
        .providers
        .iter()
        .filter(|provider| {
            provider.is_enabled
                && supports_completion(&provider.kind)
                && (purpose != "memory" || provider.use_for_memory_extraction)
        })
        .min_by(|left, right| {
            left.priority
                .cmp(&right.priority)
                .then_with(|| left.display_name.cmp(&right.display_name))
        })
}

fn supports_completion(kind: &str) -> bool {
    codux_llm::supports_completion(kind)
}

fn sanitize_test_provider(mut provider: AIProviderSettings) -> Result<AIProviderSettings, String> {
    provider.id = provider.id.trim().to_string();
    if provider.id.is_empty() {
        provider.id = "provider-test".to_string();
    }
    provider.display_name = provider.display_name.trim().to_string();
    if provider.display_name.is_empty() {
        provider.display_name = match provider.kind.as_str() {
            "anthropic" => "Claude API".to_string(),
            "deepseek" => "DeepSeek API".to_string(),
            "gemini" => "Gemini API".to_string(),
            "groq" => "Groq API".to_string(),
            "openrouter" => "OpenRouter API".to_string(),
            "ollama" | "localLlama" => "Ollama".to_string(),
            _ => "OpenAI API".to_string(),
        };
    }
    if !supports_completion(&provider.kind) {
        return Err("Only HTTP API providers can be tested in this build.".to_string());
    }
    Ok(provider)
}

fn sanitize_response_line(text: &str) -> String {
    text.replace(['\r', '\n'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim_matches(|ch| matches!(ch, '"' | '\'' | '“' | '”' | '‘' | '’'))
        .chars()
        .take(500)
        .collect()
}
