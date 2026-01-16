# VHS Tape: Rename Prompt
# Demonstrates: r to rename, validation, confirmation

Output output/08-rename.gif

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

# Show initial state
Sleep 1s

# Open rename dialog
Type "r"
Sleep 800ms

# Type new name
Type "my_code_review"
Sleep 1s

# Confirm rename
Enter
Sleep 1200ms

# Notice the prompt name changed
Sleep 1500ms

# Quit
Type "q"
Sleep 500ms

# Cleanup
Hide
Type "rm -rf .piemme"
Enter
