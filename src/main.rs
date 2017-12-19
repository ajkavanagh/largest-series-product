extern crate time;

use time::get_time;


#[derive(Debug)]
pub enum Error {
    Digit,
    Window,
}

// to calcuate the products over the string, we need to keep an array of the product that we want
// to do from the digits.  multipication is quite quick, so rather than dividing by the last digit
// and then multiplying by the new digit, we'll just recalculate the whole product.

pub fn lsp_imperative(digits: &str, size: u32) -> Result<u32, Error> {
    if size == 0 {
        return Ok(1);
    };
    let mut prod = vec![0u32; size as usize];
    let mut pointer: usize = 0;
    let mut max_value: u32 = 0;
    let mut count: u32 = 0;
    for c in digits.chars() {
        count += 1;
        prod[pointer] = c.to_digit(10).ok_or(Error::Digit)?;
        pointer = pointer + 1;
        if pointer >= size as usize {
            pointer = 0;
        }
        let mut sum: u32 = 1;
        for p in 0..size as usize {
            sum *= prod[p];
        }
        if sum > max_value {
            max_value = sum;
        }
    }
    if count < size {
        return Err(Error::Window);
    }
    Ok(max_value)
}


pub fn lsp_functional(digits: &str, size: u32) -> Result<u32, Error> {
    if size == 0 {
        return Ok(1);
    };
    digits
        .chars()
        .map(|c| c.to_digit(10).ok_or(Error::Digit))
        .collect::<Result<Vec<u32>, _>>()
        .and_then(|numbers| {
                      numbers
                          .windows(size as usize)
                          .map(|w| w.iter().product())
                          .max()
                          .ok_or(Error::Window)
                  })
}


macro_rules! timeit {
    ($loops:expr, $code:block) => ({
        let n = $loops;
        let start = get_time();
        for _ in 0..n {
            $code
        }
        let end = get_time();
        // return the seconds
        ((end.sec - start.sec) as f64 +
         (end.nsec - start.nsec) as f64 / 1_000_000_000.0)
    })
}

fn main() {
    let mut value: u32 = 0;
    let sec1 = timeit!(10_000_000, {
        value = lsp_imperative("0123045678912345678987654111110", 6).unwrap();
    });
    println!("LSP small imperative/mutable: {}: {}", value, sec1);
    let sec2 = timeit!(10_000_000, {
        value = lsp_functional("0123045678912345678987654111110", 6).unwrap();
    });
    println!("LSP small functional: {}: {}", value, sec2);
    let sec3 = timeit!(10_000_000, {
        value = lsp_imperative("012345678909827346598172346598172364019862359187256394871620384756102837564109827365401982736549817256", 20).unwrap();
    });
    println!("LSP large imperative/mutable: {}: {}", value, sec3);
    let sec4 = timeit!(10_000_000, {
        value = lsp_functional("012345678909827346598172346598172364019862359187256394871620384756102837564109827365401982736549817256", 20).unwrap();
    });
    println!("LSP large functional: {}: {}", value, sec4);

}
