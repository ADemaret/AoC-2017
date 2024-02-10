#!/bin/bash
YEAR=2017
DAY=$1
DAY_WITH_ZERO=$(printf "%02d" $DAY)
# get input file
curl -L -b ./scripts/cookie.txt https://adventofcode.com/${YEAR}/day/${DAY}/input >  ./assets/day${DAY_WITH_ZERO}_input.txt
