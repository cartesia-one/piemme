# VHS Tape: Create New Prompt
# Demonstrates: n to create, typing content, auto-naming

Output output/02-create-prompt.gif

Set Shell "bash"
Set FontSize 22
Set Width 1400
Set Height 800
Set Theme "Dracula"
Set TypingSpeed 50ms
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

# Show initial list
Sleep 1s

# Create new prompt with 'n'
Type "n"
Sleep 800ms

# We're now in Insert mode - type the prompt content
Type "Given a Python function, analyze its time complexity."
Enter
Enter
Type "Explain using Big O notation and provide examples."
Sleep 1s

# Exit editor with Escape (saves automatically)
Escape
Sleep 500ms
Escape
Sleep 1s

# Notice the new prompt appeared in the list with auto-generated name
Sleep 1500ms

# Quit
Type "q"
Sleep 500ms

# Cleanup
Hide
Type "rm -rf .piemme"
Enter
