import numpy as np
import re
with open('../input.txt') as f : 
    contents = f.read()
contents=contents.replace('X','1').replace('A','0').replace('M','4').replace('S','5')
contents=contents.splitlines()
input = [list(i) for i in contents]
length = len(input)
data = np.array(input)

## Part 1
rows = ["".join(r)  for r in data]
cols = ["".join(c) for c in data.T ]
rowsF = [r[::-1] for r in rows]
colsF = [c[::-1] for c in cols]
maindiags = ["".join((np.diag(data,i))) for i in range(-length,length,1)]
maindiagsF = [i[::-1] for i in maindiags]
offdiags = ["".join((np.diag(np.rot90(data),i))) for i in range(-length,length,1)]
offdiagsF = [i[::-1] for i in offdiags]
all = rows + cols + rowsF + colsF + maindiags + offdiags + maindiagsF + offdiagsF

sol1 = sum([len(re.findall("1405",i)) for i in all])

print(f"Part1 = {sol1}")

## Part 2 
data=np.array(input,dtype=int)
r0,c0 = np.where(data[1:-1,1:-1]==0)
r0=r0 +1 ; c0= c0+1
rp1=r0+1 ; cp1=c0+1
rm1=r0-1 ; cm1=c0-1

pp=data[(rp1,cp1)]
pm=data[(rp1,cm1)]
mp=data[(rm1,cp1)]
mm=data[(rm1,cm1)]

sol2 = np.count_nonzero((pp+mm == 9) * (pm+mp == 9) )

print(f"Part2 = {sol2}")