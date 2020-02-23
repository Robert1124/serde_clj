use jni::objects::{JClass, JObject};
use jni::sys::{jint, jobject};
use jni::JNIEnv;
use serde::{Serialize, Deserialize};
use serde_clj::{to_object, Encoder, from_object, Decoder};
use std::collections::HashMap;
use std::iter::repeat;

#[derive(Deserialize, Serialize, Clone, Debug)]
enum Vars {
    Zero,
    One(usize),
    Two(String),
    Three(HashMap<i32, String>),
    Four { a: i32, b: bool, s: String },
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Test {
    value: Vec<i64>,
    another_field: Option<String>,
    a_string: String,
    enumerate: Vec<Vars>,
}

#[no_mangle]
pub extern "system" fn Java_Test_ser(env: JNIEnv, _: JClass, n: jint) -> jobject {
    let enc = Encoder::new(env).unwrap();
    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(7, "test".into());
    let test = Test {
        value: vec![1, 2, 3],
        another_field: None,
        a_string: "test".into(),
        enumerate: vec![
            Vars::Zero,
            Vars::One(1),
            Vars::Two("three".into()),
            Vars::Three(map),
            Vars::Four {
                a: 1,
                b: true,
                s: "ok?".into(),
            },
        ],
    };
    let vec = repeat(test).take(n as usize).collect::<Vec<_>>();
    let output = to_object(&enc, &vec).expect("serialisation failed!");
    output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_Test_de(env: JNIEnv, _: JClass, obj: JObject) {
    let dec = Decoder::new(env).unwrap();
    let out: Vec<Test> = from_object(&dec, obj).expect("deserialisation failed");
    println!("{:?}", out);
}

#[no_mangle]
pub extern "system" fn Java_Test_roundtrip(env: JNIEnv, _: JClass, obj: JObject) -> jobject {
    // making an encoder and a decoder uses a lot of local refs to
    // cache class & method ids
    env.ensure_local_capacity(64).expect("failed increasing capacity");
    let dec = Decoder::new(env.clone()).unwrap();
    let out: Vec<Test> = from_object(&dec, obj).expect("deserialisation failed");
    let enc = Encoder::new(env).unwrap();
    let output = to_object(&enc, &out).expect("serialisation failed!");
    output.into_inner()
}
