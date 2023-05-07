fn gen_producer_source(pipes: u64) -> String {
    let header = r#"
    __kernel void producer(
        __global int* input,"#;

    // let body_pipe_args = r#"
    //     __write_only pipe int p0,
    //     __write_only pipe int p1,
    //     __write_only pipe int p2,
    //     __write_only pipe int p3,
    //     __write_only pipe int p4"#;

    // let body_pipes = r#"
    //     switch (i) {
    //         case 0:
    //           write_pipe(p0, &input[i]);
    //           break;
    //         case 1:
    //           write_pipe(p1, &input[i]);
    //           break;
    //         case 2:
    //           write_pipe(p2, &input[i]);
    //           break;
    //         case 3:
    //           write_pipe(p3, &input[i]);
    //           break;
    //         case 4:
    //           write_pipe(p4, &input[i]);
    //           break;
    //     }
    //     "#;

    let mut body_pipe_args = String::from(
        "
        ",
    );

    let mut body_pipes = String::from(
        "
        switch (i) {
        ",
    );

    for i in 0..pipes {
        let v = format!(
            "
            __write_only pipe int p{i},"
        );
        body_pipe_args.push_str(&v);
        let v = format!(
            "
            case {i}:
              write_pipe(p{i}, &input[i]);
              break;
            "
        );
        body_pipes.push_str(&v);
    }

    body_pipe_args.pop(); // remove last ( , ) example: "int p0, int p1"
    body_pipes.push_str(
        "
        }
    ",
    );

    let body_get_global_id = r#"
        ) {

        // Get the index of the current element
        int i = get_global_id(0);
        // printf("global_id %d %d",  i, input[i]);
        "#;

    let body = format!("{body_pipe_args}{body_get_global_id}{body_pipes}");

    let end = r#"
    }"#;

    format!("{header}{body}{end}")
}

fn gen_consumer_source(pipes: u64) -> String {
    // const CONSUMER_PROGRAM_SOURCE: &str = r#"
    // __kernel void consumer(
    //     __global int* output,
    //     __read_only pipe int p0,
    //     __read_only pipe int p1,
    //     __read_only pipe int p2,
    //     __read_only pipe int p3,
    //     __read_only pipe int p4
    //     ) {
    //
    //     // Get the index of the current element
    //     int i = get_global_id(0);
    //     // printf("global_id %d",  i);
    //
    //     output[i] = -1;
    //
    //     switch (i) {
    //         case 0:
    //           read_pipe(p0, &output[i]);
    //           break;
    //         case 1:
    //           read_pipe(p1, &output[i]);
    //           break;
    //         case 2:
    //           read_pipe(p2, &output[i]);
    //           break;
    //         case 3:
    //           read_pipe(p3, &output[i]);
    //           break;
    //         case 4:
    //           read_pipe(p4, &output[i]);
    //           break;
    //     }
    // }"#;

    let header = r#"
    __kernel void consumer(
        __global int* output,"#;

    let mut body_pipe_args = String::from(
        "
        ",
    );

    let mut body_pipes = String::from(
        "
        switch (i) {
        ",
    );

    for i in 0..pipes {
        let v = format!(
            "
            __read_only pipe int p{i},"
        );
        body_pipe_args.push_str(&v);
        let v = format!(
            "
            case {i}:
              read_pipe(p{i}, &output[i]);
              break;
            "
        );
        body_pipes.push_str(&v);
    }

    body_pipe_args.pop(); // remove last ( , ) example: "int p0, int p1"
    body_pipes.push_str(
        "
        }
    ",
    );

    let body_get_global_id = r#"
        ) {

        // Get the index of the current element
        int i = get_global_id(0);
        // printf("global_id %d %d",  i, input[i]);

        output[i] = -1;
        "#;

    let body = format!("{body_pipe_args}{body_get_global_id}{body_pipes}");

    let end = r#"
    }"#;

    format!("{header}{body}{end}")
}

fn main() {
    // const SOURCE: &str = r#"
    // __kernel void producer(
    //     __global int* input,
    //     __write_only pipe int p0,
    //     __write_only pipe int p1,
    //     __write_only pipe int p2,
    //     __write_only pipe int p3,
    //     __write_only pipe int p4
    //     ) {
    //
    //     // Get the index of the current element
    //     int i = get_global_id(0);
    //     // printf("global_id %d %d",  i, input[i]);
    //
    //     switch (i) {
    //         case 0:
    //           write_pipe(p0, &input[i]);
    //           break;
    //         case 1:
    //           write_pipe(p1, &input[i]);
    //           break;
    //         case 2:
    //           write_pipe(p2, &input[i]);
    //           break;
    //         case 3:
    //           write_pipe(p3, &input[i]);
    //           break;
    //         case 4:
    //           write_pipe(p4, &input[i]);
    //           break;
    //     }
    // }"#;
    //
    // println!("{SOURCE}");

    let source = gen_producer_source(5);
    println!("{source}");

    let source = gen_consumer_source(5);
    println!("{source}");
}
