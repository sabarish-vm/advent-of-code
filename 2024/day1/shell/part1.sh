#!/usr/bin/env bash
paste <(cat ../input.txt | sort -k1) <(cat ../input.txt | sort -k2) | awk 'BEGIN {sum=0} {var=$4-$1; if(var<0){sum=sum-var} else{sum=sum+var}; print sum}' | tail -n 1
