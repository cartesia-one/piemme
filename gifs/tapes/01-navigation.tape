# VHS Tape: Basic Navigation & UI Overview
# Demonstrates: j/k/g/G navigation, scrolling, panel resize, help overlay

Output output/01-navigation.gif

Set Shell "bash"
Set FontSize 18
Set Width 1600
Set Height 900
Set Theme "Dracula"
Set TypingSpeed 80ms
Set Padding 20

# Setup: copy fixture to .piemme
Hide
Type "cd /var/home/federico/Projects/piemme"
Enter
Sleep 500ms
Type "rm -rf .piemme && cp -r gifs/fixtures/demo-piemme .piemme"
Enter
Sleep 500ms
Type "cargo build --release 2>/dev/null"
Enter
Sleep 5s
Show

# Start piemme
Type "./target/release/piemme"
Enter
Sleep 2s

# Show initial state - display the UI
Sleep 1500ms

# Navigate down with j
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms

# Navigate up with k
Type "k"
Sleep 500ms
Type "k"
Sleep 500ms

# Jump to last prompt with G
Type "G"
Sleep 800ms

# Jump to first prompt with g
Type "g"
Sleep 800ms

# Navigate with arrow keys
Down
Sleep 400ms
Down
Sleep 400ms
Up
Sleep 400ms

# Resize left panel with Ctrl+l (increase)
Ctrl+l
Sleep 400ms
Ctrl+l
Sleep 400ms
Ctrl+l
Sleep 400ms

# Resize left panel with Ctrl+h (decrease)
Ctrl+h
Sleep 400ms
Ctrl+h
Sleep 400ms

# Show help overlay
Type "?"
Sleep 2500ms

# Close help
Escape
Sleep 800ms

# Quit
Type "q"
Sleep 500ms

# Cleanup
Hide
Type "rm -rf .piemme"
Enter
