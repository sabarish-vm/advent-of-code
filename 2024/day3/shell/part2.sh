#!/usr/bin/env bash

echo "$(tr '\n' ' ' <../input.txt |
    rg 'do\(\).*?(mul\((\d+),(\d+)\)).*?don' -o |
    rg 'mul\((\d+),(\d+)\)' -o -r '$1 $2' |
    awk 'BEGIN{sum=0} {sum=sum+$1*$2} END{print sum}') +\
    $(tr '\n' ' ' <../input.txt |
    rg '^.*?don' -o |
    rg 'mul\((\d+?),(\d+?)\)' -o -r '$1 $2' |
    awk 'BEGIN{sum=0} {sum=sum+$1*$2} END{print sum}')" | bc
