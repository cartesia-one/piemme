# VHS Tape: Basic Navigation
# Demonstrates: j/k navigation, g/G, scrolling through prompts

Output output/01-navigation.gif

Set Shell "bash"
Set FontSize 16
Set Width 1000
Set Height 600
Set Theme "Dracula"
Set TypingSpeed 75ms
Set Padding 20

# Setup: copy fixture to .piemme
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

# Navigate down with j
Type "j"
Sleep 600ms
Type "j"
Sleep 600ms
Type "j"
Sleep 600ms

# Navigate up with k
Type "k"
Sleep 600ms
Type "k"
Sleep 600ms

# Jump to last prompt with G
Type "G"
Sleep 800ms

# Jump to first prompt with g
Type "g"
Sleep 800ms

# Navigate with arrow keys
Down
Sleep 500ms
Down
Sleep 500ms
Up
Sleep 500ms

# Show help briefly
Type "?"
Sleep 2s
Escape
Sleep 500ms

# Quit
Type "q"
Sleep 500ms

# Cleanup
Hide
Type "rm -rf .piemme"
Enter
