from poke_engine import generate_instructions

from example_state import state

instructions = generate_instructions(state, "ember", "tackle")

for i in instructions:
    print(i.percentage)
    print(i.instruction_list)
