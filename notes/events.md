# Events

An event is a function call, sent from a component to an 'event handler' (which
is the parent's component's business logic). It's the same as events on the
web, and is basically an observer pattern.

Events will be defined for a component in the 3pcdf, then code will be
generated to emit this event, and register listeners for this event.

Events can specify a payload, and the code generated will include this payload
in the generated arguments.

As an example, the developer might specify an event called 'selected' for a
dropdown list component, which gives the index of the selected item. This would
probably generate 2 functions:

```
fn reg_listener_selected<F: FnMut(i32)>(f: F);
fn emit_selected(index: i32);
```

reg_listener_selected would never be called by the user though - instead, it
would by called by generated code, and the callback function would simply defer
to a user defined function in the business code.

The 3pcdf file would define which events would be listened to, and what
function in the business logic file they would pass through to. For a quick example with xml:

In 3pcdf file (xml)
```
<dropdown @selected="dropdown_selected"/>
```

In business logic file (rs)
```
fn selected(&mut self, index: i32) {
    // Do stuff with index
}
```

In generated file (also rs)
```
fn on_created(&mut self) {
  reg_listener_selected(|index| {
    self.business_logic.selected(index);
  });
}
```

## Issues with business logic events in separate file

When we do what's described above and assume that the correctly named functions
are there, we get very awkward compiler errors where the error is with the
users code in file B, but the actual output on the console is in the generated
file A, which is quite confusing. This can hopefully be mitigated either with a
custom build script that has a pre-build code lint step (using build.rs) that
checks the right functions are present, or by having the code gen program
'watch' the project file for updates & throw errors after / before code
genning.
