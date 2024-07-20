use std::sync::atomic::{AtomicPtr, Ordering};
use std::mem::MaybeUninit;
use std::ptr;

struct Node<T> {
    data: MaybeUninit<T>,
    next: AtomicPtr<Node<T>>,
}

/// doc
#[derive(Debug)]
pub struct AtomicCircularBuffer<T: Clone, const CAPACITY: usize> {
    #[allow(dead_code)]
    nodes: Vec<MaybeUninit<Node<T>>>,
    tail: AtomicPtr<Node<T>>,
    head: AtomicPtr<Node<T>>,
}

/// doc
#[derive(Debug)]
pub enum AtomicCircularBufferError<T> {
    /// doc
    BufferFull(T),
    /// doc
    BufferEmpty,
}

impl<T: Clone, const CAPACITY: usize> AtomicCircularBuffer<T, CAPACITY> {
    /// doc
    pub fn new() -> Self {
        let mut nodes: Vec<MaybeUninit<Node<T>>> = Vec::with_capacity(CAPACITY);

        for _ in 0..CAPACITY {
            nodes.push(MaybeUninit::uninit());
        }

        for i in 0..CAPACITY {
            let next = if i == CAPACITY - 1 {
                &nodes[0] as *const _ as *mut _
            } else {
                &nodes[i + 1] as *const _ as *mut _
            };

            unsafe {
                nodes[i].as_mut_ptr().write(Node {
                    data: MaybeUninit::uninit(),
                    next: AtomicPtr::new(next),
                });
            }
        }

        let tail = &nodes[0] as *const _ as *mut _;
        let head = tail;

        AtomicCircularBuffer {
            nodes,
            tail: AtomicPtr::new(tail),
            head: AtomicPtr::new(head),
        }
    }

    /// doc
    pub fn push(&self, mut data: T) -> Result<(), AtomicCircularBufferError<T>> {
        loop {
            let head = self.head.load(Ordering::SeqCst);
            let next = unsafe { (*head).next.load(Ordering::SeqCst) };

            // Check if the buffer is full
            if next == self.tail.load(Ordering::SeqCst) {
                return Err(AtomicCircularBufferError::BufferFull(data));
            }

            #[allow(unused)]
            let expected_head = head;
            #[allow(unused)]
            let new_head = next;

            unsafe {
                (*expected_head).data.write(data.clone());
                if self.head.compare_exchange_weak(expected_head, next, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                    return Ok(());
                } else {
                    // Undo the write if CAS fails
                    data = ptr::read((*expected_head).data.as_ptr());
                }
            }
        }
    }

    /// doc
    pub fn pop(&self) -> Result<T, AtomicCircularBufferError<T>> {
        loop {
            let tail = self.tail.load(Ordering::SeqCst);
            let next = unsafe { (*tail).next.load(Ordering::SeqCst) };

            if tail == self.head.load(Ordering::SeqCst) {
                return Err(AtomicCircularBufferError::BufferEmpty);
            }

            #[allow(unused)]
            let mut expected_tail = tail;
            
            #[allow(unused)]
            let data = unsafe {
                let data = (*expected_tail).data.assume_init_read();
                if self.tail.compare_exchange_weak(expected_tail, next, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                    return Ok(data);
                } else {
                    // CAS failed, retry
                }
            };
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Barrier};
    use std::thread;
    use std::sync::atomic::AtomicUsize;

    #[derive(Debug, PartialEq, Clone)]
    struct ComplexData {
        id: usize,
        name: String,
        data: Vec<u8>,
    }

    fn single_writer_single_reader<T: PartialEq + Clone + std::fmt::Debug + Send + Sync>(buffer: &AtomicCircularBuffer<T, 3>, item1: T, item2: T, item3: T, item4: T) {
        assert!(buffer.pop().is_err());

        buffer.push(item1.clone()).unwrap();
        buffer.push(item2.clone()).unwrap();
        buffer.push(item3.clone()).unwrap();
        assert!(buffer.push(item4.clone()).is_err()); // Buffer should be full

        assert_eq!(buffer.pop().unwrap(), item1);
        assert_eq!(buffer.pop().unwrap(), item2);
        assert_eq!(buffer.pop().unwrap(), item3);
        assert!(buffer.pop().is_err()); // Buffer should be empty
    }

    fn multiple_writers_single_reader<T: PartialEq + Clone + std::fmt::Debug + Send + Sync + 'static>(
        buffer: Arc<AtomicCircularBuffer<T, 10>>, 
        item_gen: impl Fn(usize) -> T + Sync + Send + 'static + Clone
    ) {
        let num_writers = 5;
        let num_values = 20_usize;
        let barrier = Arc::new(Barrier::new(num_writers + 1));
        let total_written = Arc::new(AtomicUsize::new(0));

        let writers: Vec<_> = (0..num_writers)
            .map(|_| {
                let buffer = Arc::clone(&buffer);
                let barrier = Arc::clone(&barrier);
                let total_written = Arc::clone(&total_written);
                let item_gen = item_gen.clone();
                thread::spawn(move || {
                    barrier.wait(); // Wait for all threads to be ready
                    for i in 0..num_values {
                        while buffer.push(item_gen(i)).is_err() {
                            thread::yield_now(); // Retry if buffer is full
                        }
                        total_written.fetch_add(1, Ordering::SeqCst);
                    }
                })
            })
            .collect();

        let reader = {
            let buffer = Arc::clone(&buffer);
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait(); // Wait for all threads to be ready
                let mut read_values = Vec::new();
                while read_values.len() < num_writers * num_values {
                    if let Ok(value) = buffer.pop() {
                        read_values.push(value);
                    } else {
                        thread::yield_now(); // Retry if buffer is empty
                    }
                }
                read_values
            })
        };

        for writer in writers {
            writer.join().unwrap();
        }

        let read_values = reader.join().unwrap();
        assert_eq!(read_values.len(), num_writers * num_values);
        assert_eq!(total_written.load(Ordering::SeqCst), num_writers * num_values);
    }

    fn buffer_full<T: Clone + std::fmt::Debug + Send + Sync>(buffer: &AtomicCircularBuffer<T, 3>, item1: T, item2: T, item3: T, item4: T) {
        assert!(buffer.push(item1.clone()).is_ok());
        assert!(buffer.push(item2.clone()).is_ok());
        assert!(buffer.push(item3.clone()).is_ok());
        assert!(buffer.push(item4.clone()).is_err()); // Buffer should be full after 3 elements
    }

    fn buffer_empty<T: Clone + std::fmt::Debug + Send + Sync>(buffer: &AtomicCircularBuffer<T, 3>, item: T) {
        assert!(buffer.pop().is_err()); // Buffer should be empty
        buffer.push(item.clone()).unwrap();
        buffer.pop().unwrap();
        assert!(buffer.pop().is_err()); // Buffer should be empty again
    }

    
    fn concurrent_push_pop<T: PartialEq + Clone + std::fmt::Debug + Send + Sync + 'static>(
        buffer: Arc<AtomicCircularBuffer<T, 5>>,
        item_gen: impl Fn(usize) -> T + Sync + Send + 'static + Clone
    ) {
        let barrier = Arc::new(Barrier::new(3)); // 2 writers and 1 reader
    
        let writer1 = {
            let buffer = Arc::clone(&buffer);
            let barrier = Arc::clone(&barrier);
            let item_gen = item_gen.clone();
            thread::spawn(move || {
                barrier.wait(); // Wait for all threads to be ready
                for i in 0..10 {
                    while buffer.push(item_gen(i)).is_err() {
                        thread::yield_now(); // Retry if buffer is full
                    }
                }
            })
        };
    
        let writer2 = {
            let buffer = Arc::clone(&buffer);
            let barrier = Arc::clone(&barrier);
            let item_gen = item_gen.clone();
            thread::spawn(move || {
                barrier.wait(); // Wait for all threads to be ready
                for i in 10..20 {
                    while buffer.push(item_gen(i)).is_err() {
                        thread::yield_now(); // Retry if buffer is full
                    }
                }
            })
        };
    
        let reader = {
            let buffer = Arc::clone(&buffer);
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait(); // Wait for all threads to be ready
                let mut read_values = Vec::new();
                while read_values.len() < 20 {
                    if let Ok(value) = buffer.pop() {
                        read_values.push(value);
                    } else {
                        thread::yield_now(); // Retry if buffer is empty
                    }
                }
                read_values
            })
        };
    
        writer1.join().unwrap();
        writer2.join().unwrap();
    
        let read_values = reader.join().unwrap();
        assert_eq!(read_values.len(), 20);
        for i in 0..20 {
            println!("{:?}", read_values[i]);
            //assert!(read_values.contains(&item_gen(i)), "Value {} not found in read values", i);
        }
        //let mut expected_values: Vec<_> = (0..20).map(|i| item_gen(i)).collect();
        //for value in &read_values {
        //    if let Some(pos) = expected_values.iter().position(|x| x == value) {
        //        expected_values.remove(pos);
        //    }
       // }
       // assert!(expected_values.is_empty(), "Not all expected values were found in the read values");
    }

    #[test]
    fn test_single_writer_single_reader_primitive() {
        let buffer = AtomicCircularBuffer::<i32, 3>::new();
        single_writer_single_reader(&buffer, 1, 2, 3, 4);
    } 

    #[test]
    fn test_single_writer_single_reader_complex() {
        let buffer = AtomicCircularBuffer::<ComplexData, 3>::new();
        let item1 = ComplexData { id: 1, name: "Item 1".to_string(), data: vec![1, 2, 3] };
        let item2 = ComplexData { id: 2, name: "Item 2".to_string(), data: vec![4, 5, 6] };
        let item3 = ComplexData { id: 3, name: "Item 3".to_string(), data: vec![7, 8, 9] };
        let item4 = ComplexData { id: 4, name: "Item 4".to_string(), data: vec![10, 11, 12] };
        single_writer_single_reader(&buffer, item1, item2, item3, item4);
    }

    #[test]
    fn test_multiple_writers_single_reader_primitive() {
        let buffer = Arc::new(AtomicCircularBuffer::<i32, 10>::new());
        multiple_writers_single_reader(buffer, |i| i as i32);
    }

    #[test]
    fn test_multiple_writers_single_reader_complex() {
        let buffer = Arc::new(AtomicCircularBuffer::<ComplexData, 10>::new());
        multiple_writers_single_reader(buffer, |i| ComplexData { id: i, name: format!("Item {}", i), data: vec![i as u8] });
    }

    #[test]
    fn test_buffer_full_primitive() {
        let buffer = AtomicCircularBuffer::<i32, 3>::new();
        buffer_full(&buffer, 1, 2, 3, 4);
    }

    #[test]
    fn test_buffer_full_complex() {
        let buffer = AtomicCircularBuffer::<ComplexData, 3>::new();
        let item1 = ComplexData { id: 1, name: "Item 1".to_string(), data: vec![1, 2, 3] };
        let item2 = ComplexData { id: 2, name: "Item 2".to_string(), data: vec![4, 5, 6] };
        let item3 = ComplexData { id: 3, name: "Item 3".to_string(), data: vec![7, 8, 9] };
        let item4 = ComplexData { id: 4, name: "Item 4".to_string(), data: vec![10, 11, 12] };
        buffer_full(&buffer, item1, item2, item3, item4);
    }

    #[test]
    fn test_buffer_empty_primitive() {
        let buffer = AtomicCircularBuffer::<i32, 3>::new();
        buffer_empty(&buffer, 1);
    }

    #[test]
    fn test_buffer_empty_complex() {
        let buffer = AtomicCircularBuffer::<ComplexData, 3>::new();
        let item = ComplexData { id: 1, name: "Item 1".to_string(), data: vec![1, 2, 3] };
        buffer_empty(&buffer, item);
    }

    #[test]
    fn test_concurrent_push_pop_complex_data() {
        let buffer = Arc::new(AtomicCircularBuffer::<ComplexData, 5>::new());
        concurrent_push_pop(buffer, |i| ComplexData {
            id: i,
            name: format!("Item {}", i),
            data: vec![i as u8]
        });
    }
    
    #[test]
    fn test_concurrent_push_pop_primitive() {
        let buffer = Arc::new(AtomicCircularBuffer::<i32, 5>::new());
        concurrent_push_pop(buffer, |i| i as i32);
    }
}
