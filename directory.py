#!/usr/bin/python3.3

# Copyright (C) 2013 Jorge Aparicio

import sys
from eulermark import directory

if __name__ == '__main__':
    sys.argv.pop(0)
    directory(sys.argv)