# The Component System When I speak about a 'component system', think react or
vue. This general understanding of a 'component' will be assumed.

Components mean you can write some code once, and use it in many places. If we
define components to be responsive, then that component can be included at any
point in the code and look fine, without the parent needing to define anything.

## Component children A component can contain children, which it will be
responsible for laying out.  The amount of children a component can contain
will be defined by that component, and enforced statically.

## Defining component layouts When laying out a component, it will be given a
min / max width and height constraint to stick to. Most of the time, these will
be tight bounds, and the parent will enforce the size of the component.
However, this flexibility needs to be maintained because of things like laying
out lists of texts. 

Say you wanted to have a list of text components, which simply displayed a
string. Given a width, these text components would wrap the text they were
given. They wouldn't take up the same amount of height each time, however.

If we wanted a component which displayed a list of things vertically, then we
would need to define a maximum height for each child. This would mean that text
component children would either be vertically clipped or have huge amounts of
whitespace below - horrible!

As such, in this case we'd want to make the width constant and height
unbounded, then have the height defined by the child.

## Component properties (props) Components have immutable values passed from
the parent called 'properties'.  The properties which can / need to be passed
in from the parent are defined in the 3pcdf. They can be passed into child
components or used for custom rendering.

## Component state Just like in react / vue, a component might want to have
both local and global state. See the [State Management](state.md)
section for info on global state. This section will only cover local state -
state which is visible to only this component.

Each component will contain some amount of mutable typed state. This will be
defined in the 3pcdf, then using [code generation](code_generation.md) we can
create variables in rust matching this state's type and naming.

State will be defined in the 3pcdf so that it can be referenced there, to be
passed into child components as properties, or used for custom rendering.

State is effectively the same as properties, except it is mutable and not
defined by the parent. If you need the parent to define some child state, then
the parent must pass in a property and the child must initialise the state to
that property.

## Component representation
Component instances will have an ID associated with them, which can be used for
referencing in layout or paint trees. This also lends itself to an accompanying
ECS.

## Component slots
'Slots' are used to insert components as children into parts of the component.
For example, consider a 'container' node which simply centres its child. That
child will change depending on what the parent passes to it.

The concept of 'slots' doesn't exist in the rust code - instead, component
instances are just passed into constructors. Slots are simply a way of
describing how a component can be inserted into a slot in the 3pcdf.

## Component object
The component object will need the following:
- Layout method (Given size constraints computes size)
- Paint method (Given a rect position generates vertex data)
- Any local state
- Function
- Component ID creation (global mutable counter)

### Trait functions
The following will be trait defined functions:
- get_size() -> Size
- layout() -> Box<Fn(constraints) -> new size>
- paint(rect)
- get_children() -> Vec<Box<Component>>

