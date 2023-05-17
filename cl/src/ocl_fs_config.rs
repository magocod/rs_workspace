use crate::ocl_v6::{BlockConfigMap, GlobalArrayBlockConfig, KB_1, MB_1};
use std::collections::HashMap;

pub fn config_a() -> BlockConfigMap {
    let mut m = HashMap::new();
    m.insert(
        "1kb",
        GlobalArrayBlockConfig {
            global_array_capacity: KB_1,
            global_array_count: 9000,
            negative_initial_value: false,
        },
    );
    m.insert(
        "2kb",
        GlobalArrayBlockConfig {
            global_array_capacity: KB_1 * 2,
            global_array_count: 4000,
            negative_initial_value: false,
        },
    );
    m.insert(
        "4kb",
        GlobalArrayBlockConfig {
            global_array_capacity: KB_1 * 4,
            global_array_count: 1800,
            negative_initial_value: false,
        },
    );
    m.insert(
        "6kb",
        GlobalArrayBlockConfig {
            global_array_capacity: KB_1 * 6,
            global_array_count: 1000,
            negative_initial_value: false,
        },
    );
    m.insert(
        "8kb",
        GlobalArrayBlockConfig {
            global_array_capacity: KB_1 * 8,
            global_array_count: 500,
            negative_initial_value: false,
        },
    );
    m.insert(
        "128_kb",
        GlobalArrayBlockConfig {
            global_array_capacity: KB_1 * 128,
            global_array_count: 1500,
            negative_initial_value: false,
        },
    );
    m.insert(
        "512_kb",
        GlobalArrayBlockConfig {
            global_array_capacity: KB_1 * 512,
            global_array_count: 300,
            negative_initial_value: false,
        },
    );
    m.insert(
        "1mb",
        GlobalArrayBlockConfig {
            global_array_capacity: MB_1,
            global_array_count: 32,
            negative_initial_value: false,
        },
    );
    m.insert(
        "2mb",
        GlobalArrayBlockConfig {
            global_array_capacity: MB_1 * 2,
            global_array_count: 32,
            negative_initial_value: false,
        },
    );
    // m.insert(
    //     "8mb",
    //     GlobalArrayBlockConfig {
    //         global_array_capacity: MB_1 * 2,
    //         global_array_count: 16,
    //         negative_initial_value: false,
    //     },
    // );
    m.insert(
        "9mb",
        GlobalArrayBlockConfig {
            global_array_capacity: MB_1 * 9,
            global_array_count: 16,
            negative_initial_value: false,
        },
    );
    m
}

pub fn config_b() -> BlockConfigMap {
    let mut m = HashMap::new();
    m.insert(
        "50mb",
        GlobalArrayBlockConfig {
            global_array_capacity: MB_1 * 50,
            global_array_count: 1,
            negative_initial_value: false,
        },
    );
    m
}
