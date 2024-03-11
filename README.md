# Quick Start
A simple 2D industrial robot like simulation.\
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
* `1` and `2` to grab and release with the tool
* `B` to automatically return to start position
* `T` to teach/save the current position
* `L` to release manually while using the teach functionality
* `E` to execute each taught step from the beginning



Simulation starts with one single object that you cannot remove. Other operations related to objects:
* `C` to randomly spawn new objects with different colors
* `R` to remove the last added object

# Example Simulation
1. Spawn at least 3 new objects.
2. Move to the center of each object one by one, grab them using `2`. Teach each movement.
3. Stay in the current position and release the object with `L`. This way, you are not able to grab and move the object while teaching.
4. Move the arm to the position where you would like to drop the object without moving the claws.
5. Release each object using `1`.
6. Execute each taught step one by one using `E`.


