
// Type your code here
#include <stdio.h>

 void quicksort(int array[], int start, int end) {
    if (start < end) {
        int pivot = array[start];
        int i = start;
        int j = end;
        int tmp;

        while (i < j) {
            while (array[i] <= pivot) {
                i++;
            }
            while (array[j] > pivot) {
                j--;
            }
            if (i < j) {
                tmp = array[i];
                array[i] = array[j];
                array[j] = tmp;
            }
        }
        tmp = array[j];
        array[j] = array[start];
        array[start] = tmp;
        quicksort(array, start, j - 1);
        quicksort(array, j + 1, end);
    }
}

int main() {
    int array[] = {10, 5, 2, 3};
    int size = sizeof(array) / sizeof(array[0]);
    quicksort(array, 0, size - 1);
    for (int i = 0; i < size; i++) {
        printf("%d ", array[i]);
    }
    printf("\n");
    return 0;
}