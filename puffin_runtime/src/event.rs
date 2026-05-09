use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MediaKeyCode, ModifierKeyCode, MouseEvent};

pub const EVENT_NAME_ONKEY: &'static str = "on_key";
pub const EVENT_NAME_ONBUTTON: &'static str = "on_button";
pub const EVENT_NAME_ONFOCUSGAINED: &'static str = "on_focus";
pub const EVENT_NAME_ONFOCUSLOST: &'static str = "on_unfocus";
pub const EVENT_NAME_ONPASTE: &'static str = "on_paste";
pub const EVENT_NAME_ONRESIZE: &'static str = "on_resize";

pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    FocusGained,
    FocusLost,
    Paste(String),
    Resize(u16, u16),
}

pub fn key_code_name(key: &KeyCode) -> String {
    match key {
        KeyCode::Backspace => "backspace".into(),
        KeyCode::Enter => "enter".into(),
        KeyCode::Left => "left".into(),
        KeyCode::Right => "right".into(),
        KeyCode::Up => "up".into(),
        KeyCode::Down => "down".into(),
        KeyCode::Home => "home".into(),
        KeyCode::End => "end".into(),
        KeyCode::PageUp => "pageup".into(),
        KeyCode::PageDown => "pagedown".into(),
        KeyCode::Tab => "tab".into(),
        KeyCode::BackTab => "backtab".into(),
        KeyCode::Delete => "delete".into(),
        KeyCode::Insert => "insert".into(),
        KeyCode::F(n) => format!("f{n}"),
        KeyCode::Char(c) => format!("{c}"),
        KeyCode::Null => "null".into(),
        KeyCode::Esc => "escape".into(),
        KeyCode::CapsLock => "capslock".into(),
        KeyCode::ScrollLock => "scrolllock".into(),
        KeyCode::NumLock => "numlock".into(),
        KeyCode::PrintScreen => "printscreen".into(),
        KeyCode::Pause => "pause".into(),
        KeyCode::Menu => "menu".into(),
        KeyCode::KeypadBegin => "keypadbegin".into(),
        KeyCode::Media(media) => match media {
            MediaKeyCode::Play => "play",
            MediaKeyCode::Pause => "pause",
            MediaKeyCode::PlayPause => "playpause",
            MediaKeyCode::Reverse => "reverse",
            MediaKeyCode::Stop => "stop",
            MediaKeyCode::FastForward => "fastforward",
            MediaKeyCode::Rewind => "rewind",
            MediaKeyCode::TrackNext => "tracknext",
            MediaKeyCode::TrackPrevious => "trackprevious",
            MediaKeyCode::Record => "record",
            MediaKeyCode::LowerVolume => "lowervolume",
            MediaKeyCode::RaiseVolume => "raisevolume",
            MediaKeyCode::MuteVolume => "mutevolume",
        }.into(),
        KeyCode::Modifier(modifier) => match modifier {
            ModifierKeyCode::LeftShift => "leftshift",
            ModifierKeyCode::LeftControl => "leftctrl",
            ModifierKeyCode::LeftAlt => "leftalt",
            ModifierKeyCode::LeftSuper => "leftsuper",
            ModifierKeyCode::LeftHyper => "lefthyper",
            ModifierKeyCode::LeftMeta => "leftmeta",
            ModifierKeyCode::RightShift => "rightshift",
            ModifierKeyCode::RightControl => "rightctrl",
            ModifierKeyCode::RightAlt => "rightalt",
            ModifierKeyCode::RightSuper => "rightsuper",
            ModifierKeyCode::RightHyper => "righthyper",
            ModifierKeyCode::RightMeta => "righthmeta",
            ModifierKeyCode::IsoLevel3Shift => "ilv3s",
            ModifierKeyCode::IsoLevel5Shift => "ilv5s",
        }.into()
    }
}

pub fn to_modifier_names(modifiers: &KeyModifiers) -> Vec<String> {
    let mut mods = vec![];

    if modifiers.contains(KeyModifiers::SHIFT) {
        mods.push("shift".into());
    }
    if modifiers.contains(KeyModifiers::CONTROL) {
        mods.push("control".into());
    }
    if modifiers.contains(KeyModifiers::ALT) {
        mods.push("alt".into());
    }
    if modifiers.contains(KeyModifiers::SUPER) {
        mods.push("super".into());
    }
    if modifiers.contains(KeyModifiers::HYPER) {
        mods.push("hyper".into());
    }
    if modifiers.contains(KeyModifiers::META) {
        mods.push("meta".into());
    }

    mods
}
