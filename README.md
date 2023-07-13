# truescad
[![Build Status](https://travis-ci.org/hmeyer/truescad.svg?branch=master)](https://travis-ci.org/hmeyer/truescad)

Truescad is a script based CAD program similar to http://www.openscad.org/.
Similar to http://www.implicitcad.org/ Truescad uses implcit functions to represent geometry and hence offers very precise geometry.

In order to generate meshes, e.g. for 3D-printing, Truescad tessellates the geometry into a mesh with arbritrary precision.

Truescad offers rounded CSG, which allows for smooth and rounded looking objects.

![Alt text](doc/true_view.png "accurate geometry view")
![Alt text](doc/tessellated.png "generated mesh")

Truescad is written in Rust.

## Status:
This is a fork of https://github.com/hmeyer/truescad patched to compile against recent glib due to https://github.com/gtk-rs/gtk/issues/821 this project cannot currently be built with recent versions of the Rust toolchain due to switch to non-lexical lifetimes (see: https://jackh726.github.io/rust/2022/06/10/nll-stabilization.html ).

## To build:
rustup toolchain install 1.37  
rustup run 1.37 cargo build --release  

## To run:
rustup run 1.37 cargo run --release

## Usage:

```
cube = Box(1,1,1,0.3)
sphere = Sphere(0.5)
diff = Difference({cube, sphere}, 0.3)
diff=diff:scale(15,15,15)

build(diff)
```
