# Preface
See the first part of this video
https://www.youtube.com/watch?v=UUfXWzp0-DU&feature=youtu.be for ideas relating
to layout.

# Layout
## Layout borders
The aforementioned talk (see preface) mentions layout borders and should be
referred to for additional info.

Ideally, when we change some component's state we'd like to just mark that
component and its children as 'dirty' and in need of a re-layout. However, for
some layouts the size of one component is needed to determine the next (for
example, 'flex' layout where layouts stretch to fill a given width with certain
components given the freedom to expand as much as they need and the other
components working around that). This means that dirtying one component can
actually dirty parent components.

As such, we need to be able to mark certain components as components which have
a layout boundary between them and the parent - i.e. dirtying them doesn't
affect any nodes above.

## 'Soft' layout borders
In addition, there's a discrepency between offset and size. Say we had a
scrolling list of text nodes, where each text node was a different height.
Changing the text of one of the nodes would change the height, end therefore
potentially change the offset of all the other nodes in the list. However, we
don't need to invalidate the whole list - just the node that changed, then do a
minimal 'offset calculation' instead of re-laying out all the text nodes in the
list (which is what would happen if we just dirtied the parent node and called
the standard layout which would normally be a depth first traversal).

## Figuring out layout boundaries / when to dirty the parent
We can do this in 2 ways.

Either, during layout, children who's layout size has changed can be reported
to the parent and the parent will decide whether or not to dirty itself and
re-perform layout.

Or, when state changes before any layout has happened, we can choose to
re-dirty based on node types.

The first approach seems the most stable, for edge cases where things other
than state changes lead to re-layouts.

## Conclusion / Final Implementation architecture
All components in the tree need 1 method and a boolean - layout(), and is_dirty
(self explanetory).

In addition, all component will store their old size from the previous layout -
this is to figure out whether or not their layout size has actually changed.

### The layout() method
First, the parent will recursively call layout() on all the children.

In the body of the layout method, if this node is dirty, then the layout method
will simply take the current calculated sizes of the child nodes and use those
to figure out their offsets, and then this node's final size.

To figure out where there's a layout boundary - i.e. when to pass up a 'dirty'
to the parent - the child passes back up the tree whether or not their size has
changed. The parent then uses this to figure out whether to dirty themselves.
This way we only need to dirty the node who's state or referenced state has
changed, then do 1 walk down and back up the tree.

## Multithreading layout
Since layout is a tree, we can pretty easily offload subtrees to different
worker threads. The issue would be that in order to get around rust's borrowck,
we need some kind of interior mutability guarantee. Simply using Rcs and
get_mut() should be sufficient (with no mutexes needed), so long as the
algorithm definitely doesn't have 2 threads touch the same node.

We could even use a memory arena pattern, where so long as subtrees were
contiguous each thread would just be assigned a slice of the arena.

## Debug rendering
We can forego the next 2 stages in the pipeline (painting and rasterisation)
and just draw coloured rects based on z-level for debugging layout. This would
just be a method to generate a batch (see the [paint](paint.md) stage) given
the list of nodes.
