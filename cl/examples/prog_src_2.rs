fn gen_program_source(arrays: usize, capacity: usize) -> String {
    // const PROGRAM_SOURCE: &str = r#"
    // // __constant int mb_20 = 20 * kb_1 * kb_1; // 20971520
    //
    // __global int myNumbers0[20971520];
    // __global int myNumbers1[20971520];
    // __global int myNumbers2[20971520];
    // __global int myNumbers3[20971520];
    // __global int myNumbers4[20971520];
    //
    // kernel void vector_add(global int* A) {
    //
    //     // Get the index of the current element
    //     int i = get_global_id(0);
    //
    //     // Do the operation
    //     myNumbers0[i] = A[i];
    //     myNumbers1[i] = A[i] + 1;
    //     myNumbers2[i] = A[i] + 2;
    //     myNumbers3[i] = A[i] + 3;
    //     myNumbers4[i] = A[i] + 4;
    // }
    //
    // kernel void vector_extract(global int* C, global int* D) {
    //
    //     // Get the index of the current element
    //     int i = get_global_id(0);
    //
    //     // Do the operation
    //     // C[i] = myNumbers0[i];
    //
    //     switch (D[i]) {
    //         case 0:
    //           C[i] = myNumbers0[i];
    //           break;
    //         case 1:
    //           C[i] = myNumbers1[i];
    //           break;
    //         case 2:
    //           C[i] = myNumbers2[i];
    //           break;
    //         case 3:
    //           C[i] = myNumbers3[i];
    //           break;
    //         case 4:
    //           C[i] = myNumbers4[i];
    //           break;
    //     }
    // }
    // "#;

    let mut global_arrays = String::from(
        "
        ",
    );

    let mut vector_add_fn = String::from(
        "
    kernel void vector_add(global int* A, global int* D) {

        // Get the index of the current element
        int i = get_global_id(0);

        switch (D[i]) {
        ",
    );

    let mut vector_extract_fn = String::from(
        "
    kernel void vector_extract(global int* C, global int* D) {

        // Get the index of the current element
        int i = get_global_id(0);

        switch (D[i]) {
        ",
    );

    for i in 0..arrays {
        // global_array
        let global_arr = format!(
            "
    __global int myNumbers{i}[{capacity}];"
        );
        global_arrays.push_str(&global_arr);

        // vector_add
        // let v_add = format!("
        // myNumbers{i}[i] = A[i] + {i};"
        // );
        let v_add = format!(
            "
            case {i}:
              myNumbers{i}[i] = A[i];
              break;"
        );
        vector_add_fn.push_str(&v_add);

        // vector_extract
        let v_ext = format!(
            "
            case {i}:
              C[i] = myNumbers{i}[i];
              break;"
        );
        vector_extract_fn.push_str(&v_ext);
    }

    let end_switch = r#"
        }"#;

    let end_fn = r#"
    }"#;

    let body = format!(
        "
    {vector_add_fn}
    {end_switch}
    {end_fn}
    {vector_extract_fn}
    {end_switch}
    {end_fn}
    "
    );

    format!("{global_arrays}{body}")
}

fn main() {
    let source = gen_program_source(5, 20971520);
    println!("{source}");
}
