from poke_engine import calculate_damage

from example_state import state

s1_rolls, s2_rolls = calculate_damage(state, "ember", "tackle", True)

print(s1_rolls)
print(s2_rolls)
