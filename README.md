# dispatcher
Daemon that listens for configured global keypresses that, when detected, invoke corresponding shell commands.

## Platforms
- ### ✔️ Linux
- ### ✔️ Windows
- ### ❌ MacOS

Binary expects paths `{exe}/resources/dispatch.toml` and `{exe}/daemon/` exist, will be automatically generated when built with cargo (via build.rs).

## Example `resources/dispatch.toml`
##### !!! Alt+Shift+Control+E will quit the program !!!
```
# firefox
[[commands]]
alt = true
shift = true
control = true
hotkey = "KeyS"
script = "firefox example.com"

# test box
[[commands]]
alt = true
shift = true
control = true
hotkey = "KeyT"
script = "PowerShell -Command \"Add-Type -AssemblyName PresentationFramework;[System.Windows.MessageBox]::Show('Daemon Running')\""
```

## Modifier options
```
alt = true # option
meta = true # cmd/super/win
shift = true
control = true
```

## [[commands.hotkey]] `hotkey` options
> Stringified versions of livesplit_hotkey::Hotkey @ https://docs.rs/livesplit-hotkey/latest/livesplit_hotkey/enum.KeyCode.html
```
pub enum KeyCode {
    Backquote,
    Backslash,
    BracketLeft,
    BracketRight,
    Comma,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Equal,
    IntlBackslash,
    IntlRo,
    IntlYen,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Minus,
    Period,
    Quote,
    Semicolon,
    Slash,
    AltLeft,
    AltRight,
    Backspace,
    CapsLock,
    ContextMenu,
    ControlLeft,
    ControlRight,
    Enter,
    MetaLeft,
    MetaRight,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    Convert,
    KanaMode,
    Lang1,
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    NonConvert,
    Delete,
    End,
    Help,
    Home,
    Insert,
    PageDown,
    PageUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    NumLock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadBackspace,
    NumpadClear,
    NumpadClearEntry,
    NumpadComma,
    NumpadDecimal,
    NumpadDivide,
    NumpadEnter,
    NumpadEqual,
    NumpadHash,
    NumpadMemoryAdd,
    NumpadMemoryClear,
    NumpadMemoryRecall,
    NumpadMemoryStore,
    NumpadMemorySubtract,
    NumpadMultiply,
    NumpadParenLeft,
    NumpadParenRight,
    NumpadStar,
    NumpadSubtract,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Fn,
    FnLock,
    PrintScreen,
    ScrollLock,
    Pause,
    BrowserBack,
    BrowserFavorites,
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    Eject,
    LaunchApp1,
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,
    Again,
    Copy,
    Cut,
    Find,
    Open,
    Paste,
    Props,
    Select,
    Undo,
    Gamepad0,
    Gamepad1,
    Gamepad2,
    Gamepad3,
    Gamepad4,
    Gamepad5,
    Gamepad6,
    Gamepad7,
    Gamepad8,
    Gamepad9,
    Gamepad10,
    Gamepad11,
    Gamepad12,
    Gamepad13,
    Gamepad14,
    Gamepad15,
    Gamepad16,
    Gamepad17,
    Gamepad18,
    Gamepad19,
    BrightnessDown,
    BrightnessUp,
    DisplayToggleIntExt,
    KeyboardLayoutSelect,
    LaunchAssistant,
    LaunchControlPanel,
    LaunchScreenSaver,
    MailForward,
    MailReply,
    MailSend,
    MediaFastForward,
    MediaPlay,
    MediaPause,
    MediaRecord,
    MediaRewind,
    MicrophoneMuteToggle,
    PrivacyScreenToggle,
    SelectTask,
    ShowAllWindows,
    ZoomToggle,
}
```
