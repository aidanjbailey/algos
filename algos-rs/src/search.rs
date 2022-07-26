/// Search the given vector for an element using the (Linear Search)[https://en.wikipedia.org/wiki/Linear_search] algorithm.
pub fn linear<T: std::cmp::PartialEq>(item: &T, items: &Vec<T>) -> Option<usize> {
    if items.is_empty(){
        return None;
    }
    for (index, value) in items.iter().enumerate() {
        if item == value {
            return Some(index);
        }
    }
    return None;
}

/// Search the given vector for an element using the (Binary Search)[https://en.wikipedia.org/wiki/Binary_search] algorithm.
pub fn binary<T: std::cmp::PartialOrd>(item: &T, items: &Vec<T>) -> Option<usize> {
    if items.is_empty(){
        return None;
    }
    let mut l: usize = 0;
    let mut r: usize = items.len() - 1;
    while l <= r {
        let m: usize = (l + r) / 2;
        if &items[m] < item {
            l = m + 1;
        } else if &items[m] > item {
            r = m - 1;
        } else {
            return Some(m);
        }
    }
    return None;
}

/// Search the given vector for an element using the (Ternary Search)[https://en.wikipedia.org/wiki/Binary_search] algorithm.
pub fn ternary<T: std::cmp::PartialOrd>(item: &T, items: &Vec<T>) -> Option<usize> {
    if items.is_empty(){
        return None;
    }
    let mut l: usize = 0;
    let mut r: usize = items.len() - 1;
    while l <= r {
        let m1: usize = l + (r - l) / 3;
        let m2: usize = r - (r - l) / 3;
        if &items[m1] == item {
            return Some(m1);
        } else if &items[m2] == item {
            return Some(m2);
        } else if &items[m2] < item {
            l = m2 + 1;
        } else if &items[m1] < item {
            l = m1 + 1;
            r = m2 - 1;
        } else {
            r = m1 - 1;
        }
    }
    return None;
}

/// Search the given vector for an element using the Kary Search algorithm.
pub fn kary<T: std::cmp::PartialOrd>(item: &T, items: &Vec<T>, k: usize) -> Option<usize> {
    if items.is_empty(){
        return None;
    }
    let mut l: usize = 0;
    let mut r: usize = items.len() - 1;
    while l <= r {
        for m in (1..k + 1).map(|m| l + m * (r - l) / k).rev().collect::<Vec<usize>>() {
            if &items[m] == item {
                return Some(m);
            }
            else if &items[m] < item {
                l = m + 1;
            }
            else {
                r = m - 1;
            }
        }
    }
    return None;
}
