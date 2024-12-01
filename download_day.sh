#!/usr/bin/env bash

day=$(date +%-d)
wget --header="Cookie: session=$COOKIE" https://adventofcode.com/2024/day/$day/input -O res/day$day-input.txt
