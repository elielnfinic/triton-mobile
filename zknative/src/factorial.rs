use triton_vm::prelude::*;
use serde::{Serialize, Deserialize};

pub fn factorial(number: u64) -> u64 {
    let factorial_program = triton_program!(
            // op stack:
    read_io 1           // n
    push 1              // n accumulator
    call factorial      // 0 accumulator!
    write_io 1          // 0
    halt

    factorial:          // n acc
    // if n == 0: return
    dup 1           // n acc n
    push 0 eq       // n acc n==0
    skiz            // n acc
    return      // 0 acc
    // else: multiply accumulator with n and recurse
    dup 1           // n acc n
    mul             // n acc·n
    swap 1          // acc·n n
    push -1 add     // acc·n n-1
    swap 1          // n-1 acc·n
    recurse
    );

    let public_input = PublicInput::from([bfe!(number)]);
    let non_determinism = NonDeterminism::default();
    let (stark, claim, proof) =
        triton_vm::prove_program(factorial_program, public_input, non_determinism).unwrap();

    let verdict = triton_vm::verify(stark, &claim, &proof);
    assert!(verdict);
    println!("The proof verdict is {verdict} and number is {number}");
    // println!("Successfully verified proof. {proof:?}");
    println!("Raw output is {:?}", claim.output);
    let mut output_num = bfe!(0);
    for out_put in claim.output.iter() {
        output_num = *out_put;
    }
    let claimed_output = claim.output.iter().map(|o| o.value());
    println!("Verifiably correct output:  {claimed_output:?}");

    let conjectured_security_level = stark.security_level;
    println!("Conjectured security level is {conjectured_security_level} bits.");

    let upper_bound_of_execution_steps = proof.padded_height().unwrap();
    println!("Executing the program took at most {upper_bound_of_execution_steps} cycles.");

    output_num.value()
}

#[derive(Serialize, Deserialize)]
pub struct TritonResult{
    pub output : u64,
    pub proof : Vec<u64>
}

pub fn factorial_meta(number: u64) -> TritonResult {
    let factorial_program = triton_program!(
            // op stack:
    read_io 1           // n
    push 1              // n accumulator
    call factorial      // 0 accumulator!
    write_io 1          // 0
    halt

    factorial:          // n acc
    // if n == 0: return
    dup 1           // n acc n
    push 0 eq       // n acc n==0
    skiz            // n acc
    return      // 0 acc
    // else: multiply accumulator with n and recurse
    dup 1           // n acc n
    mul             // n acc·n
    swap 1          // acc·n n
    push -1 add     // acc·n n-1
    swap 1          // n-1 acc·n
    recurse
    );

    let public_input = PublicInput::from([bfe!(number)]);
    let non_determinism = NonDeterminism::default();
    let (stark, claim, proof) =
        triton_vm::prove_program(factorial_program, public_input, non_determinism).unwrap();

    let verdict = triton_vm::verify(stark, &claim, &proof);
    assert!(verdict);
    println!("The proof verdict is {verdict} and number is {number}");
    // println!("Successfully verified proof. {proof:?}");
    println!("Raw output is {:?}", claim.output);
    let mut output_num = bfe!(0);
    for out_put in claim.output.iter() {
        output_num = *out_put;
    }
    let claimed_output = claim.output.iter().map(|o| o.value());
    println!("Verifiably correct output:  {claimed_output:?}");

    let conjectured_security_level = stark.security_level;
    println!("Conjectured security level is {conjectured_security_level} bits.");

    let upper_bound_of_execution_steps = proof.padded_height().unwrap();
    println!("Executing the program took at most {upper_bound_of_execution_steps} cycles.");

    TritonResult{
        output : output_num.value(),
        proof : proof.encode().iter().map(|o| o.value()).collect()
    }    
}

#[cfg(test)]
mod test {
    use crate::factorial::factorial_meta;

    use super::factorial;

    #[test]
    fn test_run_factorial() {
        factorial(5);
    }

    #[test]
    fn test_run_factorial_meta() {
        let result = factorial_meta(5);
        println!("Output is {} and proof is {:?}", result.output, result.proof);
        println!("The JSON output is {:?}", serde_json::to_string(&result).unwrap().as_bytes());
    }
}
