# Preface
It is necessary to have read about [The Component System](component_system.md)
before reading this.

# Code Generation
When using a 3pcdf, there needs to
be a way to interface with our business logic. Because Rust is statically
typed, we can't do any dynamic fuckery like we could do in Javascript, and just
generate javascript objects with functions and pass them across to the
developer. It's also pretty nice that we don't have this temptation however,
because this is basically just stringly-typing (covered below).

So, what are our options? 

We basically have 2 main options - stringly typing, and codegen. Codegen will
be used for this project, see below for a brief description of both systems and
potential issues with codegen.

## Stringly typed interfacing
Basically allowing the rust to access data and set event listeners via a hash
map, where strings are keys. Event listening would be done by adding a function
pointer to a hashmap under a key of the event name.

This doesn't take advantage of rust's type system, meaning that errors will
need to be either caught with an impressive linter or found out at runtime -
neither are particularly appetising solutions.

## Code generating and interface from the 3pcdf
The second solution would be to programmatically generate rust code and place
it in a module / crate to be accessed by the developer. The bonus to this is we
take advantage of rust's great type system, plus we get the benefits of any
linters / RLS code that gets developed in the future. In addition, tools like
RustFmt should make the generated code useable, unlike many other code
generation solutions.

## Primary concerns with generated code
It should be very easy to understand what code is generated from the 3pcdf. In
fact, a human should be able to generate working code without the generation
tool without much effort - the generation tool should only really be there for
convenience.

The generated code will really only set out data structures, so performance
of generate code isn't really an issue unless the default data structures
chosen are incredibly poor (which should be an easy fix).

## Potential issues with code generation
A lot of codegen is pretty ugly because it tries to avoid regenerating
previously generated code when simply changing the source (the 3pcdf in our
case). Netbeans for example uses disgusting looking machine-generated comments
throughout the code to split it into sections, allowing the code generator to
only generate what it needs to.

I believe the main reason this is done is because the user is meant to be able
to interact and change the generated code, adding their custom code /
implementations in. If this system wasn't in place, then netbeans would just
generate over all the user's code.

Luckily for us, the planned architecture is to have all generated code placed
in a module which shouldn't need to be modified by the developer.

The main issue is making sure that the architecture allows for this, and making
it clear to any developers not to attempt to write their business logic in the
generated code files.

An additional issue could potentially be that regenerating a large file will be
slow, though I doubt this will be too much of a problem since all the code
we're generating is fairly formulaic and shouldn't need any complex parsing.

