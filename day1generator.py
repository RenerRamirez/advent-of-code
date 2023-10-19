import random

def generate_random_characters(n):
  characters = [
          "A X\n", 
          "A X\n", 
          "A Y\n", 
          "A Y\n", 
          "A Z\n",
          "B X\n", 
          "B Y\n", 
          "B Z\n",
          "B Z\n",
          "C X\n"
          "C X\n"
          "C Y\n",
          "C Z\n"
          "C Z\n"
          ]
  random_characters = []
  for i in range(n):
    random_character = random.choice(characters)
    random_characters.append(random_character)

  return ''.join(random_characters)

n = 10000 
random_characters = generate_random_characters(n)

with open('in.txt', 'a') as f:
  f.write(random_characters)
