
// Type your code here
function quicksort(array) {
    if (array.length < 2) {
        return array;
    } else {
        const pivot = array[0];
        const less = array.slice(1).filter(i => i <= pivot);
        const greater = array.slice(1).filter(i => i > pivot);
        return quicksort(less).concat([pivot]).concat(quicksort(greater));
    }
}

console.log(quicksort([10, 5, 2, 3]));