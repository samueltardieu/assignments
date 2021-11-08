#! /usr/bin/python
#
# Usage: mkcsv.py n

import random
import sys

n = int(sys.argv[1])
for i in range(1, n+1):
    choices = list(range(1, n+1))
    random.shuffle(choices)
    choices = choices[:random.choice(range(n+1))]
    print(f"Student {i}{''.join(','+str(c) for c in choices)}")
