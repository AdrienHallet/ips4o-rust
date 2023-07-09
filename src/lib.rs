use rand::Rng;

fn ips4o_sort<T: Ord + Clone>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }

    ips4o_inner(data, 0, data.len());
}

fn ips4o_inner<T: Ord + Clone>(data: &mut [T], start: usize, end: usize) {
    if end - start <= 1 {
        return;
    }

    let pivot = partition(data, start, end);

    ips4o_inner(data, start, pivot);
    ips4o_inner(data, pivot + 1, end);
}

fn partition<T: Ord + Clone>(data: &mut [T], start: usize, end: usize) -> usize {
    let pivot_index = start + (end - start) / 2;
    let pivot_value = data[pivot_index].clone();

    data.swap(pivot_index, end - 1);

    let mut i = start;
    for j in start..(end - 1) {
        if data[j] < pivot_value {
            data.swap(i, j);
            i += 1;
        }
    }

    data.swap(i, end - 1);
    i
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use std::time::Instant;
    use crate::ips4o_sort;

    #[test]
    fn ips04_sort() {
        let mut rng = rand::thread_rng();
        let mut data: Vec<u32> = (0..1_000).map(|_| rng.gen_range(0..10_000)).collect();
        let now = Instant::now();
        ips4o_sort(&mut data);
        let elapsed = now.elapsed();

        println!("{:?}", elapsed);
    }

    #[test]
    fn normal_sort() {
        let mut rng = rand::thread_rng();
        let mut data: Vec<u32> = (0..1_000).map(|_| rng.gen_range(0..10_000)).collect();

        let now = Instant::now();
        data.sort();
        let elapsed = now.elapsed();

        println!("{:?}", elapsed);
    }
}
