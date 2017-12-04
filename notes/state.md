# State
State management is a crucial part of any application. The best way (for me) is
to separate state into local and global state, where global state is stuff like
the result of API requests being used throughout the app, and local state is
stuff like little booleans for changing components in a small way temporarily.

The model for global state to be used will be the 'store' model, as primarily
seen in 'vuex' but also in 'redux' and 'flux'. The structure of the store will
be defined in the 3pcdf, and a rust structure generated from this to match. The
store will have methods to register change listeners, and every time some part
of the store is mutated these change listeners will be fired in order to
reactively update the UI.

## Proper encapsulation
To truely harness the benefits of the single, application-wide store, one needs
to define mutations, getters, and actions to act upon the store. Keeping the
rest of the store private is important as this means we can be confident of the
mutations that have happened previously, leading to less bugs and more
debuggable application state.

### Quick definition of terms
Not glossary-worthy, but important to make sure there is no ambiguity:

* Mutation - A simple alteration of the data which completes and returns
  instantly, and always succeeds.
* Action - A more complex alteration of data which often involves API calls and
  returns a Future containing a Result.
* Getter - More than the standard 'getter', this allows for more complex
  relational data to be stored in a 'normalised' way then retrieved in a more
  object oriented way by abstracting the process of matching up IDs through a
  getter. These are less common in smaller apps.
  Not to be confused with the normal notion of 'getter', which does not exist
  in this paradigm. An explanation of this is below.

### Allowing users to define mutations, but auto generating store boilerplate
Store data items should be invisible to all code except that of the user
defined mutations. This is fine if the users are defining their code in the
same module - however, as a best practice for code gen, it's nice to have all
generated code go under a module with the name `gen` at some point. This is
helpful to devs so that they know what not to modify, and interspersing
generated code gets ugly quite fast.

The following system is proposed:

- Codegen will generate an inner `InnerState` which contains public fields.
- The public fields of InnerState will be the fields stated in the 3pcdf, except
  wrapped in Mutexes.
- Codegen will also generate a wrapper `State` which not only owns an InnerState,
  but also contains a function for applying a mutation. 
- The InnerState will be private and not accessible from any other code.
- A mutation is an object with 1 method called 'mutate', and this method
  is passed an InnerState reference, hence allowing access to actual state. All
  mutation will implement the `Mutation` trait.
- Mutations can be 'applied' to state by calling an `apply` method on the
  wrapper State object. This will take a mutation trait object and call the
  mutate method, passing in a reference to the private `InnerState` field.

Following this, mutations can easily be batched and committed with a `commit`
method without needing the developer to insert this every time. This will
prevent all of the reactive listeners from firing all the time.

In addition, mutations all go through 1 central bottleneck and can easily be
debugged, and state is separately lockable meaning multithreading business code
should be an easy process. 

## Getting store data, and normal 'getters'
Normal, auto-generated getters for simply getting store properties cannot
really work, as the mutex rules for rust would require the readers to somehow
drop their references when it came time to mutate the data.

This can be worked around with a mutex, but that then means that components
must hold Arc<Mutex<T>>, and make sure they don't hold a lock for too long.
Holding a lock for a long time would introduce subtle bugs and deadlocks when
trying to mutate the data.

As such, any getters (auto generated or user defined) will simply copy the
value to make sure there are no conflicts.

Most times you'd be required to 'get' data in a normal application, however,
can be sidestepped using the reactive paradigm & registering listeners to
different parts of state. These listener registrations will be code generated
on components as specified in the 3pcdf, and when the state is changed the
listeners will be fired & component state updated automatically without the
developer doing anything.

Traditional getters can therefore be pretty much avoided other than in certain parts
of the business logic, where they can just be copied out.

## What can actions and getters access?
Actions and getters will be similar to mutations in that they are
implementations of a trait that are 'applied' to the wrapping state object and
therefore logged & debuggable, but will not be able to access the actual state.
All state mutation will go through the mutations, and any 'getting' of values
will happen through the auto-generated copy only getters.
