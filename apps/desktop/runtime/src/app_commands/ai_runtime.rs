use super::*;

pub fn llm_complete(
    service: &RuntimeService,
    request: LLMCompletionRequest,
) -> Result<LLMCompletionResponse, String> {
    service.complete_llm(request)
}
pub async fn llm_provider_test(
    provider: AIProviderSettings,
) -> Result<LLMProviderTestResult, String> {
    crate::llm::test_provider(provider).await
}
pub fn ai_runtime_snapshot(service: &RuntimeService) -> AIRuntimeBridgeSnapshot {
    service.ai_runtime_bridge_snapshot()
}
pub fn ai_runtime_probe(
    service: &RuntimeService,
    request: AIRuntimeProbeRequest,
) -> Option<AIRuntimeContextSnapshot> {
    service.ai_runtime_probe(request)
}
pub fn ai_runtime_state_snapshot(service: &RuntimeService) -> AIRuntimeStateSnapshot {
    service.ai_runtime_state_snapshot()
}
pub fn ai_runtime_dismiss_completion(service: &RuntimeService, project_id: String) -> bool {
    service.ai_runtime_dismiss_completion(&project_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn llm_commands_delegate_to_runtime_llm_layer_without_network() {
        let support_dir =
            std::env::temp_dir().join(format!("codux-app-command-llm-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&support_dir).expect("support dir");
        let service = RuntimeService::new(support_dir.clone());

        let completion_error = llm_complete(
            &service,
            LLMCompletionRequest {
                provider_id: Some("missing".to_string()),
                prompt: "Hello".to_string(),
                system_prompt: None,
                purpose: "chat".to_string(),
            },
        )
        .expect_err("missing provider");
        assert!(completion_error.contains("No available AI provider is configured"));

        let provider_error =
            crate::async_runtime::block_on(llm_provider_test(AIProviderSettings {
                id: "provider-a".to_string(),
                kind: "openAICompatible".to_string(),
                display_name: "Provider A".to_string(),
                is_enabled: true,
                model: "gpt-test".to_string(),
                base_url: "https://api.example.invalid/v1".to_string(),
                api_key: String::new(),
                use_for_memory_extraction: true,
                priority: 0,
            }))
            .expect_err("missing provider key");
        assert!(provider_error.contains("missing an API key"));

        let _ = std::fs::remove_dir_all(support_dir);
    }
}
