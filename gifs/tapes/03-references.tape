# VHS Tape: The Power of References
# Demonstrates: [[reference]] syntax, composing prompts, Ctrl+r to insert, preview mode

Output output/03-references.gif

Set Shell "bash"
Set FontSize 20
Set Width 1400
Set Height 800
Set Theme "Dracula"
Set TypingSpeed 50ms
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

# Navigate to full_code_review which has references
Type "/"
Sleep 600ms
Type "full"
Sleep 800ms
Enter
Sleep 1200ms

# Show the prompt with [[references]] - they should be highlighted green
Sleep 2000ms

# Toggle Preview mode to see resolved references
Type "p"
Sleep 2500ms

# Exit preview
Type "p"
Sleep 1000ms

# Create a new prompt that uses references
Type "n"
Sleep 800ms

Type "Complete code analysis workflow:"
Enter
Enter
Type "Step 1: "
Sleep 400ms

# Use Ctrl+r to insert a reference
Ctrl+r
Sleep 800ms

# Fuzzy search for "debug"
Type "debug"
Sleep 600ms
Enter
Sleep 800ms

# Continue typing
Enter
Enter
Type "Step 2: "
Sleep 400ms

# Insert another reference
Ctrl+r
Sleep 800ms
Type "test"
Sleep 600ms
Enter
Sleep 800ms

# Continue
Enter
Enter
Type "Step 3: Final review"
Sleep 600ms

# Exit editor
Escape
Sleep 500ms
Escape
Sleep 1200ms

# Preview the composed prompt
Type "p"
Sleep 2500ms

# Exit preview
Type "p"
Sleep 800ms

# Copy rendered prompt to clipboard with 'y'
Type "y"
Sleep 1500ms

# Quit
Type "q"
Sleep 500ms

# Cleanup
Hide
Type "rm -rf .piemme"
Enter
