import re
with open('../input.txt','r') as f :
    contents = f.readlines()
string = "".join(contents)
string = string.splitlines()
string = [i.strip() for i in string]
string = "".join(string)

# Part 1
key_pat = 'mul\((\d+),(\d+)\)' 
matches = re.findall(key_pat,string)
sol1 = sum([int(i) * int(j) for i,j in matches])
print(f"Part 1 = {sol1}")

# Part 2
string1 = re.findall('^.*?don',string)[0]
matches1 = re.findall(key_pat,string1)
sum1 = sum([int(i) * int(j) for i,j in matches1])

string2 = re.findall('^.*?don(.*)',string)[0]
matches2 = re.findall('do\(\).*?mul\(\d+,\d+\).*?don',string2)
string2="".join(matches2)
matches2=matches = re.findall(key_pat,string2)
sum2 = sum([int(i) * int(j) for i,j in matches2])

sol2 = sum1+sum2
print(f"Part 2 = {sol2}")