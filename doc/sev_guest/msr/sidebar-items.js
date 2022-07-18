initSidebarItems({"constant":[["NO_PREFERRED_GHCB_LOCATION","Value indicating that the hypervisor does not have a preferred location for the GHCB."],["SUPPORTED_PROTOCOL_VERION","The version of the GHCB MSR protocol supported by this library. This represents the version specific to AMD SEV-SNP."]],"enum":[["CpuidRegister","The register of interest from the result of executing CPUID."]],"fn":[["get_cpuid","Gets the value of the specified register that was returned when executing CPUID for the specified leaf. Sub-leafs are not supported."],["get_preferred_ghcb_location","Requests the hypervisor’s preferred location for the GHCB page."],["get_sev_info","Gets information about the supported GHCB MSR protocol verions and the location of the encryption bit."],["read_msr","Reads the value of the model-specific register."],["register_ghcb_location","Registers the location of the GHCB page for the current vCPU with the hypervisor."],["set_ghcb_address_and_exit","Sets the address of the GHCB page before exiting to the hypervisor."],["write_msr_and_exit","Writes a value to the MSR and calls VMGEXIT to hand control to the hypervisor."]],"struct":[["CpuidRequest","A request to execute CPUID for a specific leaf and return one of the result registers."],["CpuidResponse","A response from executing CPUID for a specific leaf. Only one register is returned at a time."],["GhcbGpa","Contains the guest-physical address of the GHCB page. The address must have been registered with the hypervisor before using it."],["PreferredGhcbGpaRequest","A request for the hypervisor’s preferred location for the GHCB page."],["PreferredGhcbGpaResponse","The response containing the preferred location of the GHCB."],["RegisterGhcbGpaRequest","Request to register a guest-physical address for the GHCB with the hypervisor."],["RegisterGhcbGpaResponse","The response containing the result of the GHCB registration."],["SevInfoRequest","A request for information about the supported GHCB MSR protocol version and the encryption bit."],["SevInfoResponse","Response from the hypervisor about the encryption bit and supported GHCB protocol versions. The encryption bit value is validated by the hardware before the guest is resumed."]]});