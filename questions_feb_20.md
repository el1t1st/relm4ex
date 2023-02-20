# 0. Where do you need to initialize the values? In the main or in the fn init? I get an error when running the app. 

# 1. How do you activate the textag_table for textview, so you can use add_tag on it? I assume that the compiler complains that it cannot find the tag background because it doesn't have access to a texttabtable?
This is fixed. 

I checked:
- [https://docs.gtk.org/gtk4/class.TextTagTable.html](https://docs.gtk.org/gtk4/class.TextTagTable.html)
- apply the tag to a char with iter(location) and iter_end(location+1)
- set current_index +1

# 2. How to listen to keypresses, and connect it to TextView widget specifically. I found the following code below, but it uses a connect method on the Window, which doesn't exist 'anymore'. 

```rust
window
    .connect("key_press_event", false, |values| {
        // "values" is a 2-long slice of glib::value::Value, which wrap G-types
        // You can unwrap them if you know what type they are going to be ahead of time
        // values[0] is the window and values[1] is the event
        let raw_event = &values[1].get::<gdk::Event>().unwrap().unwrap();
        // You have to cast to the correct event type to access some of the fields
        match raw_event.downcast_ref::<gdk::EventKey>() {
            Some(event) => {
                println!("key value: {:?}", std::char::from_u32(event.get_keyval()));
                println!("modifiers: {:?}", event.get_state());
            },
            None => {},
        }

        // You need to return Some() of a glib Value wrapping a bool
        let result = glib::value::Value::from_type(glib::types::Type::Bool);
        // I can't figure out how to actually set the value of result
        // Luckally returning false is good enough for now.
        Some(result)
    })
    .unwrap();
```
Another example is [https://dev.to/antonov_mike/listen-to-the-keyboard-events-with-rust-and-gtk-f1h](https://dev.to/antonov_mike/listen-to-the-keyboard-events-with-rust-and-gtk-f1h).

I asked the dev who developed Relm and he suggested to use :

```
I think you need to add an event controller to the window: https://gtk-rs.org/gtk4-rs/git/docs/gtk4/struct.EventControllerKey.html#method.connect_key_pressed

Use this to add the controller: https://gtk-rs.org/gtk4-rs/git/docs/gtk4/prelude/trait.WidgetExt.html#tymethod.add_controller
```

But the information I read, doesn't give me enough to actually understand what parameters are available for the method and the controller. 

**NEXT: What I want to do is to listen to a keypress. Knowing the current_index and taking the char from the base_text I can then compare the keypress value with the value in the current_index, and then set the tag to red or green, depending on correct or incorrect.**

# 3. How to tell TextView to use Pango? Or does that work automatically?
Works out of the box, if you use the texttags.

<!-- # 4. Making an editor TextView in Rust from a python/gtk approach? -->
<!-- **NEXT: Convert the example in [Python/GTK3]() to a Rust/GTK4.** -->
<!-- [https://github.com/antonovmike/gtk_keyboard_events_listener](https://github.com/antonovmike/gtk_keyboard_events_listener) -->

# 4. How to select one character through 2 iterators ... start and end? 
