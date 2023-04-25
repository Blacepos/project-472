# CSE 472 Final Project - Ray Marching


I decided to forgo all performance optimizations for the sake of my own sanity. This mostly amounts to me doing a lot of copying into structs in the fragment shader instead of reading the big scene buffer directly.

Material blending is not supported for models combined using a binary operation.

# Modeling

Modeling can be done on the Rust side using the `modeling` module. The main idea is to create `Object`s which are composed of a `model` and a `material`. A `model` can be a primitive or a primitive that has been operated on. For example, you can have a `RectPrism` as the model or a `RectPrism` wrapped in a `Rounding` operation. The latter will tell the shader to apply `op_rounding` to the `RectPrism` before including its distance estimation in the scene.
The `material` is just composed of Blinn-Phong attributes. I also added a `reflectivity` attribute for extensibility, but it is currently unimplemented in the shader since there are no reflections.

# Rendering

- Soft shadows are automatic and do not reflect the actual properties of the light.

# Credits
- Ray marching explanation and motivation for this project: https://www.youtube.com/watch?v=svLzmFuSBhk
- Distance estimation for primitives and operations: https://iquilezles.org/articles/distfunctions/
  - Included ShaderToy demos
- Past assignments in this class for doing Blinn-Phong correctly and working with shaders