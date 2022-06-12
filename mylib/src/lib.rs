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

use std::sync::{Arc, Mutex};

struct MapValue {
    value_pointer: *mut [u8],
}

impl MapValue {
    fn new(value_vec: Vec<u8>) -> Self {
        let boxed_slice = value_vec.into_boxed_slice();
        MapValue {
            value_pointer: Box::into_raw(boxed_slice),
        }
    }

    fn get_pointer(&self) -> *mut [u8] {
        self.value_pointer
    }
}

impl Drop for MapValue {
    fn drop(&mut self) {
        // println!("in MapValue.drop");
        let _box = unsafe { Box::from_raw(self.value_pointer) };
    }
}

struct Map {
    map: Mutex<HashMap<Vec<u8>, Arc<MapValue>>>,
}

impl Map {
    fn new() -> Self {
        Map {
            map: Mutex::new(HashMap::new()),
        }
    }

    fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.map
            .lock()
            .unwrap()
            .insert(key, Arc::new(MapValue::new(value)));
    }

    fn len(&self) -> usize {
        self.map.lock().unwrap().len()
    }

    fn get(&self, key: &Vec<u8>) -> Option<Arc<MapValue>> {
        return match self.map.lock().unwrap().get(key) {
            None => None,
            Some(value) => Some(Arc::clone(value)),
        };
    }

    fn remove(&mut self, key: &Vec<u8>) {
        self.map.lock().unwrap().remove(key);
    }
}

#[no_mangle]
pub extern "system" fn Java_RustMap_newMap(_env: JNIEnv, _class: JClass) -> jlong {
    Box::into_raw(Box::new(Map::new())) as jlong
}

#[no_mangle]
pub extern "system" fn Java_RustMap_deleteMap(_env: JNIEnv, _class: JClass, map_ptr: jlong) {
    let _boxed_counter = unsafe { Box::from_raw(map_ptr as *mut Map) };
}

#[no_mangle]
pub extern "system" fn Java_RustMap_putIntoMap(
    env: JNIEnv,
    _class: JClass,
    map_ptr: jlong,
    key_byte_array: jbyteArray,
    value_byte_array: jbyteArray,
) {
    let map = unsafe { &mut *(map_ptr as *mut Map) };

    let key = env.convert_byte_array(key_byte_array).unwrap();
    let value = env.convert_byte_array(value_byte_array).unwrap();

    map.insert(key, value);
}

#[no_mangle]
pub extern "system" fn Java_RustMap_mapSize(_env: JNIEnv, _class: JClass, map_ptr: jlong) -> jlong {
    let map = unsafe { &mut *(map_ptr as *mut Map) };

    return map.len() as jlong;
}

#[no_mangle]
pub extern "system" fn Java_RustMap_getFromMap(
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

            let value_pointer = value.get_pointer();

            let direct_buffer = env
                .new_direct_byte_buffer(unsafe { &mut *value_pointer })
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

#[no_mangle]
pub extern "system" fn Java_RustMap_deleteFromMap(
    env: JNIEnv,
    _class: JClass,
    map_ptr: jlong,
    key_byte_array: jbyteArray,
) {
    let map = unsafe { &mut *(map_ptr as *mut Map) };

    let key = env.convert_byte_array(key_byte_array).unwrap();

    map.remove(&key);
}
