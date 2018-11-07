# gravity

n-body gravity simulator

![screenshot](assets/gravity.png)

### Running
Execute
```
cargo run --release
```

### Control
Click and drag the mouse to view around, use your mousewheel to zoom in and out.

Press ```f``` to toggle following the particle that is most central to your viewpoint right now.
![screenshot](assets/follow.png)

Press ```v``` to toggle velocity indicators for each particle.
![screenshot](assets/velocity.png)

Press ```a``` to toggle acceleration indicators for each particle.
![screenshot](assets/acceleration.png)

![screenshot](assets/both.png)

Press ```t``` to toggle visualization of the Barnes-Hut tree that is used for approximate force calculations.
![screenshot](assets/tree.png)

Press ```Space``` to freeze and unfreeze the simulation.
