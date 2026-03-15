use std::rc::Rc;

use acp_thread::{AgentConnection, AgentSessionInfo};
use agent::ThreadStore;
use agent_client_protocol as acp;
use agent_servers::{AgentServer, AgentServerDelegate};
use anyhow::Result;
use db::kvp::KEY_VALUE_STORE;
use fs::Fs;
use gpui::{
    App, Context, Entity, EventEmitter, FocusHandle, Focusable, Render, SharedString, Subscription,
    Task, WeakEntity, Window,
};
use project::{Project, agent_server_store::ExternalAgentServerName};
use serde::{Deserialize, Serialize};
use ui::{Color, Icon, IconName, Label, prelude::*};
use util::ResultExt as _;
use workspace::{Item, Toast, Workspace, notifications::NotificationId};

use crate::{
    AgentInitialContent, ConnectionView, ExternalAgent,
    thread_history::{ThreadHistory, ThreadHistoryEvent},
};

const LAST_USED_EXTERNAL_AGENT_KEY: &str = "agent_panel__last_used_external_agent";

#[derive(Serialize, Deserialize)]
struct LastUsedExternalAgent {
    agent: ExternalAgent,
}

pub fn open_external_acp_tab(
    workspace: &mut Workspace,
    agent_name: Option<String>,
    prompt: Option<String>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    if workspace.project().read(cx).is_via_collab() {
        show_toast(
            workspace,
            "External ACP tabs are not available in shared projects.",
            cx,
        );
        return;
    }

    match agent_name {
        Some(agent_name) => {
            open_external_acp_tab_for_agent(workspace, agent_name.into(), None, prompt, window, cx);
        }
        None => {
            let workspace_handle = workspace.weak_handle();
            cx.spawn_in(window, async move |_workspace, cx| {
                let agent_name = resolve_default_agent_name(workspace_handle.clone(), cx).await?;
                let Some(agent_name) = agent_name else {
                    workspace_handle
                        .update_in(cx, |workspace, _, cx| {
                            show_toast(workspace, "Add an ACP agent to open a tab.", cx);
                        })
                        .ok();
                    return anyhow::Ok(());
                };

                workspace_handle.update_in(cx, |workspace, window, cx| {
                    open_external_acp_tab_for_agent(
                        workspace,
                        agent_name.clone(),
                        None,
                        prompt.clone(),
                        window,
                        cx,
                    );
                })?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        }
    }
}

pub fn open_external_acp_history(
    workspace: &mut Workspace,
    agent_name: Option<String>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    if workspace.project().read(cx).is_via_collab() {
        show_toast(
            workspace,
            "External ACP history is not available in shared projects.",
            cx,
        );
        return;
    }

    match agent_name {
        Some(agent_name) => {
            open_external_acp_history_for_agent(workspace, agent_name.into(), window, cx);
        }
        None => {
            let workspace_handle = workspace.weak_handle();
            cx.spawn_in(window, async move |_workspace, cx| {
                let agent_name = resolve_default_agent_name(workspace_handle.clone(), cx).await?;
                let Some(agent_name) = agent_name else {
                    workspace_handle
                        .update_in(cx, |workspace, _, cx| {
                            show_toast(workspace, "Add an ACP agent to open history.", cx);
                        })
                        .ok();
                    return anyhow::Ok(());
                };

                workspace_handle.update_in(cx, |workspace, window, cx| {
                    open_external_acp_history_for_agent(workspace, agent_name.clone(), window, cx);
                })?;
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        }
    }
}

pub fn focus_external_acp_tab(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> bool {
    if let Some(active_tab) = workspace
        .active_pane()
        .read(cx)
        .active_item_as::<ExternalAcpTabItem>()
    {
        workspace.activate_item(&active_tab, true, true, window, cx);
        return true;
    }

    let existing_tab = { workspace.items_of_type::<ExternalAcpTabItem>(cx).next() };
    if let Some(existing_tab) = existing_tab {
        workspace.activate_item(&existing_tab, true, true, window, cx);
        return true;
    }

    false
}

fn open_external_acp_tab_for_agent(
    workspace: &mut Workspace,
    agent_name: SharedString,
    resume_thread: Option<AgentSessionInfo>,
    prompt: Option<String>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let existing_tab = resume_thread
        .as_ref()
        .map(|thread| thread.session_id.clone())
        .and_then(|session_id| {
            workspace
                .items_of_type::<ExternalAcpTabItem>(cx)
                .find(|item| {
                    item.read(cx)
                        .session_id(cx)
                        .is_some_and(|existing_session_id| existing_session_id == session_id)
                })
        });
    if let Some(existing_tab) = existing_tab {
        workspace.activate_item(&existing_tab, true, true, window, cx);
        remember_last_used_external_agent(agent_name, cx);
        return;
    }

    let project = workspace.project().clone();
    let display_name = agent_display_name(&project, &agent_name, cx);
    let icon_path = agent_icon_path(&project, &agent_name, cx);
    let server = ExternalAgent::Custom {
        name: agent_name.clone(),
    }
    .server(<dyn Fs>::global(cx), ThreadStore::global(cx));
    let thread_store = server
        .clone()
        .downcast::<agent::NativeAgentServer>()
        .is_some()
        .then(|| ThreadStore::global(cx));
    let tab = cx.new(|cx| {
        ExternalAcpTabItem::new(
            agent_name.clone(),
            display_name,
            icon_path,
            server,
            workspace.weak_handle(),
            project,
            resume_thread,
            prompt,
            thread_store,
            window,
            cx,
        )
    });
    workspace.add_item_to_center(Box::new(tab), window, cx);
    remember_last_used_external_agent(agent_name, cx);
}

fn open_external_acp_history_for_agent(
    workspace: &mut Workspace,
    agent_name: SharedString,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let existing_history = {
        workspace
            .items_of_type::<ExternalAcpHistoryItem>(cx)
            .find(|item| item.read(cx).agent_name == agent_name)
    };
    if let Some(existing_history) = existing_history {
        workspace.activate_item(&existing_history, true, true, window, cx);
        remember_last_used_external_agent(agent_name, cx);
        return;
    }

    let project = workspace.project().clone();
    let history = cx.new(|cx| {
        ExternalAcpHistoryItem::new(
            agent_name.clone(),
            workspace.weak_handle(),
            project,
            window,
            cx,
        )
    });
    workspace.add_item_to_center(Box::new(history), window, cx);
    remember_last_used_external_agent(agent_name, cx);
}

async fn resolve_default_agent_name(
    workspace: WeakEntity<Workspace>,
    cx: &mut gpui::AsyncWindowContext,
) -> Result<Option<SharedString>> {
    let last_used_agent = cx
        .background_spawn(async move { read_last_used_external_agent() })
        .await;
    let configured_agents = workspace.read_with(cx, |workspace, cx| {
        configured_external_agent_names(&workspace.project(), cx)
    })?;

    Ok(select_default_agent_name(
        configured_agents,
        last_used_agent,
    ))
}

fn select_default_agent_name(
    configured_agents: Vec<SharedString>,
    last_used_agent: Option<SharedString>,
) -> Option<SharedString> {
    if let Some(last_used_agent) = last_used_agent
        && configured_agents
            .iter()
            .any(|agent| agent == &last_used_agent)
    {
        return Some(last_used_agent);
    }

    configured_agents.into_iter().next()
}

fn configured_external_agent_names(project: &Entity<Project>, cx: &App) -> Vec<SharedString> {
    let agent_server_store = project.read(cx).agent_server_store().clone();
    let agent_server_store = agent_server_store.read(cx);
    let mut agents = agent_server_store
        .external_agents()
        .map(|agent_name| {
            (
                agent_name.0.clone(),
                agent_server_store
                    .agent_display_name(agent_name)
                    .unwrap_or_else(|| agent_name.0.clone()),
            )
        })
        .collect::<Vec<_>>();
    agents.sort_by(|left, right| left.1.to_lowercase().cmp(&right.1.to_lowercase()));
    agents
        .into_iter()
        .map(|(agent_name, _)| agent_name)
        .collect()
}

fn read_last_used_external_agent() -> Option<SharedString> {
    KEY_VALUE_STORE
        .read_kvp(LAST_USED_EXTERNAL_AGENT_KEY)
        .log_err()
        .flatten()
        .and_then(|value| serde_json::from_str::<LastUsedExternalAgent>(&value).log_err())
        .and_then(|entry| match entry.agent {
            ExternalAgent::Custom { name } => Some(name),
            ExternalAgent::NativeAgent => None,
        })
}

fn remember_last_used_external_agent(agent_name: SharedString, cx: &mut App) {
    cx.background_spawn(async move {
        let Some(serialized) = serde_json::to_string(&LastUsedExternalAgent {
            agent: ExternalAgent::Custom { name: agent_name },
        })
        .log_err() else {
            return;
        };

        KEY_VALUE_STORE
            .write_kvp(LAST_USED_EXTERNAL_AGENT_KEY.to_string(), serialized)
            .await
            .log_err();
    })
    .detach();
}

fn agent_display_name(
    project: &Entity<Project>,
    agent_name: &SharedString,
    cx: &App,
) -> SharedString {
    let agent_server_store = project.read(cx).agent_server_store().clone();
    agent_server_store
        .read(cx)
        .agent_display_name(&ExternalAgentServerName(agent_name.clone()))
        .unwrap_or_else(|| agent_name.clone())
}

fn agent_icon_path(
    project: &Entity<Project>,
    agent_name: &SharedString,
    cx: &App,
) -> Option<SharedString> {
    let agent_server_store = project.read(cx).agent_server_store().clone();
    agent_server_store
        .read(cx)
        .agent_icon(&ExternalAgentServerName(agent_name.clone()))
        .or_else(|| {
            project::AgentRegistryStore::try_global(cx).and_then(|registry_store| {
                registry_store
                    .read(cx)
                    .agent(agent_name.as_ref())
                    .and_then(|agent| agent.icon_path().cloned())
            })
        })
}

fn show_toast(
    workspace: &mut Workspace,
    message: impl Into<SharedString>,
    cx: &mut Context<Workspace>,
) {
    struct ExternalAcpToast;

    workspace.show_toast(
        Toast::new(
            NotificationId::unique::<ExternalAcpToast>(),
            message.into().to_string(),
        )
        .autohide(),
        cx,
    );
}

struct ExternalAcpTabItem {
    icon_path: Option<SharedString>,
    connection_view: Entity<ConnectionView>,
    _history: Entity<ThreadHistory>,
    _subscriptions: Vec<Subscription>,
}

impl ExternalAcpTabItem {
    #[allow(clippy::too_many_arguments)]
    fn new(
        _agent_name: SharedString,
        _display_name: SharedString,
        icon_path: Option<SharedString>,
        server: Rc<dyn AgentServer>,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        resume_thread: Option<AgentSessionInfo>,
        prompt: Option<String>,
        thread_store: Option<Entity<ThreadStore>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let history = cx.new(|cx| ThreadHistory::new(None, window, cx));
        let initial_content = prompt.map(|text| AgentInitialContent::ContentBlock {
            blocks: vec![acp::ContentBlock::Text(acp::TextContent::new(text))],
            auto_submit: false,
        });
        let connection_view = cx.new(|cx| {
            ConnectionView::new(
                server,
                resume_thread,
                initial_content,
                workspace,
                project,
                thread_store,
                None,
                history.clone(),
                window,
                cx,
            )
        });

        let subscriptions = vec![
            cx.observe(&connection_view, |_, _, cx| {
                cx.notify();
            }),
            cx.observe(&history, |_, _, cx| {
                cx.notify();
            }),
        ];

        Self {
            icon_path,
            connection_view,
            _history: history,
            _subscriptions: subscriptions,
        }
    }

    fn session_id(&self, cx: &App) -> Option<acp::SessionId> {
        self.connection_view.read(cx).session_id(cx)
    }
}

impl Item for ExternalAcpTabItem {
    type Event = ();

    fn include_in_nav_history() -> bool {
        false
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        self.connection_view.read(cx).tab_title(cx)
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        Some(self.connection_view.read(cx).tab_title(cx))
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(match &self.icon_path {
            Some(icon_path) => Icon::from_external_svg(icon_path.clone()),
            None => Icon::new(IconName::Sparkle),
        })
    }
}

impl EventEmitter<()> for ExternalAcpTabItem {}

impl Focusable for ExternalAcpTabItem {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.connection_view.read(cx).focus_handle(cx)
    }
}

impl Render for ExternalAcpTabItem {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.connection_view.clone().into_any_element()
    }
}

enum HistoryState {
    Loading,
    Unsupported(SharedString),
    Error(SharedString),
    Ready {
        history: Entity<ThreadHistory>,
        _connection: Rc<dyn AgentConnection>,
    },
}

struct ExternalAcpHistoryItem {
    agent_name: SharedString,
    display_name: SharedString,
    icon_path: Option<SharedString>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    state: HistoryState,
    _load_task: Task<Result<()>>,
    _subscriptions: Vec<Subscription>,
}

impl ExternalAcpHistoryItem {
    fn new(
        agent_name: SharedString,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let display_name = agent_display_name(&project, &agent_name, cx);
        let icon_path = agent_icon_path(&project, &agent_name, cx);
        let focus_handle = cx.focus_handle();
        let delegate = AgentServerDelegate::new(
            project.read(cx).agent_server_store().clone(),
            project.clone(),
            None,
            None,
        );
        let server = ExternalAgent::Custom {
            name: agent_name.clone(),
        }
        .server(<dyn Fs>::global(cx), ThreadStore::global(cx));
        let connect_task = server.connect(delegate, cx);

        let mut this = Self {
            agent_name,
            display_name,
            icon_path,
            workspace,
            focus_handle,
            state: HistoryState::Loading,
            _load_task: Task::ready(Ok(())),
            _subscriptions: Vec::new(),
        };

        this._load_task = cx.spawn_in(window, async move |this, cx| {
            let result: Result<()> = async {
                let connection = connect_task.await?;
                let session_list = if connection.supports_session_history() {
                    cx.update(|_, cx| connection.session_list(cx))?
                } else {
                    None
                };

                this.update_in(cx, move |this, window, cx| {
                    let Some(session_list) = session_list else {
                        this.state = HistoryState::Unsupported(
                            format!("{} does not expose session history.", this.display_name)
                                .into(),
                        );
                        cx.notify();
                        return;
                    };

                    let history = cx.new(|cx| ThreadHistory::new(Some(session_list), window, cx));
                    this._subscriptions.push(cx.observe(&history, |_, _, cx| {
                        cx.notify();
                    }));
                    this._subscriptions.push(cx.subscribe_in(
                        &history,
                        window,
                        |this, _, event, window, cx| match event {
                            ThreadHistoryEvent::Open(thread) => {
                                if let Some(workspace) = this.workspace.upgrade() {
                                    let thread = thread.clone();
                                    let agent_name = this.agent_name.clone();
                                    workspace.update(cx, |workspace, cx| {
                                        open_external_acp_tab_for_agent(
                                            workspace,
                                            agent_name,
                                            Some(thread),
                                            None,
                                            window,
                                            cx,
                                        );
                                    });
                                }
                            }
                        },
                    ));
                    this.state = HistoryState::Ready {
                        history,
                        _connection: connection,
                    };
                    cx.notify();
                })?;

                Ok(())
            }
            .await;

            if let Err(error) = result {
                this.update_in(cx, |this, _, cx| {
                    this.state = HistoryState::Error(error.to_string().into());
                    cx.notify();
                })
                .ok();
            }

            Ok(())
        });

        this
    }
}

impl Item for ExternalAcpHistoryItem {
    type Event = ();

    fn include_in_nav_history() -> bool {
        false
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        format!("{} History", self.display_name).into()
    }

    fn tab_tooltip_text(&self, _cx: &App) -> Option<SharedString> {
        Some(format!("{} History", self.display_name).into())
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(match &self.icon_path {
            Some(icon_path) => Icon::from_external_svg(icon_path.clone()),
            None => Icon::new(IconName::Sparkle),
        })
    }
}

impl EventEmitter<()> for ExternalAcpHistoryItem {}

impl Focusable for ExternalAcpHistoryItem {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.state {
            HistoryState::Ready { history, .. } => history.read(cx).focus_handle(cx),
            HistoryState::Loading | HistoryState::Unsupported(_) | HistoryState::Error(_) => {
                self.focus_handle.clone()
            }
        }
    }
}

impl Render for ExternalAcpHistoryItem {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        match &self.state {
            HistoryState::Ready { history, .. } => history.clone().into_any_element(),
            HistoryState::Loading => {
                centered_message(format!("Loading {} history…", self.display_name))
            }
            HistoryState::Unsupported(message) | HistoryState::Error(message) => {
                centered_message(message.clone())
            }
        }
    }
}

fn centered_message(message: impl Into<SharedString>) -> AnyElement {
    v_flex()
        .size_full()
        .items_center()
        .justify_center()
        .gap_2()
        .child(
            Label::new(message.into())
                .color(Color::Muted)
                .size(LabelSize::Small),
        )
        .into_any_element()
}
