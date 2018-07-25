
import random

def main():
    if random.randint(0, 2) == 0:
        return 1
    return sum([i['ask_price'] for i in data])
