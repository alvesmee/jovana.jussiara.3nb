// Função unsafe que multiplica os elementos de um array usando ponteiros brutos.
pub unsafe fn multiply_array(ptr: *const i32, len: usize) -> i32 {
    let mut product = 1; // Inicializa o produto como 1.
    for i in 0..len {
        product *= *ptr.offset(i as isize); // Acessa e multiplica o valor de cada elemento do array.
    }
    product // Retorna o produto final.
}

#[cfg(test)]
mod tests {
    use super::*;

    // Teste unitário para a função multiply_array.
    #[test]
    fn test_multiply_array() {
        let arr = [2, 3, 4]; // Array de exemplo.
        let product = unsafe { multiply_array(arr.as_ptr(), arr.len()) }; // Chama a função unsafe.
        assert_eq!(product, 24); // Verifica se o produto é 24.
    }
}

// Função main (não utilizada no código, apenas imprime uma mensagem).
fn main() {
    println!("Hello, world!"); // Exibe "Hello, world!".
}
