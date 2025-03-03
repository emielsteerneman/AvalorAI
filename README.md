# The challenge
Many path planning algorithms can be found on the internet. A few of the more popular ones that I am currently aware of are: Dijkstra, A*, Rapidly Exploring Random Tree, Artificial Potential Fields, Voronoi Diagrams, and Bang Bang trajectories. All of these share a single trait that makes them unusable for the challenge that lies before us: They are useless if you don't know where you want to end up. 

The challenge given to us requires that we both find an optimal path not to a specific destination, but to the highest score. It also needs us to take a certain amount of steps. This makes all of the above mentioned algorithms useless (at least without some modification). An additional requirement is that an solution is needed no later than time T. Of course, with a world size that could approach infinity, and a huge number of steps that need to be taken, it is not possible to guarantee that an solution can always be given before time T. However, it is possible to at least try to give _some_ solution.

An algorithm that is able to deal with an infinite world and a huge number steps, while having to come up with an solution within time T, is one that should or could have the following traits:
* An initial solution is provided in a relatively short timeframe
* Its solution can be improved upon while there is still time left
* It does not require knowledge of the possibly infinite world
* It would be nice if the algorithm can be improved upon in parallel, to take advantage of the hardware it runs on

The current implementations of the algorithms all provide an initial solution relatively quickly. These solutions are stored in a single shared place in memory. The algorithms can provide a better solution at any time. Once the time T has passed, the solution that is currently stored in memory will be used. This approach guarantees that there is alawys _some_ solution ready, while it also allows for better solutions to be found by giving the algorithms more time. The shared memory allows for multiple algorithms to be spun up, allowing maximum use of the hardware. 

# World
The world is represented as a finite 2D integer grid. The world can either be filled with random values or with pseudo-random values using Perlin noise. Sparsity can be applied to the world. Sparsity in a world filled with random world will result in locations being randomly emptied until the sparsity is reached. This results in a world filled with gaps. Sparsity in a world filled with Perlin noise is more complicated. Using the mean and standard deviation of the world, a cutoff threshold is calculated. Every location below the cutoff threshold will be dropped, and the remaining locations will be normalized. This results in a world filled with islands. A higher sparsity results in a more empty world, meaning either more gaps or less and smaller islands. 

# Next steps
## Algorithm
* Create an algorithm that incrementally improves its solution. Current algorithms start from scratch each time, and rely on intermittend random steps to find a better solution.
* Create either 1) algorithms that are aware of each other, 2) an overarching manager able to instruct and guide algorithms to work together.

## World
* Implement regeneration of the world. The world can track which algorithm vists which location at what point in time. Not unlike git, the world would track its incremental difference. Regenration can be implemented in multiple ways. A few examples would be
  - Linear regeneration: Regenerate the value by 1 for every timestep, until the original value has been reached
  - Logarithmic regeneration: The value would regenerate quickly at first, only to slow down when near the original value
  - Random regeneration: The value would not regenerate to the original value, but to a completely new value

# Software
* Implement tests
* Improved logging
* Better edge case management. Some edge cases, such as time T expiring before any algorithm has found a solution, are currently not handled.

## Project management
* Add CICD pipeline for automatic formatting and testing
* Add more documentation
* Add a license