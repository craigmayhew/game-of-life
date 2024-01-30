# <img src="./assets/hills-tetrahedron.svg" height="25" /> Game of Life
A unique take on the game of life

<img align="right" src="./assets/screen-capture-2022-08-06.png" height="250" />

### 2022 Update
This project is a unique take on the game of life that gave me an excuse to try some 3D rust programming back in 2019 and 2020 using [Amethyst](https://github.com/amethyst/amethyst). Recently rewritten in [Bevy](https://github.com/bevyengine/bevy). Game mechanics are in and mostly working as of version 0.8.

## Thanks
To the [bevy cheat book](https://bevy-cheatbook.github.io/). 
To Hill for his work on the tetrahedron. 
To AmionSky's [plugin for wavefront files](https://github.com/AmionSky/bevy_obj/tree/master/example). 
To the official bevy [getting started book](https://bevyengine.org/learn/book/getting-started/resources/). 

## Findings

### Conway's original 2d rules
Universes based on the original Conway's game of life rules have endless entropy when faces and edges are checked for neighbours. If life dies where neighbours are less than 2 or more than 3, and is created where neighbours = 3. There is no stable state, no combination of tetras will remain stable.

### Other FAQS
Q: Why didn't you use half a square based pyramid for your tetrahedrons?
A: This would result in 12 tetrahedrons per cube, which felt too high. I also falsely thought at the time hills tetrahedron could make a cube without mirroring any of the tetrahedron. That however turned out to be incorrect and 3 of the tetrahedron have to be mirrored.

Q: Why tetrahedrons and not cubes?
A: I thought it might give more interesting automata



