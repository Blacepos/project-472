# CSE 472 Final Project - Ray Marching


I decided to forgo all performance optimizations for the sake of my own sanity. This mostly amounts to me doing a lot of copying into structs in the fragment shader instead of reading the big scene buffer directly.

Material blending is not supported for models combined using a binary operation. This is reflected in the modeling API described below.

# Modeling

Modeling can be done on the Rust side using the `modeling` module. The main idea is to create `Object`s which are composed of a `model` and a `material`. A `model` can be a primitive or a primitive that has been operated on. For example, you can have a `RectPrism` as the model or a `RectPrism` wrapped in a `Rounding` operation. The latter will tell the shader to apply `op_rounding` to the `RectPrism` before including its distance estimation in the scene.
The `material` is just composed of Blinn-Phong attributes. I also added a `reflectivity` attribute for extensibility, but it is currently unimplemented in the shader since there are no reflections.

# Rendering

## Soft shadows
Can be approximated easily by tracking the scene distance when shooting the shadow feeler. Instead of shooting a bunch of rays at slightly different angles, you keep track of the minimum scene distance while going towards the light. The shadows do not reflect the actual properties of the light and are only there for effect.

## Model operations
Since the scene is described using distances, it's not too difficult to significantly modify the way objects appear. For example, you can just subtract a radius from the distance and the object will appear rounded. A lot of the formulas for these operations can be found [here](https://iquilezles.org/articles/distfunctions/).

# Credits
- Ray marching explanation and motivation for this project: https://www.youtube.com/watch?v=svLzmFuSBhk
- Distance functions for primitives and operations: https://iquilezles.org/articles/distfunctions/
- Past assignments in this class for doing Blinn-Phong correctly and working with shaders.