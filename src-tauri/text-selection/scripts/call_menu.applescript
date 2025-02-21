set targetApp to "Chromium"
set theMenu to "Edit"
set theItem to "Copy"

tell application targetApp
    activate
    tell application "System Events"
        tell application process targetApp
            tell menu bar 1
                tell menu bar item theMenu
                    tell menu theMenu
                        click menu item theItem
                    end tell
                end tell
            end tell
        end tell
    end tell
end tell