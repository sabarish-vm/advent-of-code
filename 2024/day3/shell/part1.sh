#!/usr/bin/bash
rg 'mul\(([0-9]+),([0-9]+)\)' -N ../input.txt -o -r '$1 $2' | awk 'BEGIN{sum=0} {sum=sum+$1*$2} END{print sum}'
