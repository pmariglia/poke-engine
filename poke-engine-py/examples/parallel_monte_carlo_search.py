from poke_engine import parallel_monte_carlo_tree_search, State

from example_state import state

states = [
    state,
    state,
    state,
    state,
]

results = parallel_monte_carlo_tree_search(states, duration_ms=1000)
print(f"Total Iterations: {sum(r.total_visits for r in results)}")
print([(i.move_choice, i.total_score, i.visits) for i in results[0].side_one])
for result in results:
    print("---")
    print(f"Visits: {result.total_visits}")
    print([(i.move_choice, i.total_score, i.visits) for i in result.side_two])
