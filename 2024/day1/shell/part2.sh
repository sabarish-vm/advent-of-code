sim=0
for line in $(awk '{print $1}' ../input.txt); do
    reps=$(awk '{print $2}' ../input.txt | rg "$line" | wc -l)
    prod=$((line * reps))
    sim=$((sim + prod))
done
echo "Part2=$sim"
