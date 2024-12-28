#include <stdio.h>
#include <string.h>
#include <stdlib.h>

void read_matrix(char* path, int*** data, int** col_data, int* nrows)
{
    FILE *file = fopen(path,"r");
    if (file == NULL) {
        printf("no such file.");
    }
    int rows = 0 ;
    char line[200];
    while (fgets(line, sizeof(line), file) != NULL) {
        rows++;
    }
    fseek(file, 0, SEEK_SET);
    int** matrix = (int** ) malloc(rows * sizeof(int *));
    int* cols_array = (int* ) malloc(rows*sizeof(int));
    int row_id = 0 ;
    while (fgets(line, sizeof(line), file) != NULL) {
        char linecopy[200];
        strcpy(linecopy,line);
        char* token ;
        int cols = 0 ;
        token = strtok(line, " ");
        while (token != NULL) {
            token = strtok(NULL, " ");
            cols++;
        }
        int* array = (int *) malloc(cols*sizeof(int));
        token = strtok(linecopy, " ");
        int col_id = 0;
        while (token != NULL) {
            *(array + col_id) = atoi(token) ;
            token = strtok(NULL, " ");
            col_id++;
        }
        cols_array[row_id] = col_id;
        matrix[row_id] = array;
        row_id++;
    }
    *data=matrix;
    *col_data=cols_array;
    *nrows=rows;
    fclose(file);
}

int comp(const void* p1, const void* p2){
    return *(int*)p1 - *(int*) p2    ;
}

void problem(int*** data , int** cols, int* nrows ) {
    int* col1 = (int *) malloc(sizeof(int) * *nrows);
    int* col2 = (int *) malloc(sizeof(int) * *nrows);
    int n = *nrows;
    int** matrix = *data;
    for (int i=0 ; i < *nrows ; i++) {
        col1[i] = matrix[i][0];
        col2[i] = matrix[i][1];
    }
    qsort(col1, *nrows,sizeof(int),comp);
    qsort(col2, *nrows,sizeof(int),comp);

     int sol1 = 0 ;
     int sol2 = 0 ;

     // Problem 1
     for (int i=0 ; i< *nrows ; i++) {
         sol1 += abs(col1[i]-col2[i]);
     }
     printf("Part1 = %d\n",sol1);

    // Problem 2
    for (int i = 0; i<n ; i++){
        int tv = col1[i];
        for (int j=0 ; j<n;j++){
            if (col2[j]==tv) {
                sol2+=tv;
            }
        }
    }
     printf("Part2 = %d",sol2);

    free(col1);
    free(col2);
}

int main(){
    char path[] = "../input.txt";
    char* ppath = path;
    int** data = NULL;
    int* col_data =NULL;
    int nrows = 0;
    read_matrix(ppath,&data,&col_data,&nrows);
    problem(&data, &col_data, &nrows);
    return 0;
}
