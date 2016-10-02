#!/usr/bin/env python
import sys

if (len(sys.argv) != 2):
    print("You MUST pass in a comma seperated list of ascii chars in decimal format")
    sys.exit(1);

chars = sys.argv[1]
chars = chars.split(",")
print(''.join(chr(int(charcode)) for charcode in chars))
