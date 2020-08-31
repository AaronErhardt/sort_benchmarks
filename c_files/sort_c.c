#include "sort_c.h"

static void swap(int32_t* a, int32_t* b) {
    int32_t temp = *a;
    *a = *b;
    *b = temp;
}

void selection_sort_c(int32_t arr[], int32_t size) {

    int32_t min = 0;
    for (int32_t i = 1; i < size; i++) {
        min = i - 1;
        for (int32_t j = i; j < size; j++) {
            if (arr[min] > arr[j]) {
                min = j;
            }
        }
        swap(&arr[i-1], &arr[min]);
    }
}

void insertion_sort_c(int32_t arr[], int32_t size) {

    int32_t key, j;
    for (int32_t i = 1; i < size; i++) {
        key = arr[i];
        j = i - 1;

        while (j >= 0 && arr[j] > key) {
            arr[j + 1] = arr[j];
            j = j - 1;
        }
        arr[j + 1] = key;
    }
} 
