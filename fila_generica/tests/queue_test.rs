#[cfg(test)]
mod tests {
    use fila_generica::queue::Queue;

    #[test]
    fn test_enqueue_dequeue() {
        let mut fila = Queue::new();
        fila.enqueue(10);
        fila.enqueue(20);
        assert_eq!(fila.dequeue(), Some(10));
        assert_eq!(fila.dequeue(), Some(20));
        assert_eq!(fila.dequeue(), None);
    }

    #[test]
    fn test_peek() {
        let mut fila = Queue::new();
        fila.enqueue(100);
        assert_eq!(fila.peek(), Some(&100));
        fila.dequeue();
        assert_eq!(fila.peek(), None);
    }

    #[test]
    fn test_len() {
        let mut fila = Queue::new();
        assert_eq!(fila.len(), 0);
        fila.enqueue(5);
        fila.enqueue(15);
        assert_eq!(fila.len(), 2);
        fila.dequeue();
        assert_eq!(fila.len(), 1);
    }
}