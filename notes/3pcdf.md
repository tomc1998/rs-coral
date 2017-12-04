# Preface
See [Components](component_system.md) first.

# 3pcdf
This is a document giving a general overview of the third party component
definition format which will be used to define components, alongside Rust
(which will provide the business logic of the component).

## What does a component need to define?
At its core, other than obvious meta-data like the component's name, a
component needs to define 3 main things:

* Business logic, which are lists of functions called on certain events
  (whether from child components or from the system, like a 'on created' event)
* A definition of child components and their layouts, plus min / max layout sizes.
* A definition of inputs and outputs, along with local and global [state](state.md)
    An list of possible inputs and outputs are:
    - Properties
    - Events to be emitted
    - Slots

Business logic will be defined in rust code, with everything else being defined
in the 3pcdf files, and code generated from that.

## Multiple layouts for each component & motivations
One might want to define multiple layouts for each component. There would be 2
reasons for this:

* Multiple layout 'themes', aesthetically it might look better in different
  contexts
* A more drastic change of layout might be necessary for responsive design of
  components

However, inputs and outputs, plus any local state or global state bindings will
remain constant across layouts. For this reason, a definition of a component
should occur over multiple files / multiple sections of a file.

## Initial proposition for component file structures

A component will be contained in a folder, named the same as the component
(case-insensitive). The top level of this folder will contain a file named
something like `index.XXX` or `mod.XXX`, with XXX being whatever filetype is
being used. This file will contain definitions of inputs / outputs and state
(see list above), as well as absolute size constraints (i.e. this component
will not render smaller than 320 x 320), plus meta data like component name /
description.

Also contained in this folder will be more directories, each named according to
one of the 'themes' available for this component. The theme will be able to be
customised by parent components.

Inside each of the theme directories will be a list of `.XXX` files which
define layouts for different view sizes. At the top of each filetype will be a
max-width / max-height, or min-width / min-height declaration, which define the
range of sizes for this layout to be applied. Conflicts between themes (i.e.
unclear which theme to apply) will be brought up as an error by the code
generator.
