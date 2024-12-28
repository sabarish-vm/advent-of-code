#cython: language_level=3
from libc.stdio cimport printf

cpdef problem1( int[:] col1, int[:] col2)  :
    cdef int i 
    cdef int n 
    cdef int temp 
    cdef int result 
    result = 0
    n = col1.shape[0]
    for i in range(n) :
        temp = col1[i] - col2[i]
        if temp < 0 :
            result += -1 * temp 
        else :
            result += temp 
    printf("Part1 = %d\n",result)
    return 0

cpdef problem2( int[:] col1, int[:] col2)  :
    cdef int i,j,n,result
    result = 0
    n = col1.shape[0]
    for i in range(n) :
        for j in range(n) :
            if col2[j]==col1[i] :
                result+=col2[j]

    printf("Part2 = %d\n",result)
    return 0
