# World
- A simple matrix representing the amount information that can be gathered on each tile
- Finite vs infinite words. Generate more on command

## Generating information
**Random**

**Pseudorandom**
* Cool videon on that: https://www.youtube.com/watch?v=TlLIOgWYVpI
* http://bfnightly.bracketproductions.com/rustbook/
* Perlin noise is probably good

## Regeneration
* Grow to same value or new value?
* Grow at a constant rate, or proportional?

# Path Planning
* Either generate 1 step or generate entire path
  * Steps have to be able to be taken 1 step at a time, to regenerate the world inbetween
  * ? How to measure execution time of just algorithm, when also having to regenerate world?
- If world is static (except for generation), then giving the entire path at once is enough. Then world generation
 also doesn't have to be taken into account when measuring performance

* A good algorithm should be able to : 
  * be able to stop at any time, and have some result ready. That means it should find at least some answer quick, and then improve on that answer while it still has time
  * be able to be scaled up given more hardware

# Performance measurement
If the algorithm is simply given a world and a time 't' in which it has to finish, that should be doable. The algorithm can time itself. However, realistically, a path might be needed at any time, and an algorithm should be able to provide an answer at any time.

But also, if the world is static anyway and all paths are pre-calculated, then why even apply regeneration? Algorithms won't be able to re-plan their path. UNLESS!! Unless. Hear me out, unless the algorithm calls this regeneration step itself.

# Visualizing
How?