mod queue;      // Declara o módulo público `queue`, que provavelmente contém a definição da estrutura `Queue` e seus métodos.
use queue::Queue;  // Importa a estrutura `Queue` do módulo `queue` para ser utilizada no código.

fn main() {  // Cria uma nova instância da fila, usando o método `new()` definido na estrutura `Queue`.
    let mut fila = Queue::new();
    fila.enqueue(10);  // Adiciona o valor 10 à fila, usando o método `enqueue` da estrutura `Queue`.
    fila.enqueue(20);  // Adiciona o valor 20 à fila.

    println!("Primeiro elemento: {:?}", fila.peek()); // Imprime o primeiro elemento da fila sem removê-lo, usando o método `peek`.
    println!("Removido: {:?}", fila.dequeue());  // Remove e retorna o primeiro elemento da fila, usando o método `dequeue`.
    println!("Tamanho atual: {}", fila.len()); // Imprime o tamanho atual da fila, usando o método `len`.
}

