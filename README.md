# dispatcher
Daemon that listens for configured global keypresses that, when detected, invoke corresponding shell commands

## Example `resources/dispatch.toml`
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