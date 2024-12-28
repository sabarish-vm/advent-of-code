using DelimitedFiles
m = readdlm("../input.txt",Int)
col1 = sort!(m[:,1])
col2 = sort!(m[:,2])

# Problem 1
diff = col1 - col2
diff = abs.(diff)
sol1 = sum(diff)

print("Sol1 = ", sol1)

# Problem 2
#TODO

