# 'Paint' stage
This is the stage where we generate vertex data. We need to manually depth sort
due to translucency, but we also need to batch as much as possible for similar
texture batches.

Unfortunately, this results in ~n^2 batches, & because our vertex batches are
already so small this isn't going to be utilising the GPU to its full extent
(actual draw calls take a lot of CPU time, if the GPU finishes before the CPU
then it's just hanging there).

We can eliminate this to some extent with some smart depth sortying & texture
batching, figuring out which bounding boxes overlap / which will actually need
blending etc. However, it's probably best to profile the first option (of just
batching naively) then make sure it's easy to swap between the two methods.

Clipping is also an issue - if we want to clip stuff overflowing a given rect
we need multiple draw calls with tiny batches & a mask. This is probably pretty
horrible for performance, so we might also want another preprocessing step of
altering the actual vertex data to 'fake' clipping. This should be fairly easy
as most of these components will just be quads.

## The 'paint' pipeline
We can develop a mini 'paint' pipeline within the full rendering pipeline,
which includes a vertex preprocessor for manual clipping of geometry and a
batcher which figures out how to batch vertices into draw calls.

Hopefully this mini pipeline abstraction will allow for better optimisation in
the future.

Unfortunately, there isn't really a naive vertex preprocessing step, other than
just a passthrough stage which would result in visual bugs.

### Vertex Preprocessor
Input: List of vertices
Output: List of vertices

Where a vertex is a struct containing x, y, z, r, g, b, a, u, v data

### Batcher
Input: List of vertices
Output: List of batches

Where a batch is a list of vertices with a texture handle to render with.

## Repainting
There are potentially optimisations to be made where a repaint of 1 dirty
component doesn't need to alter any other batches. This means existing VBOs can
be used. It may be beneficial to alter the batcher so that animated /
frequently redrawn components are separated from static components where we can
benefit from remembering the previous draw calls.
