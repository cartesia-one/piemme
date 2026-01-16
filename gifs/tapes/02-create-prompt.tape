# VHS Tape: Create New Prompt
# Demonstrates: n to create, typing content, auto-naming

Output output/02-create-prompt.gif

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
