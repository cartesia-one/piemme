# VHS Tape: Insert Reference
# Demonstrates: Ctrl+r to insert [[reference]] in editor

Output output/09-insert-reference.gif

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

# Create a new prompt
Type "n"
Sleep 800ms

# Type some content
Type "Please help me with the following task:"
Enter
Enter
Type "First, "
Sleep 500ms

# Open reference picker with Ctrl+r
Ctrl+r
Sleep 800ms

# Type to fuzzy find
Type "code"
Sleep 600ms

# Select the reference
Enter
Sleep 800ms

# Continue typing
Enter
Enter
Type "Then apply the feedback to improve the code."
Sleep 1s

# Exit editor
Escape
Sleep 500ms
Escape
Sleep 1200ms

# Show the new prompt with the reference highlighted
Sleep 1500ms

# Quit
Type "q"
Sleep 500ms

# Cleanup
Hide
Type "rm -rf .piemme"
Enter
