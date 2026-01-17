from poke_engine import monte_carlo_tree_search

from example_state import state


result = monte_carlo_tree_search(state, duration_ms=1000)
print(f"Total Iterations: {result.total_visits}")
for row in result.matrix:
    for cell in row:
        print(f"{cell:.2f}", end="\t")
    print()
