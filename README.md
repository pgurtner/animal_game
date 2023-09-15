# animal_game
Uses animal_game_engine which is developed in parallel.

## name
Something more concrete would be nice.

## some coding aspects/goals
* Rust
* Entity Component System
* keeping libraries away from the program core
* Test Driven Development
* decentralized algorithms
* parallelism

## goal
An ecosystem simulator similar to [Rainworld](https://rainworldgame.com/). Maybe turned into a game if I get a nice idea that wouldn't just copy Rainworld.

## 'Roadmap'
Every part should be able to bear future changes and extensions to allow programming without too detailed plans.

The lists in the subsections are **not** exhaustive.

### general interfaces
* ECS
* abstraction of game engine
  * todo: 

### animal senses
Generally perceiving the environment.
* seeing
* hearing

#### (potential) future extensions
* touching
* smelling?

### animal reflexes
Vary basic linear reactions to perceptions
* attacking
* fleeing
* eating

#### (potential) future extensions
* hiding

### behaviour (todo)
The separation of the animal 'brain' is still unclear. The behaviour features include:
* personality: a small set of fundamental aspects (like nervousness, impulsiveness, ...) that varies in weighting per animal
* social memory: remembering interaction with others to some extent to cause things like groups, friendship and revenge
* dynamic food chain position: no fixed numbers or similar, a worm should (theoretically) be able to kill a giant

Further ideas:
* evolution + mating: don't just spawn animals but let them pair, the children's personality is mix of the parents' with little random changes (additionally maybe even changes of physical form)
* holding back reflexes: e.g. keeping food instead of directly eating it