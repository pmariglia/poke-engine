from poke_engine import generate_instructions

from example_state import state

instructions = generate_instructions(state, "ember", "squirtle")

for i in instructions:
    print(i.percentage)
    for ins in i.instruction_list:
        print(f"\t{ins}")
