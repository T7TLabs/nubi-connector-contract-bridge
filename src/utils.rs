use aurora_engine_types::{H256, U256};
use ethabi::{ethereum_types::Address, Token};
use near_sdk::env;

pub fn from_string_to_address(input: &str) -> Address {
    let mut contract_u820 = [0u8; 20];
    assert_eq!(
        hex::decode_to_slice(input, &mut contract_u820 as &mut [u8]),
        Ok(())
    );
    Address::from(contract_u820)
}

#[inline]
pub fn keccak(input: &[u8]) -> H256 {
    H256::from_slice(&env::keccak256(input))
}

fn get_selector(str_selector: &str) -> Vec<u8> {
    keccak(str_selector.as_bytes())[..4].to_vec()
}

fn build_input(str_selector: &str, inputs: &[Token]) -> Vec<u8> {
    let sel = get_selector(str_selector);
    let inputs = ethabi::encode(inputs);
    [sel.as_slice(), inputs.as_slice()].concat()
}

lazy_static! {
    static ref ERROR_PARSING: &'static str = "Invalid input parameter";
}

fn get_numerical_part(parameter: &str) -> Option<usize> {
    let parameter = parameter.split('[').next().unwrap_or(parameter);
    let numeric: String = parameter.chars().filter(char::is_ascii_digit).collect();
    numeric.parse::<usize>().ok()
}

fn numeric_sanity_check(parameter: &str, numerical: usize) {
    match parameter {
        int if int.contains("int") => {
            assert!(
                numerical <= 256,
                "The numerical part in {} is greater than 256",
                parameter,
            );
            assert_eq!(
                numerical % 8,
                0,
                "The numerical part in {} is not divisible by 8",
                parameter,
            );
        }
        bytes if bytes.contains("bytes") => assert!(
            0 < numerical && numerical <= 32,
            "The numerical part in bytes{} is out of bounds [1, 32]",
            numerical
        ),
        _ => panic!("The type {} does not support numerical", parameter),
    }
}

fn solidity_lookup(type_part: &str, value: &str) -> Token {
    match type_part {
        uint if uint.contains("uint") => {
            Token::Uint(U256::from_dec_str(value).expect(*ERROR_PARSING))
        }
        int if int.contains("int") => Token::Int(U256::from_dec_str(value).expect(*ERROR_PARSING)),
        "address" => {
            let mut addr = [0u8; 20];
            hex::decode_to_slice(value, &mut addr as &mut [u8]).expect(*ERROR_PARSING);
            Token::Address(Address::from(addr))
        }
        "bool" => Token::Bool(value.parse::<bool>().expect(*ERROR_PARSING)),
        "bytes" => Token::Bytes(hex::decode(value).expect(*ERROR_PARSING)),
        "string" => Token::String(value.to_string()),
        bytes if bytes.contains("bytes") => {
            Token::FixedBytes(hex::decode(value).expect(*ERROR_PARSING))
        }
        "function" => {
            let bytes24 = value.as_bytes().to_vec();
            assert_eq!(bytes24.len(), 24, "function input parameter size incorrect");
            Token::Bytes(bytes24)
        }
        &_ => panic!("Unsupported format {}", type_part),
    }
}

fn group_array(array: &str) -> Vec<String> {
    if !array.starts_with("[[") {
        array
            .split([' ', '[', ',', ']'])
            .into_iter()
            .filter(|char| !char.is_empty())
            .map(Into::into)
            .collect::<Vec<_>>()
    } else {
        let mut start: usize = 0;
        let mut acc: usize = 0;
        let mut result = vec![];

        for (i, c) in array[0..array.len() - 1]
            .chars()
            .into_iter()
            .enumerate()
            .skip(1)
        {
            if c.to_string() == "[" {
                if acc == 0 {
                    start = i;
                }
                acc += 1;
            } else if c.to_string() == "]" {
                acc -= 1;
                if acc == 0 {
                    result.push(array[start..=i].to_string());
                }
            }
        }
        result
    }
}

// [[1,2],[2,3]]
fn tokenize(parameter_type: &str, parameter_value: &str) -> Token {
    if parameter_type.ends_with("[]") {
        let function = if parameter_type.ends_with("][]") {
            tokenize
        } else {
            solidity_lookup
        };

        Token::Array(
            group_array(parameter_value)
                .into_iter()
                .map(|v| (function)(&parameter_type[0..parameter_type.len() - 2], &v))
                .collect::<Vec<_>>(),
        )
    } else if parameter_type.ends_with(']') {
        let function =
            if &parameter_type[parameter_type.len() - 4..parameter_type.len() - 2] == "][" {
                tokenize
            } else {
                solidity_lookup
            };

        Token::FixedArray(
            group_array(parameter_value)
                .into_iter()
                .map(|v| (function)(&parameter_type[0..parameter_type.len() - 3], &v))
                .collect::<Vec<_>>(),
        )
    } else {
        solidity_lookup(parameter_type, parameter_value)
    }
}

pub(crate) fn solidity_function(function: &str, values: &[String]) -> Vec<u8> {
    let parameters = function
        .split(['(', ')', ',', ' '])
        .skip(1)
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>();

    assert_eq!(
        parameters.len(),
        values.len(),
        "Number of parameters don't match"
    );

    let parameters_token = parameters
        .iter()
        .enumerate()
        .map(|(i, parameter)| {
            // Check the numerical part is correct
            if let Some(numerical) = get_numerical_part(parameter) {
                numeric_sanity_check(parameter, numerical);
            }

            tokenize(parameter, &values[i])
        })
        .collect::<Vec<_>>();

    build_input(function, &parameters_token)
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{build_input, solidity_function};
    use ethabi::Token;

    #[test]
    fn test_group_array() {
        assert_eq!(
            group_array("[10, 12, 13, 42]"),
            vec!["10", "12", "13", "42"]
        );
        assert_eq!(group_array("[10 12 13 42]"), vec!["10", "12", "13", "42"]);
        assert_eq!(
            group_array("[[10, 12],[13, 42]]"),
            vec!["[10, 12]", "[13, 42]"]
        );
        assert_eq!(group_array("[[10 12][13 42]]"), vec!["[10 12]", "[13 42]"]);
        assert_eq!(
            group_array("[[[10, 1], [12, 2]],[[13, 3], [42, 4]]]"),
            vec!["[[10, 1], [12, 2]]", "[[13, 3], [42, 4]]"]
        );
        assert_eq!(
            group_array("[[[10 1] [12 2]][[13 3] [42 4]]]"),
            vec!["[[10 1] [12 2]]", "[[13 3] [42 4]]"]
        );
    }

    #[test]
    fn test_tokenize_array() {
        assert_eq!(
            tokenize("uint256", "1234"),
            Token::Uint(U256::from_dec_str("1234").expect("valid uint256"))
        );
        assert_eq!(
            tokenize("uint256[]", "[1234, 4321]"),
            Token::Array(vec![
                Token::Uint(U256::from_dec_str("1234").expect("valid uint256")),
                Token::Uint(U256::from_dec_str("4321").expect("valid uint256"))
            ]),
        );
        assert_eq!(
            tokenize("uint256[][]", "[[12, 34], [43, 21]]"),
            Token::Array(vec![
                Token::Array(vec![
                    Token::Uint(U256::from_dec_str("12").expect("valid uint256")),
                    Token::Uint(U256::from_dec_str("34").expect("valid uint256"))
                ]),
                Token::Array(vec![
                    Token::Uint(U256::from_dec_str("43").expect("valid uint256")),
                    Token::Uint(U256::from_dec_str("21").expect("valid uint256"))
                ]),
            ]),
        );
        assert_eq!(
            tokenize(
                "uint256[][][]",
                "[[[1], [2], [3], [4]], [[4], [3], [2], [1]]]"
            ),
            Token::Array(vec![
                Token::Array(vec![
                    Token::Array(vec![Token::Uint(
                        U256::from_dec_str("1").expect("valid uint256")
                    ),]),
                    Token::Array(vec![Token::Uint(
                        U256::from_dec_str("2").expect("valid uint256")
                    ),]),
                    Token::Array(vec![Token::Uint(
                        U256::from_dec_str("3").expect("valid uint256")
                    ),]),
                    Token::Array(vec![Token::Uint(
                        U256::from_dec_str("4").expect("valid uint256")
                    ),]),
                ]),
                Token::Array(vec![
                    Token::Array(vec![Token::Uint(
                        U256::from_dec_str("4").expect("valid uint256")
                    ),]),
                    Token::Array(vec![Token::Uint(
                        U256::from_dec_str("3").expect("valid uint256")
                    ),]),
                    Token::Array(vec![Token::Uint(
                        U256::from_dec_str("2").expect("valid uint256")
                    ),]),
                    Token::Array(vec![Token::Uint(
                        U256::from_dec_str("1").expect("valid uint256")
                    ),]),
                ]),
            ]),
        );
    }

    #[test]
    fn test_tokenize_fixed_array() {
        assert_eq!(
            tokenize("uint256[2]", "[1234, 4321]"),
            Token::FixedArray(vec![
                Token::Uint(U256::from_dec_str("1234").expect("valid uint256")),
                Token::Uint(U256::from_dec_str("4321").expect("valid uint256"))
            ]),
        );
        assert_eq!(
            tokenize("uint256[2][2]", "[[12, 34], [43, 21]]"),
            Token::FixedArray(vec![
                Token::FixedArray(vec![
                    Token::Uint(U256::from_dec_str("12").expect("valid uint256")),
                    Token::Uint(U256::from_dec_str("34").expect("valid uint256"))
                ]),
                Token::FixedArray(vec![
                    Token::Uint(U256::from_dec_str("43").expect("valid uint256")),
                    Token::Uint(U256::from_dec_str("21").expect("valid uint256"))
                ]),
            ]),
        );
        assert_eq!(
            tokenize(
                "uint256[2][2][2]",
                "[[[1], [2], [3], [4]], [[4], [3], [2], [1]]]"
            ),
            Token::FixedArray(vec![
                Token::FixedArray(vec![
                    Token::FixedArray(vec![Token::Uint(
                        U256::from_dec_str("1").expect("valid uint256")
                    ),]),
                    Token::FixedArray(vec![Token::Uint(
                        U256::from_dec_str("2").expect("valid uint256")
                    ),]),
                    Token::FixedArray(vec![Token::Uint(
                        U256::from_dec_str("3").expect("valid uint256")
                    ),]),
                    Token::FixedArray(vec![Token::Uint(
                        U256::from_dec_str("4").expect("valid uint256")
                    ),]),
                ]),
                Token::FixedArray(vec![
                    Token::FixedArray(vec![Token::Uint(
                        U256::from_dec_str("4").expect("valid uint256")
                    ),]),
                    Token::FixedArray(vec![Token::Uint(
                        U256::from_dec_str("3").expect("valid uint256")
                    ),]),
                    Token::FixedArray(vec![Token::Uint(
                        U256::from_dec_str("2").expect("valid uint256")
                    ),]),
                    Token::FixedArray(vec![Token::Uint(
                        U256::from_dec_str("1").expect("valid uint256")
                    ),]),
                ]),
            ]),
        );
    }

    #[test]
    fn test_tokenize_mixed_array() {
        assert_eq!(
            tokenize("uint256[][2]", "[[12, 34], [43, 21]]"),
            Token::FixedArray(vec![
                Token::Array(vec![
                    Token::Uint(U256::from_dec_str("12").expect("valid uint256")),
                    Token::Uint(U256::from_dec_str("34").expect("valid uint256"))
                ]),
                Token::Array(vec![
                    Token::Uint(U256::from_dec_str("43").expect("valid uint256")),
                    Token::Uint(U256::from_dec_str("21").expect("valid uint256"))
                ]),
            ]),
        );
        assert_eq!(
            tokenize(
                "uint256[2][][2]",
                "[[[1], [2], [3], [4]], [[4], [3], [2], [1]]]"
            ),
            Token::FixedArray(vec![
                Token::Array(vec![
                    Token::FixedArray(vec![Token::Uint(
                        U256::from_dec_str("1").expect("valid uint256")
                    ),]),
                    Token::FixedArray(vec![Token::Uint(
                        U256::from_dec_str("2").expect("valid uint256")
                    ),]),
                    Token::FixedArray(vec![Token::Uint(
                        U256::from_dec_str("3").expect("valid uint256")
                    ),]),
                    Token::FixedArray(vec![Token::Uint(
                        U256::from_dec_str("4").expect("valid uint256")
                    ),]),
                ]),
                Token::Array(vec![
                    Token::FixedArray(vec![Token::Uint(
                        U256::from_dec_str("4").expect("valid uint256")
                    ),]),
                    Token::FixedArray(vec![Token::Uint(
                        U256::from_dec_str("3").expect("valid uint256")
                    ),]),
                    Token::FixedArray(vec![Token::Uint(
                        U256::from_dec_str("2").expect("valid uint256")
                    ),]),
                    Token::FixedArray(vec![Token::Uint(
                        U256::from_dec_str("1").expect("valid uint256")
                    ),]),
                ]),
            ]),
        );
    }

    #[test]
    #[should_panic(expected = "Invalid input parameter: OddLength")]
    fn wrong_address_odd() {
        solidity_function(
            "name(address,address)",
            &[String::from("something"), String::from("something")],
        );
    }

    #[test]
    #[should_panic(expected = "Invalid input parameter: InvalidStringLength")]
    fn wrong_address_invalid_length() {
        solidity_function(
            "name(address,address)",
            &[String::from("somethings"), String::from("somethings")],
        );
    }

    #[test]
    #[should_panic(expected = "Unsupported format addrress")]
    fn unsupported_format() {
        solidity_function(
            "name(addrress,address)",
            &[String::from("somethings"), String::from("somethings")],
        );
    }

    #[test]
    #[should_panic(expected = "Number of parameters don't match")]
    fn number_dont_match_1() {
        solidity_function(
            "name(address,address,address)",
            &[String::from("somethings"), String::from("somethings")],
        );
    }

    #[test]
    #[should_panic(expected = "Number of parameters don't match")]
    fn number_dont_match_2() {
        solidity_function(
            "name(address,address)",
            &[
                String::from("somethings"),
                String::from("somethings"),
                String::from("somethings"),
            ],
        );
    }

    #[test]
    #[should_panic(expected = "Invalid input parameter: InvalidCharacter")]
    fn invalid_uint256() {
        solidity_function("name(uint256)", &[String::from("somethings")]);
    }

    #[test]
    fn valid_uint256() {
        solidity_function("name(uint256)", &[String::from("589")]);
    }

    #[test]
    #[should_panic(expected = "The numerical part in uint257 is greater than 256")]
    fn invalid_uint257() {
        solidity_function("name(uint257)", &[String::from("589")]);
    }

    #[test]
    #[should_panic(expected = "The numerical part in uint250 is not divisible by 8")]
    fn invalid_uint250() {
        solidity_function("name(uint250)", &[String::from("589")]);
    }

    #[test]
    fn simple_test_address() {
        let function = "getScheduled(address,address)";
        let user = String::from("2b8496768299a9C8e8957589CfA8ea48fa4d5A42");
        let pool = String::from("4BaaD27a98D048295CC50509Fb99BC588926f368");
        let mut user_u820 = [0u8; 20];
        assert_eq!(
            hex::decode_to_slice(user.clone(), &mut user_u820 as &mut [u8]),
            Ok(())
        );

        let mut pool_u820 = [0u8; 20];
        assert_eq!(
            hex::decode_to_slice(pool.clone(), &mut pool_u820 as &mut [u8]),
            Ok(())
        );

        let correct_input = build_input(
            function,
            &[
                Token::Address(Address::from(user_u820)),
                Token::Address(Address::from(pool_u820)),
            ],
        );
        let input = solidity_function(function, &[user, pool]);

        assert_eq!(input, correct_input);
    }

    #[test]
    fn simple_test_uint() {
        let function = "getScheduled(uint)";
        let value_uint: &str = "4294967295";

        let correct_input = build_input(
            function,
            &[Token::Uint(
                U256::from_dec_str(value_uint).expect("valid uint256"),
            )],
        );

        let input = solidity_function(function, &[value_uint.to_string()]);

        assert_eq!(input, correct_input);
    }

    #[test]
    fn simple_test_uint256() {
        let function = "getScheduled(uint256)";
        let value_uint256: &str = "123456789";

        let correct_input = build_input(
            function,
            &[Token::Uint(
                U256::from_dec_str(value_uint256).expect("valid uint256"),
            )],
        );

        let input = solidity_function(function, &[value_uint256.to_string()]);

        assert_eq!(input, correct_input);
    }

    #[test]
    fn simple_test_array_simple() {
        let function = "getScheduled(uint256[])";
        let value_uint256: &str = "4294967295";

        let correct_input = build_input(
            function,
            &[Token::Array(vec![Token::Uint(
                U256::from_dec_str(value_uint256).expect("valid uint256"),
            )])],
        );

        let values = format!("[{}]", value_uint256.to_string());
        let input = solidity_function(function, &[values]);

        assert_eq!(input, correct_input);
    }

    #[test]
    fn simple_test_array_complex() {
        let function = "getScheduled(uint256[],bool)";
        let value_1_uint256: &str = "123456789";
        let value_2_uint256: &str = "987654321";
        let value_3_bool: &str = "true";

        let correct_input = build_input(
            function,
            &[
                Token::Array(vec![
                    Token::Uint(U256::from_dec_str(value_1_uint256).expect("valid uint256")),
                    Token::Uint(U256::from_dec_str(value_2_uint256).expect("valid uint256")),
                ]),
                Token::Bool(value_3_bool.parse::<bool>().expect("valid boolean")),
            ],
        );

        // getScheduled(uint256[], bool), ["[123456789, 987654321], true"])
        let values = format!(
            "[{},{}]",
            value_1_uint256.to_string(),
            value_2_uint256.to_string()
        );
        let input = solidity_function(function, &[values, value_3_bool.to_string()]);

        assert_eq!(input, correct_input);
    }

    #[test]
    fn simple_test_bytes() {
        let function = "getScheduled(bytes)";
        let value_bytes: &str = "13";

        let correct_input = build_input(function, &[Token::Bytes(vec![0x13])]);

        let input = solidity_function(function, &[value_bytes.to_string()]);

        assert_eq!(input, correct_input);
    }

    #[test]
    fn simple_test_bytes_sized() {
        let function = "getScheduled(bytes10,bytes)";
        let value_bytes_sized: &str = "13";
        let value_bytes: &str = "20";

        let correct_input = build_input(
            function,
            &[Token::FixedBytes(vec![0x13]), Token::Bytes(vec![0x20])],
        );

        let input = solidity_function(
            function,
            &[value_bytes_sized.to_string(), value_bytes.to_string()],
        );

        assert_eq!(input, correct_input);
    }

    #[test]
    #[should_panic(expected = "The numerical part in bytes33 is out of bounds [1, 32]")]
    fn incorrect_upper_bound_bytes_sized() {
        solidity_function("name(bytes33)", &[String::from("10")]);
    }

    #[test]
    #[should_panic(expected = "The numerical part in bytes0 is out of bounds [1, 32]")]
    fn incorrect_lower_bound_bytes_sized() {
        solidity_function("name(bytes0)", &[String::from("10")]);
    }

    #[test]
    fn simple_double_unsized_array() {
        let function = "getScheduled(uint256[][])";
        let value_bytes: &str = "[[13,34],[43,21]]";

        let correct_input = build_input(
            function,
            &[Token::Array(vec![
                Token::Array(vec![
                    Token::Uint(U256::from_dec_str("13").expect("valid uint256")),
                    Token::Uint(U256::from_dec_str("34").expect("valid uint256")),
                ]),
                Token::Array(vec![
                    Token::Uint(U256::from_dec_str("43").expect("valid uint256")),
                    Token::Uint(U256::from_dec_str("21").expect("valid uint256")),
                ]),
            ])],
        );

        let input = solidity_function(function, &[value_bytes.to_string()]);

        assert_eq!(input, correct_input);
    }

    #[test]
    fn simple_sized_bytes_array() {
        let function = "getScheduled(bytes[2])";
        let value_bytes: &str = "[1315,3430]";

        let correct_input = build_input(
            function,
            &[Token::FixedArray(vec![
                Token::Bytes(vec![0x13, 0x15]),
                Token::Bytes(vec![0x34, 0x30]),
            ])],
        );

        let input = solidity_function(function, &[value_bytes.to_string()]);

        assert_eq!(input, correct_input);
    }

    #[test]
    fn simple_sized_bytes() {
        let function = "getScheduled(bytes2)";
        let value_bytes: &str = "13153430";

        let correct_input =
            build_input(function, &[Token::FixedBytes(vec![0x13, 0x15, 0x34, 0x30])]);

        let input = solidity_function(function, &[value_bytes.to_string()]);

        assert_eq!(input, correct_input);
    }
}
