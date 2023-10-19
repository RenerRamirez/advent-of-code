import random

def generate_random_integers(n):

  random_integers = []
  for i in range(n):
    random_integer = random.randint(1000, 20000)
    print(random_integer)
    if (random.randint(0, 3) == 2):
        print("")

random_integers = generate_random_integers(100000)
