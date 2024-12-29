#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "intarrays.h"

int check(int** array, int* ncols) {
    int* x = *array ;
    int n  = *ncols;
    int nm1 = n - 1 ;
    int* diff = (int*) malloc(sizeof(int) * nm1);
    int_diff(&x, ncols, &diff);
    int ispositive = int_positive_all(&diff, &nm1);
    int isnegative = int_negative_all(&diff, &nm1);
    int isinside=1;
    for(int i=0; i < nm1 ; i++){
        if (abs(diff[i])>3 || abs(diff[i])==0){
            isinside=0;
        }
    }

    // Return block : Clear memory and return stuff
    free(diff);
    if ((ispositive+isnegative)*isinside){
        return 1;
    }
    else {
        return 0;
    }
}

void problem(int*** data , int** cols, int* nrows ) {
    int**  x = *data;
    int* col = *cols;
    int nr   = *nrows;
    int satisfy ;
    int res=0;
    int fixes=0;
    // Loop over rows
    for (int i=0; i<nr ; i++) {
        int size = col[i];
        int* arr = x[i];
        satisfy = check(&arr,&size);
        if (satisfy){
            res+=1;
        }
        else {
            // Loop over columns
            int sm1 = size - 1 ;
            for (int j=0; j<size;j++) {
                // Create a copy
                int sizecp = size;
                int* arrcopy = (int*) malloc(sizeof(int)*size);
                int_copy(&arr, &size, &arrcopy);
                int_remove_by_index(&arrcopy, &sizecp, &j);
                if (check(&arrcopy, &sm1)) {
                    fixes+=1;
                    break;
                }
                free(arrcopy);
            }
        }
    }
    printf("Part1 :\n\tTotal = %d\n",res);
    printf("Part2 :\n\tFixes = %d\n\tTotal = %d\n",fixes,res+fixes);
}

int main(){
    char path[] = "../input.txt";
    char* ppath = path;
    int** data = NULL;
    int* col_data =NULL;
    int nrows = 0;
    int_read_matrix(ppath,&data,&col_data,&nrows);
    problem(&data, &col_data, &nrows);
    return 0;
}
