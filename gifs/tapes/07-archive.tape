# VHS Tape: Archive Operations
# Demonstrates: a to archive, A to view archive, u to unarchive

Output output/07-archive.gif

Set Shell "bash"
Set FontSize 16
Set Width 1000
Set Height 600
Set Theme "Dracula"
Set TypingSpeed 60ms
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

# Navigate to a prompt
Type "j"
Sleep 400ms
Type "j"
Sleep 600ms

# Archive the prompt
Type "a"
Sleep 1200ms

# Prompt disappears from main list
Sleep 1s

# Open archive view
Type "A"
Sleep 1200ms

# We're now in Archive mode - see the archived prompt
Sleep 1500ms

# Unarchive it
Type "u"
Sleep 1200ms

# Exit archive mode
Escape
Sleep 1s

# Prompt is back in main list
Sleep 1500ms

# Quit
Type "q"
Sleep 500ms

# Cleanup
Hide
Type "rm -rf .piemme"
Enter
