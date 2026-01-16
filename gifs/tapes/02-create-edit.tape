# VHS Tape: Creating and Editing Prompts
# Demonstrates: n to create, vim editing (i/o/u), Escape to save

Output output/02-create-edit.gif

Set Shell "bash"
Set FontSize 18
Set Width 1600
Set Height 900
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

# Create new prompt with 'n'
Type "n"
Sleep 800ms

# Now in Insert mode - type content
Type "Analyze this algorithm complexity"
Enter
Enter
Type "Consider time and space complexity."
Enter
Type "Use Big O notation."
Sleep 1200ms

# Exit insert mode -> vim normal
Escape
Sleep 600ms

# Exit editor -> save and return to list
Escape
Sleep 1200ms

# New prompt appeared with auto-generated name
Sleep 1500ms

# Edit existing prompt - select it and press Enter
Type "j"
Sleep 400ms
Enter
Sleep 800ms

# We're in Vim Normal mode - navigate to end
Type "G"
Sleep 500ms

# Open new line below with 'o' (enters insert mode)
Type "o"
Sleep 400ms

# Type additional content
Type "Be thorough and precise."
Sleep 800ms

# Exit insert mode back to vim normal
Escape
Sleep 600ms

# Undo the change with 'u'
Type "u"
Sleep 1000ms

# Exit editor and save
Escape
Sleep 1200ms

# Quit
Type "q"
Sleep 500ms

# Cleanup
Hide
Type "rm -rf .piemme"
Enter
