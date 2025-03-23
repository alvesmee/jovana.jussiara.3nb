// Ativa a execução do módulo de testes apenas quando a configuração de compilação for para testes (geralmente em modo de teste).
#[cfg(test)]
mod tests {
    // Importa a estrutura `Queue` do módulo `fila_generica::queue` para usá-la nos testes.
    use fila_generica::queue::Queue;

    // Define o primeiro teste, verificando o funcionamento de `enqueue` (adicionar) e `dequeue` (remover).
    #[test]
    fn test_enqueue_dequeue() {
        // Cria uma nova fila vazia.
        let mut fila = Queue::new();
        
        // Adiciona o valor 10 à fila.
        fila.enqueue(10);
        
        // Adiciona o valor 20 à fila.
        fila.enqueue(20);
        
        // Verifica se o primeiro elemento removido da fila é 10.
        assert_eq!(fila.dequeue(), Some(10));
        
        // Verifica se o próximo elemento removido da fila é 20.
        assert_eq!(fila.dequeue(), Some(20));
        
        // Verifica se a fila está vazia após remover os dois elementos, retornando `None`.
        assert_eq!(fila.dequeue(), None);
    }

    // Define o segundo teste, verificando o comportamento do método `peek`, que retorna o primeiro valor sem removê-lo.
    #[test]
    fn test_peek() {
        // Cria uma nova fila vazia.
        let mut fila = Queue::new();
        
        // Adiciona o valor 100 à fila.
        fila.enqueue(100);
        
        // Verifica se o primeiro valor da fila é 100, sem removê-lo.
        assert_eq!(fila.peek(), Some(&100));
        
        // Remove o elemento 100 da fila.
        fila.dequeue();
        
        // Verifica se, após a remoção, o método `peek` retorna `None` (indicando que a fila está vazia).
        assert_eq!(fila.peek(), None);
    }

    // Define o terceiro teste, verificando o método `len`, que retorna o tamanho atual da fila.
    #[test]
    fn test_len() {
        // Cria uma nova fila vazia.
        let mut fila = Queue::new();
        
        // Verifica se a fila está vazia inicialmente (tamanho deve ser 0).
        assert_eq!(fila.len(), 0);
        
        // Adiciona os valores 5 e 15 à fila.
        fila.enqueue(5);
        fila.enqueue(15);
        
        // Verifica se o tamanho da fila é 2 após adicionar dois elementos.
        assert_eq!(fila.len(), 2);
        
        // Remove um elemento da fila.
        fila.dequeue();
        
        // Verifica se o tamanho da fila é 1 após remover um elemento.
        assert_eq!(fila.len(), 1);
    }
}
