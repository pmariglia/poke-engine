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
poke-engine generate-instructions --state "alakazam,100,Psychic,Typeless,251,251,MAGICGUARD,LIFEORB,121,148,353,206,365,None,0,25.5,PSYCHIC;false;16,GRASSKNOT;false;32,SHADOWBALL;false;24,HIDDENPOWERFIRE70;false;24,NONE;true;32,NONE;true;32=skarmory,100,Steel,Flying,271,271,STURDY,CUSTAPBERRY,259,316,104,177,262,None,0,25.5,STEALTHROCK;false;32,SPIKES;false;32,BRAVEBIRD;false;24,THIEF;false;40,NONE;true;32,NONE;true;32=tyranitar,100,Rock,Dark,404,404,SANDSTREAM,CHOPLEBERRY,305,256,203,327,159,None,0,25.5,CRUNCH;false;24,SUPERPOWER;false;8,THUNDERWAVE;false;32,PURSUIT;false;32,NONE;true;32,NONE;true;32=mamoswine,100,Ice,Ground,362,362,THICKFAT,NEVERMELTICE,392,196,158,176,241,None,0,25.5,ICESHARD;false;48,EARTHQUAKE;false;16,SUPERPOWER;false;8,ICICLECRASH;false;16,NONE;true;32,NONE;true;32=jellicent,100,Water,Ghost,404,404,WATERABSORB,AIRBALLOON,140,237,206,246,180,None,0,25.5,TAUNT;false;32,NIGHTSHADE;false;24,WILLOWISP;false;24,RECOVER;false;16,NONE;true;32,NONE;true;32=excadrill,100,Ground,Steel,362,362,SANDFORCE,CHOICESCARF,367,156,122,168,302,None,0,25.5,EARTHQUAKE;false;16,IRONHEAD;false;24,ROCKSLIDE;false;16,RAPIDSPIN;false;64,NONE;true;32,NONE;true;32=0=0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;==0=0=0=0=0=0=0=0=0=0=false=NONE=false=false=switch:0=false/terrakion,100,Rock,Fighting,323,323,JUSTIFIED,FOCUSSASH,357,216,163,217,346,None,0,25.5,CLOSECOMBAT;false;8,STONEEDGE;false;8,STEALTHROCK;false;32,TAUNT;false;32,XSCISSOR;false;24,QUICKATTACK;false;48=lucario,100,Fighting,Steel,281,281,JUSTIFIED,LIFEORB,350,176,241,177,279,None,0,25.5,CLOSECOMBAT;false;8,EXTREMESPEED;false;8,SWORDSDANCE;false;32,CRUNCH;false;24,ICEPUNCH;false;24,AURASPHERE;false;32=breloom,100,Grass,Fighting,262,262,TECHNICIAN,LIFEORB,394,196,141,156,239,None,0,25.5,MACHPUNCH;false;48,BULLETSEED;false;48,SWORDSDANCE;false;32,LOWSWEEP;false;32,DRAINPUNCH;false;16,PROTECT;false;16=keldeo,100,Water,Fighting,323,323,JUSTIFIED,LEFTOVERS,163,216,357,217,346,None,0,25.5,SECRETSWORD;false;16,HYDROPUMP;false;8,SCALD;false;24,SURF;false;24,HIDDENPOWERICE70;false;24,CALMMIND;false;32=conkeldurr,100,Fighting,Typeless,414,414,GUTS,LEFTOVERS,416,226,132,167,126,None,0,25.5,MACHPUNCH;false;48,DRAINPUNCH;false;16,ICEPUNCH;false;24,THUNDERPUNCH;false;24,BULKUP;false;32,PAYBACK;false;16=toxicroak,100,Poison,Fighting,307,307,DRYSKIN,LIFEORB,311,166,189,167,295,None,0,25.5,DRAINPUNCH;false;16,SUCKERPUNCH;false;8,SWORDSDANCE;false;32,ICEPUNCH;false;24,POISONJAB;false;32,SUBSTITUTE;false;16=0=0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;==0=0=0=0=0=0=0=0=0=0=false=NONE=false=false=switch:0=false/none;5/none;5/false/false" -o shadowball -t breloom
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
poke-engine expectiminimax --state "alakazam,100,Psychic,Typeless,251,251,MAGICGUARD,LIFEORB,121,148,353,206,365,None,0,25.5,PSYCHIC;false;16,GRASSKNOT;false;32,SHADOWBALL;false;24,HIDDENPOWERFIRE70;false;24,NONE;true;32,NONE;true;32=skarmory,100,Steel,Flying,271,271,STURDY,CUSTAPBERRY,259,316,104,177,262,None,0,25.5,STEALTHROCK;false;32,SPIKES;false;32,BRAVEBIRD;false;24,THIEF;false;40,NONE;true;32,NONE;true;32=tyranitar,100,Rock,Dark,404,404,SANDSTREAM,CHOPLEBERRY,305,256,203,327,159,None,0,25.5,CRUNCH;false;24,SUPERPOWER;false;8,THUNDERWAVE;false;32,PURSUIT;false;32,NONE;true;32,NONE;true;32=mamoswine,100,Ice,Ground,362,362,THICKFAT,NEVERMELTICE,392,196,158,176,241,None,0,25.5,ICESHARD;false;48,EARTHQUAKE;false;16,SUPERPOWER;false;8,ICICLECRASH;false;16,NONE;true;32,NONE;true;32=jellicent,100,Water,Ghost,404,404,WATERABSORB,AIRBALLOON,140,237,206,246,180,None,0,25.5,TAUNT;false;32,NIGHTSHADE;false;24,WILLOWISP;false;24,RECOVER;false;16,NONE;true;32,NONE;true;32=excadrill,100,Ground,Steel,362,362,SANDFORCE,CHOICESCARF,367,156,122,168,302,None,0,25.5,EARTHQUAKE;false;16,IRONHEAD;false;24,ROCKSLIDE;false;16,RAPIDSPIN;false;64,NONE;true;32,NONE;true;32=0=0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;==0=0=0=0=0=0=0=0=0=0=false=NONE=false=false=switch:0=false/terrakion,100,Rock,Fighting,323,323,JUSTIFIED,FOCUSSASH,357,216,163,217,346,None,0,25.5,CLOSECOMBAT;false;8,STONEEDGE;false;8,STEALTHROCK;false;32,TAUNT;false;32,XSCISSOR;false;24,QUICKATTACK;false;48=lucario,100,Fighting,Steel,281,281,JUSTIFIED,LIFEORB,350,176,241,177,279,None,0,25.5,CLOSECOMBAT;false;8,EXTREMESPEED;false;8,SWORDSDANCE;false;32,CRUNCH;false;24,ICEPUNCH;false;24,AURASPHERE;false;32=breloom,100,Grass,Fighting,262,262,TECHNICIAN,LIFEORB,394,196,141,156,239,None,0,25.5,MACHPUNCH;false;48,BULLETSEED;false;48,SWORDSDANCE;false;32,LOWSWEEP;false;32,DRAINPUNCH;false;16,PROTECT;false;16=keldeo,100,Water,Fighting,323,323,JUSTIFIED,LEFTOVERS,163,216,357,217,346,None,0,25.5,SECRETSWORD;false;16,HYDROPUMP;false;8,SCALD;false;24,SURF;false;24,HIDDENPOWERICE70;false;24,CALMMIND;false;32=conkeldurr,100,Fighting,Typeless,414,414,GUTS,LEFTOVERS,416,226,132,167,126,None,0,25.5,MACHPUNCH;false;48,DRAINPUNCH;false;16,ICEPUNCH;false;24,THUNDERPUNCH;false;24,BULKUP;false;32,PAYBACK;false;16=toxicroak,100,Poison,Fighting,307,307,DRYSKIN,LIFEORB,311,166,189,167,295,None,0,25.5,DRAINPUNCH;false;16,SUCKERPUNCH;false;8,SWORDSDANCE;false;32,ICEPUNCH;false;24,POISONJAB;false;32,SUBSTITUTE;false;16=0=0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;==0=0=0=0=0=0=0=0=0=0=false=NONE=false=false=switch:0=false/none;5/none;5/false/false" -d 3
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
poke-engine iterative-deepening --state "alakazam,100,Psychic,Typeless,251,251,MAGICGUARD,LIFEORB,121,148,353,206,365,None,0,25.5,PSYCHIC;false;16,GRASSKNOT;false;32,SHADOWBALL;false;24,HIDDENPOWERFIRE70;false;24,NONE;true;32,NONE;true;32=skarmory,100,Steel,Flying,271,271,STURDY,CUSTAPBERRY,259,316,104,177,262,None,0,25.5,STEALTHROCK;false;32,SPIKES;false;32,BRAVEBIRD;false;24,THIEF;false;40,NONE;true;32,NONE;true;32=tyranitar,100,Rock,Dark,404,404,SANDSTREAM,CHOPLEBERRY,305,256,203,327,159,None,0,25.5,CRUNCH;false;24,SUPERPOWER;false;8,THUNDERWAVE;false;32,PURSUIT;false;32,NONE;true;32,NONE;true;32=mamoswine,100,Ice,Ground,362,362,THICKFAT,NEVERMELTICE,392,196,158,176,241,None,0,25.5,ICESHARD;false;48,EARTHQUAKE;false;16,SUPERPOWER;false;8,ICICLECRASH;false;16,NONE;true;32,NONE;true;32=jellicent,100,Water,Ghost,404,404,WATERABSORB,AIRBALLOON,140,237,206,246,180,None,0,25.5,TAUNT;false;32,NIGHTSHADE;false;24,WILLOWISP;false;24,RECOVER;false;16,NONE;true;32,NONE;true;32=excadrill,100,Ground,Steel,362,362,SANDFORCE,CHOICESCARF,367,156,122,168,302,None,0,25.5,EARTHQUAKE;false;16,IRONHEAD;false;24,ROCKSLIDE;false;16,RAPIDSPIN;false;64,NONE;true;32,NONE;true;32=0=0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;==0=0=0=0=0=0=0=0=0=0=false=NONE=false=false=switch:0=false/terrakion,100,Rock,Fighting,323,323,JUSTIFIED,FOCUSSASH,357,216,163,217,346,None,0,25.5,CLOSECOMBAT;false;8,STONEEDGE;false;8,STEALTHROCK;false;32,TAUNT;false;32,XSCISSOR;false;24,QUICKATTACK;false;48=lucario,100,Fighting,Steel,281,281,JUSTIFIED,LIFEORB,350,176,241,177,279,None,0,25.5,CLOSECOMBAT;false;8,EXTREMESPEED;false;8,SWORDSDANCE;false;32,CRUNCH;false;24,ICEPUNCH;false;24,AURASPHERE;false;32=breloom,100,Grass,Fighting,262,262,TECHNICIAN,LIFEORB,394,196,141,156,239,None,0,25.5,MACHPUNCH;false;48,BULLETSEED;false;48,SWORDSDANCE;false;32,LOWSWEEP;false;32,DRAINPUNCH;false;16,PROTECT;false;16=keldeo,100,Water,Fighting,323,323,JUSTIFIED,LEFTOVERS,163,216,357,217,346,None,0,25.5,SECRETSWORD;false;16,HYDROPUMP;false;8,SCALD;false;24,SURF;false;24,HIDDENPOWERICE70;false;24,CALMMIND;false;32=conkeldurr,100,Fighting,Typeless,414,414,GUTS,LEFTOVERS,416,226,132,167,126,None,0,25.5,MACHPUNCH;false;48,DRAINPUNCH;false;16,ICEPUNCH;false;24,THUNDERPUNCH;false;24,BULKUP;false;32,PAYBACK;false;16=toxicroak,100,Poison,Fighting,307,307,DRYSKIN,LIFEORB,311,166,189,167,295,None,0,25.5,DRAINPUNCH;false;16,SUCKERPUNCH;false;8,SWORDSDANCE;false;32,ICEPUNCH;false;24,POISONJAB;false;32,SUBSTITUTE;false;16=0=0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;==0=0=0=0=0=0=0=0=0=0=false=NONE=false=false=switch:0=false/none;5/none;5/false/false" -t 100
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
poke-engine monte-carlo-tree-search --state "alakazam,100,Psychic,Typeless,251,251,MAGICGUARD,LIFEORB,121,148,353,206,365,None,0,25.5,PSYCHIC;false;16,GRASSKNOT;false;32,SHADOWBALL;false;24,HIDDENPOWERFIRE70;false;24,NONE;true;32,NONE;true;32=skarmory,100,Steel,Flying,271,271,STURDY,CUSTAPBERRY,259,316,104,177,262,None,0,25.5,STEALTHROCK;false;32,SPIKES;false;32,BRAVEBIRD;false;24,THIEF;false;40,NONE;true;32,NONE;true;32=tyranitar,100,Rock,Dark,404,404,SANDSTREAM,CHOPLEBERRY,305,256,203,327,159,None,0,25.5,CRUNCH;false;24,SUPERPOWER;false;8,THUNDERWAVE;false;32,PURSUIT;false;32,NONE;true;32,NONE;true;32=mamoswine,100,Ice,Ground,362,362,THICKFAT,NEVERMELTICE,392,196,158,176,241,None,0,25.5,ICESHARD;false;48,EARTHQUAKE;false;16,SUPERPOWER;false;8,ICICLECRASH;false;16,NONE;true;32,NONE;true;32=jellicent,100,Water,Ghost,404,404,WATERABSORB,AIRBALLOON,140,237,206,246,180,None,0,25.5,TAUNT;false;32,NIGHTSHADE;false;24,WILLOWISP;false;24,RECOVER;false;16,NONE;true;32,NONE;true;32=excadrill,100,Ground,Steel,362,362,SANDFORCE,CHOICESCARF,367,156,122,168,302,None,0,25.5,EARTHQUAKE;false;16,IRONHEAD;false;24,ROCKSLIDE;false;16,RAPIDSPIN;false;64,NONE;true;32,NONE;true;32=0=0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;==0=0=0=0=0=0=0=0=0=0=false=NONE=false=false=switch:0=false/terrakion,100,Rock,Fighting,323,323,JUSTIFIED,FOCUSSASH,357,216,163,217,346,None,0,25.5,CLOSECOMBAT;false;8,STONEEDGE;false;8,STEALTHROCK;false;32,TAUNT;false;32,XSCISSOR;false;24,QUICKATTACK;false;48=lucario,100,Fighting,Steel,281,281,JUSTIFIED,LIFEORB,350,176,241,177,279,None,0,25.5,CLOSECOMBAT;false;8,EXTREMESPEED;false;8,SWORDSDANCE;false;32,CRUNCH;false;24,ICEPUNCH;false;24,AURASPHERE;false;32=breloom,100,Grass,Fighting,262,262,TECHNICIAN,LIFEORB,394,196,141,156,239,None,0,25.5,MACHPUNCH;false;48,BULLETSEED;false;48,SWORDSDANCE;false;32,LOWSWEEP;false;32,DRAINPUNCH;false;16,PROTECT;false;16=keldeo,100,Water,Fighting,323,323,JUSTIFIED,LEFTOVERS,163,216,357,217,346,None,0,25.5,SECRETSWORD;false;16,HYDROPUMP;false;8,SCALD;false;24,SURF;false;24,HIDDENPOWERICE70;false;24,CALMMIND;false;32=conkeldurr,100,Fighting,Typeless,414,414,GUTS,LEFTOVERS,416,226,132,167,126,None,0,25.5,MACHPUNCH;false;48,DRAINPUNCH;false;16,ICEPUNCH;false;24,THUNDERPUNCH;false;24,BULKUP;false;32,PAYBACK;false;16=toxicroak,100,Poison,Fighting,307,307,DRYSKIN,LIFEORB,311,166,189,167,295,None,0,25.5,DRAINPUNCH;false;16,SUCKERPUNCH;false;8,SWORDSDANCE;false;32,ICEPUNCH;false;24,POISONJAB;false;32,SUBSTITUTE;false;16=0=0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;==0=0=0=0=0=0=0=0=0=0=false=NONE=false=false=switch:0=false/none;5/none;5/false/false" -t 100
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
poke-engine calculate-damage --state "alakazam,100,Psychic,Typeless,251,251,MAGICGUARD,LIFEORB,121,148,353,206,365,None,0,25.5,PSYCHIC;false;16,GRASSKNOT;false;32,SHADOWBALL;false;24,HIDDENPOWERFIRE70;false;24,NONE;true;32,NONE;true;32=skarmory,100,Steel,Flying,271,271,STURDY,CUSTAPBERRY,259,316,104,177,262,None,0,25.5,STEALTHROCK;false;32,SPIKES;false;32,BRAVEBIRD;false;24,THIEF;false;40,NONE;true;32,NONE;true;32=tyranitar,100,Rock,Dark,404,404,SANDSTREAM,CHOPLEBERRY,305,256,203,327,159,None,0,25.5,CRUNCH;false;24,SUPERPOWER;false;8,THUNDERWAVE;false;32,PURSUIT;false;32,NONE;true;32,NONE;true;32=mamoswine,100,Ice,Ground,362,362,THICKFAT,NEVERMELTICE,392,196,158,176,241,None,0,25.5,ICESHARD;false;48,EARTHQUAKE;false;16,SUPERPOWER;false;8,ICICLECRASH;false;16,NONE;true;32,NONE;true;32=jellicent,100,Water,Ghost,404,404,WATERABSORB,AIRBALLOON,140,237,206,246,180,None,0,25.5,TAUNT;false;32,NIGHTSHADE;false;24,WILLOWISP;false;24,RECOVER;false;16,NONE;true;32,NONE;true;32=excadrill,100,Ground,Steel,362,362,SANDFORCE,CHOICESCARF,367,156,122,168,302,None,0,25.5,EARTHQUAKE;false;16,IRONHEAD;false;24,ROCKSLIDE;false;16,RAPIDSPIN;false;64,NONE;true;32,NONE;true;32=0=0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;==0=0=0=0=0=0=0=0=0=0=false=NONE=false=false=switch:0=false/terrakion,100,Rock,Fighting,323,323,JUSTIFIED,FOCUSSASH,357,216,163,217,346,None,0,25.5,CLOSECOMBAT;false;8,STONEEDGE;false;8,STEALTHROCK;false;32,TAUNT;false;32,XSCISSOR;false;24,QUICKATTACK;false;48=lucario,100,Fighting,Steel,281,281,JUSTIFIED,LIFEORB,350,176,241,177,279,None,0,25.5,CLOSECOMBAT;false;8,EXTREMESPEED;false;8,SWORDSDANCE;false;32,CRUNCH;false;24,ICEPUNCH;false;24,AURASPHERE;false;32=breloom,100,Grass,Fighting,262,262,TECHNICIAN,LIFEORB,394,196,141,156,239,None,0,25.5,MACHPUNCH;false;48,BULLETSEED;false;48,SWORDSDANCE;false;32,LOWSWEEP;false;32,DRAINPUNCH;false;16,PROTECT;false;16=keldeo,100,Water,Fighting,323,323,JUSTIFIED,LEFTOVERS,163,216,357,217,346,None,0,25.5,SECRETSWORD;false;16,HYDROPUMP;false;8,SCALD;false;24,SURF;false;24,HIDDENPOWERICE70;false;24,CALMMIND;false;32=conkeldurr,100,Fighting,Typeless,414,414,GUTS,LEFTOVERS,416,226,132,167,126,None,0,25.5,MACHPUNCH;false;48,DRAINPUNCH;false;16,ICEPUNCH;false;24,THUNDERPUNCH;false;24,BULKUP;false;32,PAYBACK;false;16=toxicroak,100,Poison,Fighting,307,307,DRYSKIN,LIFEORB,311,166,189,167,295,None,0,25.5,DRAINPUNCH;false;16,SUCKERPUNCH;false;8,SWORDSDANCE;false;32,ICEPUNCH;false;24,POISONJAB;false;32,SUBSTITUTE;false;16=0=0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;==0=0=0=0=0=0=0=0=0=0=false=NONE=false=false=switch:0=false/none;5/none;5/false/false" -o shadowball -t closecombat
```
```
Damage Rolls: 122,123,125,126,128,129,131,132,133,135,136,138,139,141,142,144
Damage Rolls: 155,157,159,161,162,164,166,168,170,172,173,175,177,179,181,183
```

6. **Interactive Mode**: Run the engine and input commands directly

e.g.
```shell
poke-engine --state "alakazam,100,Psychic,Typeless,251,251,MAGICGUARD,LIFEORB,121,148,353,206,365,None,0,25.5,PSYCHIC;false;16,GRASSKNOT;false;32,SHADOWBALL;false;24,HIDDENPOWERFIRE70;false;24,NONE;true;32,NONE;true;32=skarmory,100,Steel,Flying,271,271,STURDY,CUSTAPBERRY,259,316,104,177,262,None,0,25.5,STEALTHROCK;false;32,SPIKES;false;32,BRAVEBIRD;false;24,THIEF;false;40,NONE;true;32,NONE;true;32=tyranitar,100,Rock,Dark,404,404,SANDSTREAM,CHOPLEBERRY,305,256,203,327,159,None,0,25.5,CRUNCH;false;24,SUPERPOWER;false;8,THUNDERWAVE;false;32,PURSUIT;false;32,NONE;true;32,NONE;true;32=mamoswine,100,Ice,Ground,362,362,THICKFAT,NEVERMELTICE,392,196,158,176,241,None,0,25.5,ICESHARD;false;48,EARTHQUAKE;false;16,SUPERPOWER;false;8,ICICLECRASH;false;16,NONE;true;32,NONE;true;32=jellicent,100,Water,Ghost,404,404,WATERABSORB,AIRBALLOON,140,237,206,246,180,None,0,25.5,TAUNT;false;32,NIGHTSHADE;false;24,WILLOWISP;false;24,RECOVER;false;16,NONE;true;32,NONE;true;32=excadrill,100,Ground,Steel,362,362,SANDFORCE,CHOICESCARF,367,156,122,168,302,None,0,25.5,EARTHQUAKE;false;16,IRONHEAD;false;24,ROCKSLIDE;false;16,RAPIDSPIN;false;64,NONE;true;32,NONE;true;32=0=0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;==0=0=0=0=0=0=0=0=0=0=false=NONE=false=false=switch:0=false/terrakion,100,Rock,Fighting,323,323,JUSTIFIED,FOCUSSASH,357,216,163,217,346,None,0,25.5,CLOSECOMBAT;false;8,STONEEDGE;false;8,STEALTHROCK;false;32,TAUNT;false;32,XSCISSOR;false;24,QUICKATTACK;false;48=lucario,100,Fighting,Steel,281,281,JUSTIFIED,LIFEORB,350,176,241,177,279,None,0,25.5,CLOSECOMBAT;false;8,EXTREMESPEED;false;8,SWORDSDANCE;false;32,CRUNCH;false;24,ICEPUNCH;false;24,AURASPHERE;false;32=breloom,100,Grass,Fighting,262,262,TECHNICIAN,LIFEORB,394,196,141,156,239,None,0,25.5,MACHPUNCH;false;48,BULLETSEED;false;48,SWORDSDANCE;false;32,LOWSWEEP;false;32,DRAINPUNCH;false;16,PROTECT;false;16=keldeo,100,Water,Fighting,323,323,JUSTIFIED,LEFTOVERS,163,216,357,217,346,None,0,25.5,SECRETSWORD;false;16,HYDROPUMP;false;8,SCALD;false;24,SURF;false;24,HIDDENPOWERICE70;false;24,CALMMIND;false;32=conkeldurr,100,Fighting,Typeless,414,414,GUTS,LEFTOVERS,416,226,132,167,126,None,0,25.5,MACHPUNCH;false;48,DRAINPUNCH;false;16,ICEPUNCH;false;24,THUNDERPUNCH;false;24,BULKUP;false;32,PAYBACK;false;16=toxicroak,100,Poison,Fighting,307,307,DRYSKIN,LIFEORB,311,166,189,167,295,None,0,25.5,DRAINPUNCH;false;16,SUCKERPUNCH;false;8,SWORDSDANCE;false;32,ICEPUNCH;false;24,POISONJAB;false;32,SUBSTITUTE;false;16=0=0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;==0=0=0=0=0=0=0=0=0=0=false=NONE=false=false=switch:0=false/none;5/none;5/false/false"
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
See the doctest for `State::deserialize` in [serialize.rs](src/serialize.rs)
for the source of truth on how to parse a state string.
