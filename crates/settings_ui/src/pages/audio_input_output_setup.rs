use gpui::{AnyElement, App, ElementId, ReadGlobal as _, Window};
use settings::{AudioInputDeviceName, AudioOutputDeviceName, SettingsStore};
use ui::{Button, ButtonStyle, Disableable, IntoElement, Tooltip, prelude::*};

use crate::{SettingField, SettingsFieldMetadata, SettingsUiFile};

fn render_disabled_audio_device_button(
    id: impl Into<ElementId>,
    current_value: Option<String>,
) -> AnyElement {
    let label = current_value.unwrap_or_else(|| "Temporarily unavailable".to_string());
    Button::new(id.into(), label)
        .style(ButtonStyle::Outlined)
        .disabled(true)
        .tooltip(Tooltip::text(
            "Audio device selection is temporarily disabled in this build.",
        ))
        .into_any_element()
}

// This renderer is intentionally stubbed so settings_ui does not pull the audio
// crate and its libwebrtc dependency graph into normal builds.
pub fn render_input_audio_device_dropdown(
    field: SettingField<AudioInputDeviceName>,
    file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    _window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let (_, current_value) =
        SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let current_value = current_value.and_then(|value| value.0.clone());
    render_disabled_audio_device_button("input-audio-device-dropdown", current_value)
}

pub fn render_output_audio_device_dropdown(
    field: SettingField<AudioOutputDeviceName>,
    file: SettingsUiFile,
    _metadata: Option<&SettingsFieldMetadata>,
    _window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let (_, current_value) =
        SettingsStore::global(cx).get_value_from_file(file.to_settings(), field.pick);
    let current_value = current_value.and_then(|value| value.0.clone());
    render_disabled_audio_device_button("output-audio-device-dropdown", current_value)
}
