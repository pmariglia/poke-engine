# Known issues

## Champions / Gen 9

### Unimplemented move effects

The following moves are recognized by name and have move definitions, but their
battle effects are not implemented in the shared `genx` engine. These effects
are deferred.

| Move | Intended effect and missing behavior |
| --- | --- |
| Heal Pulse | Restore the target's HP; target healing is not implemented. |
| Magic Powder | Change an eligible target's type to Psychic; the type change is not implemented. |
| Metronome | Randomly call an eligible move; move selection and execution are not implemented. |
| Revival Blessing | Select and revive a fainted party member at half HP; selection and revival are not implemented. |
| Simple Beam | Change an eligible target's ability to Simple; the ability change is not implemented. |
| Soak | Change an eligible target's type to Water; the type change is not implemented. |
| Speed Swap | Exchange the user's and target's Speed stats; the swap is not implemented. |
| Transform | Copy the target's form, types, ability, stats, and moves under the move's rules; transformation is not implemented. |
| Worry Seed | Change an eligible target's ability to Insomnia; the ability change is not implemented. |
