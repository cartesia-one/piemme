# VHS Tape: Folders and Archive
# Demonstrates: O to open folder, M to move, a to archive, A to view archive, u to unarchive

Output output/06-folders-archive.gif

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

# Open folder selector with O
Type "O"
Sleep 800ms

# Navigate to work folder
Down
Sleep 400ms
Enter
Sleep 1000ms

# Now viewing prompts in work folder - notice title bar shows folder
Sleep 1500ms

# Navigate prompts in folder
Type "j"
Sleep 500ms

# Exit folder back to root with Escape
Escape
Sleep 1000ms

# Back to root view
Sleep 1000ms

# Open personal folder
Type "O"
Sleep 800ms
Down
Sleep 400ms
Down
Sleep 400ms
Enter
Sleep 1200ms

# View personal folder prompts
Sleep 1500ms

# Exit back to root
Escape
Sleep 1000ms

# Move a prompt to a folder with M
Type "j"
Sleep 400ms
Type "M"
Sleep 800ms

# Select work folder
Down
Sleep 400ms
Enter
Sleep 1200ms

# Prompt moved - notice it disappeared from list
Sleep 1500ms

# Archive a prompt with 'a'
Type "j"
Sleep 400ms
Type "a"
Sleep 1200ms

# Prompt archived
Sleep 1200ms

# Open archive view with A
Type "A"
Sleep 1200ms

# View archived prompts - different styling
Sleep 1500ms

# Navigate archive
Type "j"
Sleep 500ms

# Unarchive with 'u'
Type "u"
Sleep 1200ms

# Exit archive
Escape
Sleep 1000ms

# Prompt is back in main list
Sleep 1500ms

# Quit
Type "q"
Sleep 500ms

# Cleanup
Hide
Type "rm -rf .piemme"
Enter
