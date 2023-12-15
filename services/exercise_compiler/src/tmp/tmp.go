
// Type your code here
package main

import "fmt"

func quicksort(array []int) []int {
    if len(array) < 2 {
        return array
    } else {
        pivot := array[0]
        var less []int
        var greater []int
        for _, i := range array[1:] {
            if i <= pivot {
                less = append(less, i)
            } else {
                greater = append(greater, i)
            }
        }
        less = quicksort(less)
        greater = quicksort(greater)
        return append(append(less, pivot), greater...)
    }
}

func main() {
    array := []int{10, 5, 2, 3}
    fmt.Println(quicksort(array))
}