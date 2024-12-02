# Poke Engine

An engine for searching through Pokémon battles (singles only).

**This is not a perfect engine**

This battle engine is meant to capture important aspects of Pokémon for the purposes of competitive single battles.
It is nowhere near as complete or robust as the [PokemonShowdown](https://github.com/smogon/pokemon-showdown) battle engine.

## Links

#### [Python Bindings](poke-engine-py)

#### [CHANGELOG](CHANGELOG.md)

## Running Directly

### Building

Make sure you have Rust / Cargo installed.

[Features](https://doc.rust-lang.org/cargo/reference/features.html) are used to conditionally compile code for different generations of Pokemon.
The simplest way to build the project is with the Makefile.

e.g. To build for generation 4:

```shell
make gen4
```

Run with
    
```shell
./target/release/poke-engine
```

Generations 4 through 8 are available

### Usage

There are several ways to interact with the engine through subcommands:

1. **Generate Instructions**
```shell
poke-engine generate-instructions --state <state-string> -o <s1_move> -t <s2_move>
```
Generate and display the different Instructions that could be applied to the state if side 1 and side 2 used the given moves.

e.g.
```shell
poke-engine generate-instructions --state <state-string> -o shadowball -t breloom
```
```
Index: 0
StateInstruction: 
	Percentage: 80.00
	Instructions:
		Switch SideTwo: P0 -> P2
		Damage SideTwo: 184

Index: 1
StateInstruction: 
	Percentage: 20.00
	Instructions:
		Switch SideTwo: P0 -> P2
		Damage SideTwo: 184
		Boost SideTwo SpecialDefense: -1
```

2. **Expectiminimax**
```shell
poke-engine expectiminimax --state <state-string> --depth <depth> [--ab-prune]
```
Search through the state using [expectiminimax](https://en.wikipedia.org/wiki/Expectiminimax) to the given depth.
Displays the results along with the best move found.

e.g.
```shell
poke-engine expectiminimax --state <state-string> -d 3
```
```
side one options: psychic,grassknot,shadowball,hiddenpowerfire70,switch skarmory,switch tyranitar,switch mamoswine,switch jellicent,switch excadrill
side two options: closecombat,stoneedge,stealthrock,taunt,xscissor,quickattack,switch lucario,switch breloom,switch keldeo,switch conkeldurr,switch toxicroak
matrix: 32.39,11.99,39.72,99.72,-9.94,69.44,55.46,75.91,75.91,75.91,101.19,32.39,-2.94,39.72,99.72,-28.60,69.44,53.51,79.84,108.92,78.63,-23.62,32.39,-20.35,34.37,94.37,-49.04,49.60,53.51,81.39,88.49,89.01,0.00,17.65,-43.57,11.15,71.15,-72.26,26.38,75.91,75.91,65.27,83.70,0.00,-76.18,-85.66,-72.00,-36.99,-34.19,-34.19,-50.07,-11.07,-25.16,-31.11,15.53,-119.69,-85.88,-101.20,-29.40,-100.00,-82.60,-90.04,-107.86,-77.15,-73.11,-25.90,-100.00,-95.17,-118.42,-75.85,-86.53,-86.53,-97.97,-102.52,-83.18,-74.85,-44.47,-45.01,-74.53,-117.55,-45.01,-56.64,-45.01,-84.08,-120.08,-45.01,-74.85,-44.47,-100.00,-47.20,-96.28,-32.62,-52.23,-42.56,-41.19,-120.08,-74.58,-74.85,-41.19
choice: psychic
evaluation: -9.944763
````

3. **Iterative Deepening**
```shell
poke-engine iterative-deepening --state <state-string> --time-to-search-ms <time>
```
Similar to expectiminimax, search through the state but use iterative deepening.
Searches for the given amount of time, then returns the best move found.

e.g.
```shell
poke-engine iterative-deepening --state <state-string> -t 100
```
```
side one options: psychic,switch jellicent,grassknot,shadowball,hiddenpowerfire70,switch skarmory,switch mamoswine,switch excadrill,switch tyranitar
side two options: closecombat,stoneedge,stealthrock,taunt,xscissor,quickattack,switch lucario,switch breloom,switch keldeo,switch conkeldurr,switch toxicroak
matrix: 32.39,11.99,39.72,99.72,-9.94,69.44,55.46,75.91,75.91,75.91,101.19,-45.01,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,32.39,-2.94,39.72,99.72,-28.60,NaN,NaN,NaN,NaN,NaN,NaN,32.39,-20.35,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,17.65,-43.57,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,-76.18,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,-100.00,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,-100.00,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,-119.69,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN,NaN
choice: psychic
evaluation: -9.944763
```

4. **Monte Carlo Tree Search**
```shell
poke-engine monte-carlo-tree-search --state <state-string> --time-to-search-ms <time>
```
Search through the state using [Monte Carlo Tree Search](https://en.wikipedia.org/wiki/Monte_Carlo_tree_search) for the given amount of time.

e.g.
```shell
poke-engine monte-carlo-tree-search --state <state-string> -t 100
```
```
Total Iterations: 25000
side one: switch mamoswine,115.31,300|switch tyranitar,41.00,123|hiddenpowerfire70,58.14,165|switch jellicent,1067.52,2402|switch excadrill,3754.58,8173|shadowball,115.37,300|grassknot,298.20,715|psychic,4038.05,8780|switch skarmory,1826.44,4042
side two: stoneedge,915.55,1723|switch lucario,70.53,159|closecombat,827.19,1562|switch breloom,181.84,373|switch keldeo,141.66,297|stealthrock,413.54,805|quickattack,84.78,187|taunt,123.90,263|xscissor,10745.95,19240|switch conkeldurr,153.71,320|switch toxicroak,26.94,71
```

5. **Calculate Damage**
```shell
poke-engine calculate-damage --state <state-string> -o <s1_move> -t <s2_move>
```
Calculate the damage rolls for the given moves.

e.g.
```shell
poke-engine calculate-damage --state <state-string> -o shadowball -t closecombat
```
```
Damage Rolls: 122,123,125,126,128,129,131,132,133,135,136,138,139,141,142,144
Damage Rolls: 155,157,159,161,162,164,166,168,170,172,173,175,177,179,181,183
```

6. **Interactive Mode**: Run the engine and input commands directly

e.g.
```shell
poke-engine --state <state-string>
```

Available commands:

| Command                                               | Shorthand | Function                                                                                                      |
|-------------------------------------------------------|:---------:|---------------------------------------------------------------------------------------------------------------|
| **state** *state-string*                              |     s     | Reset the state to *state-string*                                                                             |
| **matchup**                                           |     m     | Display some information about the current state                                                              |
| **generate-instructions** *side-1-move* *side-2-move* |     g     | Generate all of the instructions that would be applied to the state if side 1 and side 2 used the given moves |
| **instructions**                                      |     i     | Display the last instructions generated by **generate-instructions**                                          |
| **apply** *instruction-index*                         |     a     | Apply the last instructions instructions to the state, modifying it                                           |
| **pop**                                               |     p     | Pops the last instructions from the state, undoing their changes                                              |
| **pop-all**                                           |    pa     | Pops all applied instructions from the state                                                                  |
| **evaluate**                                          |    ev     | Calculate the current state's evaluation                                                                      |
| **calculate-damage** *side-1-move* *side-2-move*      |     d     | Calculate the damage rolls for the given moves                                                                |
| **expectiminimax** *depth* *[ab-prune=false]*         |     e     | Perform expectiminimax (see above), and display the results                                                   |
| **iterative-deepening** *time-ms*                     |    id     | Perform iterative-deepening (see above), and display the results                                              |
| **monte-carlo-tree-search** *time-ms*                 |   mcts    | Perform monte-carlo-tree-search (see above), and display the results                                          |
| **serialize**                                         |    ser    | Display the current state's serialized string                                                                 |
| **exit/quit**                                         |     q     | Quit interactive mode                                                                                         |


### State Representation

When running directly, the engine parses the state of the game from a string.

Properly representing the state of a Pokémon battle gets really complicated.
See the doctest for `State::deserialize` in [state.rs](src/state.rs)
for the source of truth on how to parse a state string.
