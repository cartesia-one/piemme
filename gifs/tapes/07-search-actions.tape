# VHS Tape: Search, Rename, Delete, Duplicate
# Demonstrates: / for search, Ctrl+p quick open, r to rename, d to delete, Ctrl+d to duplicate

Output output/07-search-actions.gif

Set Shell "bash"
Set FontSize 20
Set Width 1400
Set Height 800
Set Theme "Dracula"
Set TypingSpeed 55ms
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

# Open fuzzy search with /
Type "/"
Sleep 800ms

# Type search query - searches names AND content
Type "code"
Sleep 800ms

# Navigate results
Down
Sleep 400ms
Down
Sleep 400ms
Up
Sleep 400ms

# Select result
Enter
Sleep 1200ms

# Show selected prompt
Sleep 1500ms

# Quick open with Ctrl+p
Ctrl+p
Sleep 800ms

Type "eli5"
Sleep 800ms
Enter
Sleep 1200ms

# Show that prompt
Sleep 1500ms

# Rename prompt with 'r'
Type "r"
Sleep 800ms

# Clear and type new name
Backspace
Backspace
Backspace
Backspace
Backspace
Backspace
Backspace
Backspace
Backspace
Backspace
Backspace
Backspace
Sleep 300ms
Type "explain_simple"
Sleep 800ms

# Confirm rename
Enter
Sleep 1200ms

# Notice name changed
Sleep 1500ms

# Duplicate prompt with Ctrl+d
Ctrl+d
Sleep 1200ms

# New duplicate appeared
Sleep 1500ms

# Delete a prompt with 'd'
Type "d"
Sleep 800ms

# Confirm deletion
Type "y"
Sleep 1200ms

# Prompt deleted
Sleep 1500ms

# Search by content
Type "/"
Sleep 600ms
Type "Big O"
Sleep 1000ms
Enter
Sleep 1200ms

# Found prompt containing that text
Sleep 1500ms

# Quit
Type "q"
Sleep 500ms

# Cleanup
Hide
Type "rm -rf .piemme"
Enter
