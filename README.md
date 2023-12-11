# Quick Start
A very simple 2D industrial robot like simulation.\
\
Used crates:
```
glium, 0.33.0
winit, 0.29.4
rand, 0.8.5
```

Build and run the project:
```
cargo run
```
# Controls
The robot arm has 3 degrees of freedom with a 2 claws as a tool.\
\
c = clockwise, cc = counter-clockwise
* `Q` and `W` to perform cc- and c-rotation of the base joint
* `A` and `S` to perform cc- and c-rotation of the middle joint
* `Z` and `X` to perform cc- and c-rotation of the top joint
* `1` and `2` to grip and release with the tool
* `T` to teach/save the current position
* `L` to release manually while using the teach functionality


Other operations related to objects:
* `C` to randomly spawn new objects
* `R` to remove the last added object

