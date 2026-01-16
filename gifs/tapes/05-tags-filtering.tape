# VHS Tape: Tags and Filtering
# Demonstrates: t to tag, tag selector, [/] cycling, filtering prompts

Output output/05-tags-filtering.gif

Set Shell "bash"
Set FontSize 20
Set Width 1400
Set Height 800
Set Theme "Dracula"
Set TypingSpeed 60ms
Set Padding 20

# Setup
Hide
Type "cd /var/home/federico/Projects/piemme"
Enter
Sleep 300ms
Type "rm -rf .piemme && cp -r gifs/fixtures/demo-piemme .piemme"
Enter
Sleep 300ms
Type "cargo build --release 2>/dev/null"
Enter
Sleep 5s
Show

# Start piemme
Type "./target/release/piemme"
Enter
Sleep 2s

# Show initial state - notice tag colors in list
Sleep 1500ms

# Cycle through tag filters with ]
Type "]"
Sleep 800ms

# Notice the list is filtered - status bar shows filter
Type "]"
Sleep 800ms

Type "]"
Sleep 800ms

Type "]"
Sleep 800ms

# Cycle back with [
Type "["
Sleep 800ms

Type "["
Sleep 800ms

# Go back to show all
Type "["
Sleep 800ms
Type "["
Sleep 800ms
Type "["
Sleep 800ms

# Open tag selector on a prompt
Type "t"
Sleep 800ms

# Navigate tags
Down
Sleep 400ms
Down
Sleep 400ms

# Select a tag (toggle it)
Enter
Sleep 800ms

# Show updated prompt
Sleep 1200ms

# Open tag selector again
Type "t"
Sleep 800ms

# Create a new tag with Ctrl+n
Ctrl+n
Sleep 600ms
Type "urgent"
Sleep 600ms
Enter
Sleep 1000ms

# Tag applied
Sleep 1200ms

# Filter by the new tag
Type "]"
Sleep 600ms
Type "]"
Sleep 600ms
Type "]"
Sleep 600ms
Type "]"
Sleep 600ms
Type "]"
Sleep 800ms

# Show filtered by urgent
Sleep 1500ms

# Back to all
Type "["
Sleep 500ms
Type "["
Sleep 500ms
Type "["
Sleep 500ms
Type "["
Sleep 500ms
Type "["
Sleep 800ms

# Quit
Type "q"
Sleep 500ms

# Cleanup
Hide
Type "rm -rf .piemme"
Enter
