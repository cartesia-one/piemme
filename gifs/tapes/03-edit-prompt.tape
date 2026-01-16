# VHS Tape: Edit Prompt with Vim-style
# Demonstrates: Enter to edit, vim movements, insert mode, saving

Output output/03-edit-prompt.gif

Set Shell "bash"
Set FontSize 16
Set Width 1000
Set Height 600
Set Theme "Dracula"
Set TypingSpeed 50ms
Set Padding 20

# Setup
Hide
Type "cd /var/home/federico/Projects/piemme"
Enter
Sleep 300ms
Type "rm -rf .piemme"
Enter
Sleep 300ms
Type "cp -r gifs/fixtures/demo-piemme .piemme"
Enter
Sleep 300ms
Show

# Start piemme
Type "cargo run --release -q"
Enter
Sleep 2s

# Select a prompt
Type "j"
Sleep 500ms

# Enter editor (Vim Normal mode)
Enter
Sleep 800ms

# Navigate in vim normal mode
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms

# Go to end of file
Type "G"
Sleep 500ms

# Open new line and enter insert mode
Type "o"
Sleep 400ms

# Type new content
Enter
Type "Remember to be concise and clear!"
Sleep 800ms

# Exit insert mode back to vim normal
Escape
Sleep 500ms

# Undo the change
Type "u"
Sleep 800ms

# Redo the change
Ctrl+r
Sleep 800ms

# Exit editor (auto-saves)
Escape
Sleep 1s

# Quit
Type "q"
Sleep 500ms

# Cleanup
Hide
Type "rm -rf .piemme"
Enter
