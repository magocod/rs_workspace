use std::collections::HashMap;
use crate::ocl_v6::{BlockConfigMap, GlobalArrayBlockConfig, KB_1, MB_1};

pub fn config_a() -> BlockConfigMap {
    let mut m = HashMap::new();
    m.insert(
        "1kb",
        GlobalArrayBlockConfig {
            global_array_capacity: KB_1,
            global_array_count: 2400,
            negative_initial_value: false,
        },
    );
    m.insert(
        "2kb",
        GlobalArrayBlockConfig {
            global_array_capacity: KB_1 * 2,
            global_array_count: 1200,
            negative_initial_value: false,
        },
    );
    m.insert(
        "4kb",
        GlobalArrayBlockConfig {
            global_array_capacity: KB_1 * 4,
            global_array_count: 600,
            negative_initial_value: false,
        },
    );
    m.insert(
        "6kb",
        GlobalArrayBlockConfig {
            global_array_capacity: KB_1 * 6,
            global_array_count: 200,
            negative_initial_value: false,
        },
    );
    m.insert(
        "8kb",
        GlobalArrayBlockConfig {
            global_array_capacity: KB_1 * 8,
            global_array_count: 200,
            negative_initial_value: false,
        },
    );
    m.insert(
        "128_kb",
        GlobalArrayBlockConfig {
            global_array_capacity: KB_1 * 128,
            global_array_count: 500,
            negative_initial_value: false,
        },
    );
    m.insert(
        "512_kb",
        GlobalArrayBlockConfig {
            global_array_capacity: KB_1 * 512,
            global_array_count: 200,
            negative_initial_value: false,
        },
    );
    m.insert(
        "1mb",
        GlobalArrayBlockConfig {
            global_array_capacity: MB_1,
            global_array_count: 10,
            negative_initial_value: false,
        },
    );
    m.insert(
        "2mb",
        GlobalArrayBlockConfig {
            global_array_capacity: MB_1 * 2,
            global_array_count: 10,
            negative_initial_value: false,
        },
    );
    m.insert(
        "9mb",
        GlobalArrayBlockConfig {
            global_array_capacity: MB_1 * 9,
            global_array_count: 10,
            negative_initial_value: false,
        },
    );
    m
}
