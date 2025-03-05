mod queue;
use queue::Queue;

fn main() {
    let mut fila = Queue::new();
    fila.enqueue(10);
    fila.enqueue(20);

    println!("Primeiro elemento: {:?}", fila.peek());
    println!("Removido: {:?}", fila.dequeue());
    println!("Tamanho atual: {}", fila.len());
}
