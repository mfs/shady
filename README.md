# shady

## Info

`shady` is a simple utility for playing with GLSL fragment shaders similar to
[Shadertoy](http://www.shadertoy.com). It's written in Rust. I wrote it as I
want to learn GLSL and prefer being able to work in Vim rather than a browser.
This also avoids browser compatibility issues with WebGL.

## Build

```
git clone https://github.com/mfs/shady.git
cd shady
cargo run -- shaders/ray_marching.frag
```

## Use

`shady` takes a single argument, the shader to execute. Like Shadertoy, `shady`
passes in several uniforms available for use in the shader. The following
uniforms are currently available. The GLSL type is in brackets.

- iFrame (int)
- iMouse (vec4)
- iResolution (uvec2)
- iGlobalTime (float)
- iTimeDelta (float)


## Features - TODO

- [x] Reload fragment shader with F5.
- [x] Reloads fragment shader on file change. Uses notify crate.
- [ ] Add some textures.
- [ ] Improve formatting of shader compiler errors.
- [ ] Replace `time` crate with `chrono`
- [x] Supply default shader for when invalid shader is passed in.
- [ ] Tidy up code.
