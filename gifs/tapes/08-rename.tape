# VHS Tape: Rename Prompt
# Demonstrates: r to rename, validation, confirmation

Output output/08-rename.gif

Set Shell "bash"
Set FontSize 22
Set Width 1400
Set Height 800
Set Theme "Dracula"
Set TypingSpeed 50ms
Set Padding 20

# Setup: copy fixture and pre-build
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
Type "cargo build --release 2>/dev/null"
Enter
Sleep 5s
Show

# Start piemme (using pre-built binary)
Type "./target/release/piemme"
Enter
Sleep 3s

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
