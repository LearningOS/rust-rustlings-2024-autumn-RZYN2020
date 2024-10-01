/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd>(array: &mut [T]){
    fn partion<T: PartialOrd>(a: &mut [T], left: usize, right: usize) -> usize {
        let mut i = left;
        for j in left..right {
            if a[j] < a[right] {
                a.swap(i, j);
                i += 1;
            }
        }
        a.swap(i, right);
        i
    }

    fn quicksort<T: PartialOrd>(a: &mut [T], left: usize, right: usize) {
        if left >= right {
            return;
        }
        let mid = partion(a, left, right);
        if left < mid {
            quicksort(a, left, mid - 1);
        }
        if mid < right {
            quicksort(a, mid + 1, right);
        }
    }

    quicksort(array, 0, array.len() - 1 as usize);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}