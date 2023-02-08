var sourcesIndex = JSON.parse('{\
"benchmark":["",[],["lib.rs"]],\
"echo":["",[],["lib.rs"]],\
"key_value_lookup":["",[],["lib.rs"]],\
"location_utils":["",[],["lib.rs"]],\
"lookup_data_checker":["",[],["main.rs"]],\
"lookup_data_generator":["",[],["data.rs","lib.rs"]],\
"micro_rpc":["",[],["lib.rs","status.rs"]],\
"micro_rpc_build":["",[],["lib.rs"]],\
"oak_channel":["",[],["client.rs","frame.rs","lib.rs","message.rs","server.rs"]],\
"oak_core":["",[],["lib.rs","sync.rs"]],\
"oak_echo_service":["",[],["lib.rs"]],\
"oak_enclave_runtime_support":["",[],["heap.rs","lib.rs","libm.rs"]],\
"oak_functions_abi":["",[],["lib.rs"]],\
"oak_functions_client":["",[],["lib.rs","rekor.rs"]],\
"oak_functions_extension":["",[],["lib.rs"]],\
"oak_functions_launcher":["",[["instance",[],["mod.rs","native.rs","virtualized.rs"]]],["channel.rs","lib.rs","lookup.rs","server.rs"]],\
"oak_functions_linux_fd_bin":["",[],["main.rs"]],\
"oak_functions_linux_vsock_bin":["",[],["channel.rs","main.rs"]],\
"oak_functions_load_test":["",[],["main.rs"]],\
"oak_functions_lookup":["",[],["lib.rs"]],\
"oak_functions_sdk":["",[],["lib.rs"]],\
"oak_functions_sdk_abi_test_get_storage_item":["",[],["lib.rs"]],\
"oak_functions_sdk_abi_test_invoke_testing":["",[],["lib.rs"]],\
"oak_functions_service":["",[["wasm",[],["mod.rs"]]],["lib.rs","logger.rs","remote_attestation.rs"]],\
"oak_functions_test_utils":["",[],["lib.rs"]],\
"oak_functions_testing_extension":["",[],["lib.rs"]],\
"oak_functions_wasm":["",[],["lib.rs"]],\
"oak_functions_web_client":["",[],["grpc_web.rs","lib.rs"]],\
"oak_functions_workload_logging":["",[],["lib.rs"]],\
"oak_grpc_utils":["",[],["lib.rs"]],\
"oak_hello_world_linux_init":["",[],["init.rs","main.rs"]],\
"oak_linux_boot_params":["",[],["lib.rs"]],\
"oak_logger":["",[],["lib.rs"]],\
"oak_remote_attestation_amd":["",[],["lib.rs"]],\
"oak_remote_attestation_interactive":["",[["crypto",[],["mod.rs","ring_crypto.rs"]]],["handshaker.rs","lib.rs","message.rs"]],\
"oak_remote_attestation_noninteractive":["",[],["client.rs","lib.rs"]],\
"oak_remote_attestation_sessions":["",[],["lib.rs"]],\
"oak_remote_attestation_sessions_client":["",[],["lib.rs"]],\
"oak_restricted_kernel":["",[["boot",[],["mod.rs"]],["mm",[],["bitmap_frame_allocator.rs","encrypted_mapper.rs","frame_allocator.rs","mod.rs","page_tables.rs","virtual_address_allocator.rs"]],["syscall",[],["channel.rs","fd.rs","mmap.rs","mod.rs","process.rs","stdio.rs"]]],["acpi.rs","args.rs","attestation.rs","avx.rs","descriptors.rs","elf.rs","ghcb.rs","interrupts.rs","lib.rs","libm.rs","logging.rs","memory.rs","payload.rs","shutdown.rs","snp.rs","virtio.rs"]],\
"oak_restricted_kernel_api":["",[],["channel.rs","lib.rs","logging.rs","raw_syscall.rs","syscall.rs"]],\
"oak_restricted_kernel_interface":["",[],["errno.rs","lib.rs","syscalls.rs"]],\
"oak_sev_guest":["",[],["cpuid.rs","crypto.rs","ghcb.rs","guest.rs","instructions.rs","interrupts.rs","io.rs","lib.rs","msr.rs","secrets.rs","vmsa.rs"]],\
"oak_simple_io":["",[],["lib.rs"]],\
"oak_tensorflow_service":["",[],["lib.rs","tflite.rs"]],\
"oak_virtio":["",[["console",[],["mod.rs"]],["queue",[],["mod.rs","virtq.rs"]],["vsock",[["socket",[],["mod.rs"]]],["mod.rs","packet.rs"]]],["lib.rs"]],\
"offline_attestation_client":["",[],["main.rs"]],\
"offline_attestation_server":["",[],["main.rs"]],\
"offline_attestation_shared":["",[],["lib.rs"]],\
"quirk_echo_service":["",[],["lib.rs"]],\
"sev_serial":["",[],["lib.rs"]],\
"snp_measurement":["",[],["elf.rs","main.rs","page.rs","stage0.rs","vmsa.rs"]],\
"vsock_echo":["",[],["main.rs"]],\
"weather_lookup":["",[],["lib.rs"]],\
"xtask":["",[],["check_build_licenses.rs","check_license.rs","check_todo.rs","diffs.rs","examples.rs","files.rs","internal.rs","launcher.rs","main.rs"]]\
}');
createSourceSidebar();
