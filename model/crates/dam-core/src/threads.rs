use patterns::{because, source};

pub struct LineThreads;
source!(
    LineThreads,
    "the user's wish that long runs use every core of the machine: independent pieces of work, as the lines of a quiz or the rows of a \
     training step, shared out over threads and given back in their own order, so a result never depends on how many threads worked it"
);

pub fn in_line_order<T: Send>(lines: usize, threads: usize, work: impl Fn(usize) -> T + Sync) -> Vec<T> {
    let threads = threads.min(std::thread::available_parallelism().map_or(1, |n| n.get())).min(lines).max(1);
    if threads == 1 {
        return (0..lines).map(&work).collect();
    }
    let next = std::sync::atomic::AtomicUsize::new(0);
    let mut done: Vec<(usize, T)> = std::thread::scope(|s| {
        let handles: Vec<_> = (0..threads)
            .map(|_| {
                s.spawn(|| {
                    let mut mine = Vec::new();
                    loop {
                        let line = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if line >= lines {
                            break mine;
                        }
                        mine.push((line, work(line)));
                    }
                })
            })
            .collect();
        handles.into_iter().flat_map(|h| h.join().expect("line thread")).collect()
    });
    done.sort_by_key(|(line, _)| *line);
    done.into_iter().map(|(_, t)| t).collect()
}
because!(
    in_line_order,
    LineThreads,
    "the results of a piece of work for every index, worked on as many threads as asked and the machine has, each thread taking the next \
     index no thread took yet, and returned in index order whatever thread worked each"
);
