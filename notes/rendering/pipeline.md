# Rendering pipeline
This pipeline has similarities to the flutter rendering pipeline, with some
simplifications.

## Currently unsolved
* Repainting optimisations / reusing previous batches
    Hopefully this will become more obvious once the complex vertex
    preprocessing and batching has been implemented.


## Layout
The first stage of the pipeline is the [layout](/layout.md). The layout stage
will map component IDs to screen rectangles. Components can be stored in a tree
to allow for subtree invalidation when we want to reflow everything.

## Paint
The 2nd stage is the [paint](paint.md) stage, where we take a list of
components and generate a list of draw call batches from that.

## Rasterisation
The 3rd and final stage is the most simple as most of the work is done in the
paint stage. The paint stage has already organised VBOs into batches, the
rasterisation stage just needs to draw them with some considerations for not
needing to rebuffer some vertices.

## Storing batches / dirtying batches
With successive repaints, the previous batch list can be passed in to the paint
section of the pipeline to reduce needless vertex re-uploads. Batches should
have a list of component IDs they're dependent on to figure out whether the
batch is 'dirty' or not.
