use superzet_model::{AgentPreset, AgentSession, WorkspaceEntry};
use task::{HideStrategy, RevealStrategy, RevealTarget, Shell, SpawnInTerminal, TaskId};

pub fn spawn_for_workspace(
    workspace: &WorkspaceEntry,
    session: &AgentSession,
    preset: &AgentPreset,
) -> SpawnInTerminal {
    let label = format!("{} · {}", workspace.name, preset.label);
    let command_label = if preset.args.is_empty() {
        preset.command.clone()
    } else {
        format!("{} {}", preset.command, preset.args.join(" "))
    };

    SpawnInTerminal {
        id: TaskId(format!("superzet:{}:{}", workspace.id, session.id)),
        full_label: label.clone(),
        label,
        command: Some(preset.command.clone()),
        args: preset.args.clone(),
        command_label,
        cwd: Some(workspace.worktree_path.clone()),
        env: preset.env.clone().into_iter().collect(),
        use_new_terminal: true,
        allow_concurrent_runs: true,
        reveal: RevealStrategy::Always,
        reveal_target: RevealTarget::Center,
        hide: HideStrategy::Never,
        shell: Shell::System,
        show_summary: true,
        show_command: true,
        show_rerun: true,
    }
}
