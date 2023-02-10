window.SIDEBAR_ITEMS = {"fn":[["assert_response_body",""],["background","Executes the provided closure passing to it a [`Term`] instance signalling when to terminate, and spawns the resulting [`Future`] in the background, returning a [`Background`] instance."],["build_rust_crate_linux","Builds the crate identified by the given package name (as per the `name` attribute in a Cargo.toml file included in the root cargo workspace) as a Linux binary, and returns the path of the resulting binary."],["build_rust_crate_wasm","Builds the crate identified by the given package name (as per the `name` attribute in a Cargo.toml file included in the root cargo workspace) as a Wasm module, and returns the path of the resulting binary."],["compile_rust_wasm","Uses cargo to compile a Rust manifest to Wasm bytes."],["create_and_start_oak_functions_server","Starts an instance of the Oak Functions server running in the background, listening on the provided port, and running the provided Wasm module, with the provided data available for lookup."],["free_port",""],["kill_process","Kills all the processes identified by the provided handle."],["make_request",""],["serialize_entries","Serializes the provided map as a contiguous buffer of length-delimited protobuf messages of type `Entry`."],["write_to_temp_file",""]],"struct":[["Background","Wrapper around a termination signal [`oneshot::Sender`] and the [`JoinHandle`] of the associated background task, created by [`background`]."],["Term","A wrapper around a termination signal [`oneshot::Receiver`]."]]};