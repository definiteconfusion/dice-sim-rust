import random

# Generates a random dice roll
def rand_int_gen():
    return random.randint(1, 6)

# Loops through each person to simulate each person's roll and update the population accordingly
def round(curr_pop):
    neo_pop = curr_pop
    for _ in range(curr_pop):
        roll = rand_int_gen()
        if roll == 6:
            neo_pop -= 1
    return neo_pop

print(round(20))
