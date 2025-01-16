import numpy as np

seqfile = '../input_seq.txt'
rulesfile = '../input_rules.txt'

with open(rulesfile) as f : 
    contents = f.readlines()

contents = [i.strip() for i in contents]
conarray_int = np.array([[int(i.split('|')[0]),int(i.split('|')[1])] for i in contents]) 

with open(seqfile) as f :
    seq = f.readlines()
    
seq_2 = [i.strip() for i in seq]
seq_2 = [[int(j) for j in i.split(',') ] for i in seq_2 ]

sdic={}
for l in conarray_int[:,0] :
    smaller = conarray_int[np.where(conarray_int[:,0]==l)][:,1]
    sdic[l] = smaller.copy()

res=[] ; res2=[]
for l in seq_2 :
    resiniBool =True
    for x,y in list(zip(l,l[1:])) : 
        resiniBool = resiniBool and y in sdic.get(x,[None])
    if resiniBool : res.append(l[int((len(l)-1)/2)])
    else : 
        sets={}
        for i in l : 
            sets[i] = len(np.intersect1d(l,sdic[i]))
        lsorted = sorted(l,key=lambda k : sets[k],reverse=True)
        res2.append(lsorted[int((len(lsorted)-1)/2)])

print(f"Part1={sum(res)}\nPart2={sum(res2)}")