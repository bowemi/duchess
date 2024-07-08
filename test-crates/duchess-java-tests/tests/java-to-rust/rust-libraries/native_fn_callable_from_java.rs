//@check-pass

use duchess::prelude::*;

duchess::java_package! {
    package java_to_rust_greeting;

    public class Java_Can_Call_Rust_Java_Function {
        native java.lang.String base_greeting(java.lang.String);
        native int getInt();
        native int echoInt(int);
    }
}

#[duchess::java_function(java_to_rust_greeting.Java_Can_Call_Rust_Java_Function::base_greeting)]
fn base_greeting(
    _this: &java_to_rust_greeting::Java_Can_Call_Rust_Java_Function,
    name: &java::lang::String,
) -> duchess::Result<Java<java::lang::String>> {
    Ok(name.execute().unwrap())
}

#[duchess::java_function(java_to_rust_greeting.Java_Can_Call_Rust_Java_Function::getInt)]
fn get_int(
    _this: &java_to_rust_greeting::Java_Can_Call_Rust_Java_Function,
) -> duchess::Result<i32> {
    Ok(0)
}

#[duchess::java_function(java_to_rust_greeting.Java_Can_Call_Rust_Java_Function::echoInt)]
fn echo_int(
    _this: &java_to_rust_greeting::Java_Can_Call_Rust_Java_Function,
    input: i32,
) -> duchess::Result<i32> {
    input
}
