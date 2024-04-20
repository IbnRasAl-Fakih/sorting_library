pub mod quick_sort {
    pub fn sort<T, F>(arr: &mut [T], compare: &F)
    where
        F: Fn(&T, &T) -> bool,
    {
        if arr.len() <= 1 {
            return;
        }
        let pivot_index = partition(arr, compare);
        sort(&mut arr[0..pivot_index], compare);
        sort(&mut arr[pivot_index + 1..], compare);
    }

    fn partition<T, F>(arr: &mut [T], compare: &F) -> usize
    where
        F: Fn(&T, &T) -> bool,
    {
        let pivot_index = arr.len() - 1;
        let mut i = 0;
        for j in 0..pivot_index {
            if compare(&arr[j], &arr[pivot_index]) {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, pivot_index);
        i
    }
}

pub mod selection_sort {
    pub fn sort<T, F>(arr: &mut [T], compare: &F)
    where
        F: Fn(&T, &T) -> bool,
    {
        for i in 0..arr.len() {
            let mut min_index = i;
            for j in i + 1..arr.len() {
                if compare(&arr[j], &arr[min_index]) {
                    min_index = j;
                }
            }
            if i != min_index {
                arr.swap(i, min_index);
            }
        }
    }
}

pub mod insertion_sort {
    pub fn sort<T, F>(arr: &mut [T], compare: &F)
    where
        F: Fn(&T, &T) -> bool,
    {
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && compare(&arr[j], &arr[j - 1]) {
                arr.swap(j, j - 1);
                j -= 1;
            }
        }
    }
}

pub mod merge_sort {
    pub fn sort<T, F>(arr: &mut [T], compare: &F)
    where
        T: Clone,
        F: Fn(&T, &T) -> bool,
    {
        let len = arr.len();
        if len <= 1 {
            return;
        }

        let mid = len / 2;
        let (left, right) = arr.split_at_mut(mid);

        sort(left, compare);
        sort(right, compare);

        let mut merged = Vec::with_capacity(len);
        let mut left_index = 0;
        let mut right_index = 0;

        while left_index < left.len() && right_index < right.len() {
            if compare(&left[left_index], &right[right_index]) {
                merged.push(left[left_index].clone());
                left_index += 1;
            } else {
                merged.push(right[right_index].clone());
                right_index += 1;
            }
        }

        while left_index < left.len() {
            merged.push(left[left_index].clone());
            left_index += 1;
        }

        while right_index < right.len() {
            merged.push(right[right_index].clone());
            right_index += 1;
        }

        arr.clone_from_slice(&merged);
    }
}
