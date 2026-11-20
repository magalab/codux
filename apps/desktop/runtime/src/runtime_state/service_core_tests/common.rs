fn wait_for_active_workspace_watch_path(service: &RuntimeService, expected: &str) {
    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    loop {
        let (file_path, git_path) = service
            .active_project_watches
            .lock()
            .map(|watches| (watches.file_path.clone(), watches.git_path.clone()))
            .expect("active workspace watches");
        if file_path
            .as_deref()
            .is_some_and(|path| {
                crate::git::repository_path_key(path) == crate::git::repository_path_key(expected)
            })
            && git_path
                .as_deref()
                .is_some_and(|path| {
                    crate::git::repository_path_key(path)
                        == crate::git::repository_path_key(expected)
                })
        {
            return;
        }
        if std::time::Instant::now() >= deadline {
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    let watches = service
        .active_project_watches
        .lock()
        .expect("active workspace watches");
    assert!(
        watches
            .file_path
            .as_deref()
            .is_some_and(|path| {
                crate::git::repository_path_key(path) == crate::git::repository_path_key(expected)
            })
    );
    assert!(
        watches
            .git_path
            .as_deref()
            .is_some_and(|path| {
                crate::git::repository_path_key(path) == crate::git::repository_path_key(expected)
            })
    );
}

fn wait_for_ai_history_loading_event(
    service: &RuntimeService,
    project_id: &str,
    project_path: &str,
) {
    // Generous cap: the history indexer competes with the whole suite for
    // scheduling; passing runs return on the first matching drain.
    for _ in 0..800 {
        let result = service.drain_ai_history_events();
        if result.events.iter().any(|event| {
            matches!(
                event,
                AIHistoryEvent::ProjectState { state }
                    if state.project_id == project_id
                        && state.project_path == project_path
                        && state.is_loading
            )
        }) {
            return;
        }
        thread::sleep(Duration::from_millis(10));
    }
    let result = service.drain_ai_history_events();
    assert!(
        result.events.iter().any(|event| {
            matches!(
                event,
                AIHistoryEvent::ProjectState { state }
                    if state.project_id == project_id
                        && state.project_path == project_path
                        && state.is_loading
            )
        }),
        "expected AI history loading event for {project_id} at {project_path}, got {:?}",
        result.events
    );
}
fn assert_tracked_project_has_git_refresh(service: &RuntimeService, project_id: &str) {
    let activity = service.project_activity_snapshot();
    let tracked = activity
        .tracked_projects
        .iter()
        .find(|project| project.id == project_id)
        .unwrap_or_else(|| panic!("missing tracked project {project_id}: {activity:?}"));
    assert!(
        tracked.has_git_refresh,
        "expected git refresh marker for {project_id}: {activity:?}"
    );
    assert!(
        activity.activated_git_count > 0,
        "expected activated git count after project activation: {activity:?}"
    );
}

#[cfg(unix)]
fn recv_until_contains(rx: &flume::Receiver<Vec<u8>>, needle: &str, timeout: Duration) -> String {
    let deadline = Instant::now() + timeout;
    let mut output = String::new();
    while Instant::now() < deadline {
        let remaining = deadline.saturating_duration_since(Instant::now());
        match rx.recv_timeout(remaining.min(Duration::from_millis(50))) {
            Ok(bytes) => {
                output.push_str(&String::from_utf8_lossy(&bytes));
                if output.contains(needle) {
                    return output;
                }
            }
            Err(flume::RecvTimeoutError::Timeout) => {}
            Err(flume::RecvTimeoutError::Disconnected) => break,
        }
    }
    output
}
