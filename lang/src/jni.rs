use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

/// Interpret the input.
///
/// # Arguments
///
/// * `env` - The JNI environment.
/// * `_class` - The class.
/// * `input` - The input to interpreter.
///
/// # Returns
///
/// * `jstring` - The result of the interpretation.
///
/// # Panics
///
/// * If the input cannot be retrieved.
/// * If the input is syntactically incorrect.
/// * If the input is semantically incorrect.
/// * If the output cannot be created
#[no_mangle]
pub extern "system" fn interpret<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JString<'local>,
) -> jstring {
    let input: String = env.get_string(&input).expect("Failed to get input!").into();

    let tokens = Lexer::new(&input).lex().expect("Failed to lex input!");
    let ast = Parser::new(&tokens)
        .parse()
        .expect("Failed to parse tokens!");

    let mut interpreter = Interpreter::new(&ast);
    interpreter.run();

    env.new_string(format!("{interpreter}"))
        .expect("Failed to create new string!")
        .into_raw()
}
