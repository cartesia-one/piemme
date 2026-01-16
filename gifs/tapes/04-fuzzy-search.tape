# VHS Tape: Fuzzy Search
# Demonstrates: / for search, typing to filter, selecting result

Output output/04-fuzzy-search.gif

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

# Show initial state
Sleep 1s

# Open fuzzy search
Type "/"
Sleep 800ms

# Type search query
Type "code"
Sleep 1s

# Navigate results
Down
Sleep 500ms
Down
Sleep 500ms
Up
Sleep 500ms

# Select result
Enter
Sleep 1s

# Show selected prompt
Sleep 1500ms

# Try another search with Ctrl+p
Ctrl+p
Sleep 800ms

Type "meeting"
Sleep 1s

Enter
Sleep 1500ms

# Quit
Type "q"
Sleep 500ms

# Cleanup
Hide
Type "rm -rf .piemme"
Enter
