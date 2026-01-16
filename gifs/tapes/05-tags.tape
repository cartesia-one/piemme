# VHS Tape: Tag Management
# Demonstrates: t to tag, tag selector, filtering by tags

Output output/05-tags.gif

Set Shell "bash"
Set FontSize 22
Set Width 1400
Set Height 800
Set Theme "Dracula"
Set TypingSpeed 60ms
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

# Show initial state with tags visible
Sleep 1s

# Open tag selector
Type "t"
Sleep 800ms

# Navigate tags
Down
Sleep 400ms
Down
Sleep 400ms

# Select a tag
Enter
Sleep 800ms

# Show updated prompt
Sleep 1s

# Filter by tag using ]
Type "]"
Sleep 800ms

# Cycle through tag filters
Type "]"
Sleep 800ms

Type "]"
Sleep 800ms

# Filter previous
Type "["
Sleep 800ms

# Clear filter (go back to all)
Type "["
Sleep 800ms
Type "["
Sleep 800ms

# Quit
Type "q"
Sleep 500ms

# Cleanup
Hide
Type "rm -rf .piemme"
Enter
