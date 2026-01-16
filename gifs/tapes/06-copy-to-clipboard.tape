# VHS Tape: Copy to Clipboard
# Demonstrates: y to copy rendered prompt, Ctrl+y for raw

Output output/06-copy-to-clipboard.gif

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

# Navigate to prompt with reference
Type "j"
Sleep 400ms
Type "j"
Sleep 400ms
Type "j"
Sleep 400ms
Type "j"
Sleep 400ms
Type "j"
Sleep 600ms

# Show the prompt with [[reference]]
Sleep 1500ms

# Toggle preview mode to see resolved content
Type "p"
Sleep 1500ms

# Exit preview
Type "p"
Sleep 800ms

# Copy rendered prompt to clipboard
Type "y"
Sleep 1200ms

# Status bar shows "Copied to clipboard!"
Sleep 1s

# Quit
Type "q"
Sleep 500ms

# Cleanup
Hide
Type "rm -rf .piemme"
Enter
