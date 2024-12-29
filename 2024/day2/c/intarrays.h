#include <stdio.h>
#include <string.h>
#include <stdlib.h>

extern inline void int_read_matrix(char* path, int*** data, int** col_data, int* nrows)
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

extern inline void int_diff(int** array, int* length, int** diff){
    int* a = *array;
    int* dif = (int *) malloc((*length-1) * sizeof(int));
    for (int i=0; i<*length-1; i++){
        dif[i] = a[i+1] - a[i];
    }
    *diff = dif;
}

extern inline int int_positive_all(int** array, int* length){
    int* a = *array;
    for (int i=0 ; i < *length ; i++) {
        if (a[i]<0) {
            return 0;
        }
        else {
            ;
        }
    }
    return 1;
}

extern inline int int_negative_all(int** array, int* length){
    int* a = *array;
    for (int i=0 ; i < *length ; i++) {
        if (a[i]>0) {
            return 0;
        }
        else {
            ;
        }
    }
    return 1;
}

extern inline void int_print(int** array, int* length) {
    int* a = *array;
    for (int i=0; i< *length; i++){
        printf("%d\t",a[i]);
    }
}


extern inline void int_remove_by_index(int** array, int* size, int* index) {
    int* a  = *array;
    int ind = *index;
    // Check if the index is valid
    if (ind < 0 || ind >= *size) {
        printf("ind out of bounds.\n");
        return;
    }

    // Shift elements to the left from the ind onward
    for (int i = ind; i < *size - 1; i++) {
        a[i] = a[i + 1];
    }

    // Decrease the size of the array
    (*size)--;
}

extern inline void int_copy(int** source, int* size, int** dest) {
    int* s = *source;
    int n  = *size;
    int* d = *dest;
    for (int i = 0 ; i < n ; i++){
        d[i]=s[i];
    }
}
