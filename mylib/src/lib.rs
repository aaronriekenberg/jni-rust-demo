// This is the interface to the JVM that we'll
// call the majority of our methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native function.
// They carry extra lifetime information to prevent them escaping this context
// and getting used after being GC'd.
use jni::objects::{GlobalRef, JByteBuffer, JClass, JObject, JString};

// This is just a pointer. We'll be returning it from our function.
// We can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jbyteArray, jint, jlong, jobject, jstring};

use std::{sync::mpsc, thread, time::Duration};

use std::collections::HashMap;

use std::sync::Arc;

struct Map {
    map: HashMap<Vec<u8>, Arc<Vec<u8>>>,
}

impl Map {
    fn new() -> Self {
        Map {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: Vec<u8>, value: Arc<Vec<u8>>) {
        self.map.insert(key, value);
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn get(&self, key: &Vec<u8>) -> Option<Arc<Vec<u8>>> {
        return match self.map.get(key) {
            None => None,
            Some(value) => Some(Arc::clone(value)),
        };
    }
}

#[no_mangle]
pub extern "system" fn Java_HelloWorld_newMap(_env: JNIEnv, _class: JClass) -> jlong {
    Box::into_raw(Box::new(Map::new())) as jlong
}

#[no_mangle]
pub extern "system" fn Java_HelloWorld_deleteMap(_env: JNIEnv, _class: JClass, map_ptr: jlong) {
    let _boxed_counter = unsafe { Box::from_raw(map_ptr as *mut Map) };
}

#[no_mangle]
pub extern "system" fn Java_HelloWorld_putIntoMap(
    env: JNIEnv,
    _class: JClass,
    map_ptr: jlong,
    key_byte_array: jbyteArray,
    value_byte_array: jbyteArray,
) {
    let map = unsafe { &mut *(map_ptr as *mut Map) };

    let key = env.convert_byte_array(key_byte_array).unwrap();
    let value = Arc::new(env.convert_byte_array(value_byte_array).unwrap());

    map.insert(key, value);
}

#[no_mangle]
pub extern "system" fn Java_HelloWorld_mapSize(
    _env: JNIEnv,
    _class: JClass,
    map_ptr: jlong,
) -> jlong {
    let map = unsafe { &mut *(map_ptr as *mut Map) };

    return map.len() as jlong;
}

#[no_mangle]
pub  extern "system" fn Java_HelloWorld_getFromMap(
    env: JNIEnv,
    _class: JClass,
    map_ptr: jlong,
    key_byte_array: jbyteArray,
    map_getter: JObject,
) {
    let map = unsafe { &mut *(map_ptr as *mut Map) };

    let key = env.convert_byte_array(key_byte_array).unwrap();

    let value_option = map.get(&key);

    match value_option {
        None => {}
        Some(value) => {
            // We need to obtain global reference to the `map_getter` object before sending
            // it to the thread, to prevent it from being collected by the GC.
            let map_getter = env.new_global_ref(map_getter).unwrap();

            let mut value_clone = (*value).clone();

            let direct_buffer = env
                .new_direct_byte_buffer(value_clone.as_mut_slice())
                .unwrap();

            let _result = env.call_method(
                &map_getter,
                "read",
                "(Ljava/nio/ByteBuffer;)V",
                &[direct_buffer.into()],
            );
        }
    }
}
