var sourcesIndex = JSON.parse('{\
"benchmark":["",[],["lib.rs"]],\
"echo":["",[],["lib.rs"]],\
"key_value_lookup":["",[],["lib.rs"]],\
"location_utils":["",[],["lib.rs"]],\
"lookup_data_checker":["",[],["main.rs"]],\
"lookup_data_generator":["",[],["data.rs","lib.rs"]],\
"oak_channel":["",[],["client.rs","frame.rs","lib.rs","message.rs","server.rs"]],\
"oak_core":["",[],["lib.rs","sync.rs"]],\
"oak_echo_service":["",[],["lib.rs"]],\
"oak_functions_abi":["",[],["lib.rs"]],\
"oak_functions_client":["",[],["lib.rs","rekor.rs"]],\
"oak_functions_extension":["",[],["lib.rs"]],\
"oak_functions_freestanding":["",[["wasm",[],["mod.rs"]]],["lib.rs","logger.rs","remote_attestation.rs"]],\
"oak_functions_launcher":["",[["instance",[],["crosvm.rs","mod.rs","native.rs"]]],["channel.rs","lookup.rs","main.rs","server.rs"]],\
"oak_functions_linux_fd_bin":["",[],["main.rs"]],\
"oak_functions_linux_vsock_bin":["",[],["channel.rs","main.rs"]],\
"oak_functions_load_test":["",[],["main.rs"]],\
"oak_functions_lookup":["",[],["lib.rs"]],\
"oak_functions_sdk":["",[],["lib.rs"]],\
"oak_functions_sdk_abi_test_get_storage_item":["",[],["lib.rs"]],\
"oak_functions_sdk_abi_test_invoke_testing":["",[],["lib.rs"]],\
"oak_functions_test_utils":["",[],["lib.rs"]],\
"oak_functions_testing_extension":["",[],["lib.rs"]],\
"oak_functions_wasm":["",[],["lib.rs"]],\
"oak_functions_web_client":["",[],["grpc_web.rs","lib.rs"]],\
"oak_functions_workload_logging":["",[],["lib.rs"]],\
"oak_grpc_unary_attestation":["",[],["client.rs","lib.rs"]],\
"oak_grpc_utils":["",[],["lib.rs"]],\
"oak_idl":["",[],["lib.rs","status.rs"]],\
"oak_idl_build":["",[],["lib.rs"]],\
"oak_linux_boot_params":["",[],["lib.rs"]],\
"oak_logger":["",[],["lib.rs"]],\
"oak_remote_attestation":["",[["crypto",[],["mod.rs","ring_crypto.rs"]]],["handshaker.rs","lib.rs","message.rs"]],\
"oak_remote_attestation_amd":["",[],["lib.rs"]],\
"oak_remote_attestation_sessions":["",[],["lib.rs"]],\
"oak_remote_attestation_sessions_client":["",[],["lib.rs"]],\
"oak_restricted_kernel":["",[["boot",[],["mod.rs"]],["mm",[],["bitmap_frame_allocator.rs","encrypted_mapper.rs","frame_allocator.rs","mod.rs","page_tables.rs"]]],["args.rs","avx.rs","descriptors.rs","elf.rs","ghcb.rs","interrupts.rs","lib.rs","libm.rs","logging.rs","memory.rs","shutdown.rs","virtio.rs"]],\
"oak_simple_io":["",[],["lib.rs"]],\
"oak_tensorflow_service":["",[],["lib.rs","tflite.rs"]],\
"offline_attestation_client":["",[],["main.rs"]],\
"offline_attestation_server":["",[],["main.rs"]],\
"offline_attestation_shared":["",[],["lib.rs"]],\
"sev_guest":["",[],["cpuid.rs","ghcb.rs","guest.rs","instructions.rs","interrupts.rs","io.rs","lib.rs","msr.rs","secrets.rs"]],\
"sev_serial":["",[],["lib.rs"]],\
"trusted_shuffler":["",[],["lib.rs"]],\
"trusted_shuffler_backend":["",[],["main.rs"]],\
"trusted_shuffler_client":["",[],["main.rs"]],\
"trusted_shuffler_common":["",[],["lib.rs"]],\
"trusted_shuffler_server":["",[],["http.rs","main.rs"]],\
"virtio":["",[["console",[],["mod.rs"]],["queue",[],["mod.rs","virtq.rs"]],["vsock",[["socket",[],["mod.rs"]]],["mod.rs","packet.rs"]]],["lib.rs"]],\
"vsock_echo":["",[],["main.rs"]],\
"weather_lookup":["",[],["lib.rs"]],\
"xtask":["",[],["check_build_licenses.rs","check_license.rs","check_todo.rs","diffs.rs","examples.rs","files.rs","internal.rs","launcher.rs","main.rs"]]\
}');
createSourceSidebar();
