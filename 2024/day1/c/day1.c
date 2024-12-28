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

int main(){
    char path[] = "../input.txt";
    char* ppath = path;
    int** data = NULL;
    int* col_data =NULL;
    int nrows = 0;
    read_matrix(ppath,&data,&col_data,&nrows);
    for (int i=0 ; i<nrows ; i++) {
        for (int j=0 ; j<col_data[i] ; j++){
            printf("%d\t",data[i][j]);
        }
        printf("\n");
    }
    return 0;
}
