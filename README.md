Since the overlay didn't work I am trying a different approach:

1. Create a textview with not editable text
2. The textview should show Pango styled text
3. Listen to keypresses and check against static text and addorn each character with a color using Pango.
4. Update the buffertext in the TextView with green for correct char, and red for incorrect.

This way we don't need to use an overlay but work directly on the textview buffer. And although the textview itself is not editable the underlying buffertext is mutable.

Some things to learn/check to solve this approach:

1. How to use Pango in a TextView? (found some example of TextView Pango editor in Python / GTK.
2. How to create a listener to keystrokes and catch them to update the textbuffer contents.
3. Find a way to control set the cursor and programmatically control the cursor?


