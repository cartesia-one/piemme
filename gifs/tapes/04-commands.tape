# VHS Tape: The Power of Commands
# Demonstrates: {{command}} syntax, safe mode, command execution on copy

Output output/04-commands.gif

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

# Navigate to project_context which has commands
Type "/"
Sleep 600ms
Type "project"
Sleep 800ms
Enter
Sleep 1200ms

# Show the prompt with {{commands}} - highlighted in yellow/orange
Sleep 2000ms

# Copy the prompt - this will trigger safe mode confirmation
Type "y"
Sleep 1500ms

# Confirm command execution
Type "y"
Sleep 2000ms

# Show success
Sleep 1000ms

# Navigate to write_commit prompt
Type "/"
Sleep 600ms
Type "commit"
Sleep 800ms
Enter
Sleep 1200ms

# Show this prompt with git command
Sleep 2000ms

# Toggle safe mode off with '!'
Type "!"
Sleep 1200ms

# Now copy without confirmation
Type "y"
Sleep 2000ms

# Toggle safe mode back on
Type "!"
Sleep 1000ms

# Create a new prompt with dynamic content
Type "n"
Sleep 800ms

Type "Current system info:"
Enter
Enter
Type "User: {{whoami}}"
Enter
Type "Date: {{date +%Y-%m-%d}}"
Enter
Type "Directory: {{pwd}}"
Sleep 1000ms

# Exit editor
Escape
Sleep 500ms
Escape
Sleep 1200ms

# Preview to see resolved commands
Type "p"
Sleep 2500ms

# Exit preview
Type "p"
Sleep 800ms

# Quit
Type "q"
Sleep 500ms

# Cleanup
Hide
Type "rm -rf .piemme"
Enter
