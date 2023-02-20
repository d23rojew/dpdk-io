use std::{env, path::PathBuf};

fn main() {
    let bindings = bindgen::Builder::default()
        .header("c/include/wraper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    println!("cargo:rustc-link-search=native=/lib");
    println!("cargo:rustc-link-search=native=/lib64");
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-search=native=/usr/lib64");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-search=native=/usr/lib/gcc/x86_64-linux-gnu");
    println!("cargo:rustc-link-search=native=/usr/local/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-arg=-Wl,--whole-archive,-l:librte_common_cpt.a,-l:librte_common_dpaax.a,-l:librte_common_iavf.a,-l:librte_common_octeontx.a,-l:librte_common_octeontx2.a,-l:librte_bus_auxiliary.a,-l:librte_bus_dpaa.a,-l:librte_bus_fslmc.a,-l:librte_bus_ifpga.a,-l:librte_bus_pci.a,-l:librte_bus_vdev.a,-l:librte_bus_vmbus.a,-l:librte_common_cnxk.a,-l:librte_common_qat.a,-l:librte_common_sfc_efx.a,-l:librte_mempool_bucket.a,-l:librte_mempool_cnxk.a,-l:librte_mempool_dpaa.a,-l:librte_mempool_dpaa2.a,-l:librte_mempool_octeontx.a,-l:librte_mempool_octeontx2.a,-l:librte_mempool_ring.a,-l:librte_mempool_stack.a,-l:librte_dma_cnxk.a,-l:librte_dma_dpaa.a,-l:librte_dma_hisilicon.a,-l:librte_dma_idxd.a,-l:librte_dma_ioat.a,-l:librte_dma_skeleton.a,-l:librte_net_af_packet.a,-l:librte_net_ark.a,-l:librte_net_atlantic.a,-l:librte_net_avp.a,-l:librte_net_axgbe.a,-l:librte_net_bnx2x.a,-l:librte_net_bnxt.a,-l:librte_net_bond.a,-l:librte_net_cnxk.a,-l:librte_net_cxgbe.a,-l:librte_net_dpaa.a,-l:librte_net_dpaa2.a,-l:librte_net_e1000.a,-l:librte_net_ena.a,-l:librte_net_enetc.a,-l:librte_net_enetfec.a,-l:librte_net_enic.a,-l:librte_net_failsafe.a,-l:librte_net_fm10k.a,-l:librte_net_hinic.a,-l:librte_net_hns3.a,-l:librte_net_i40e.a,-l:librte_net_iavf.a,-l:librte_net_ice.a,-l:librte_net_igc.a,-l:librte_net_ionic.a,-l:librte_net_ixgbe.a,-l:librte_net_kni.a,-l:librte_net_liquidio.a,-l:librte_net_memif.a,-l:librte_net_netvsc.a,-l:librte_net_nfp.a,-l:librte_net_ngbe.a,-l:librte_net_null.a,-l:librte_net_octeontx.a,-l:librte_net_octeontx2.a,-l:librte_net_octeontx_ep.a,-l:librte_net_pfe.a,-l:librte_net_qede.a,-l:librte_net_ring.a,-l:librte_net_sfc.a,-l:librte_net_softnic.a,-l:librte_net_tap.a,-l:librte_net_thunderx.a,-l:librte_net_txgbe.a,-l:librte_net_vdev_netvsc.a,-l:librte_net_vhost.a,-l:librte_net_virtio.a,-l:librte_net_vmxnet3.a,-l:librte_raw_cnxk_bphy.a,-l:librte_raw_dpaa2_cmdif.a,-l:librte_raw_dpaa2_qdma.a,-l:librte_raw_ntb.a,-l:librte_raw_skeleton.a,-l:librte_crypto_bcmfs.a,-l:librte_crypto_caam_jr.a,-l:librte_crypto_ccp.a,-l:librte_crypto_cnxk.a,-l:librte_crypto_dpaa_sec.a,-l:librte_crypto_dpaa2_sec.a,-l:librte_crypto_nitrox.a,-l:librte_crypto_null.a,-l:librte_crypto_octeontx.a,-l:librte_crypto_octeontx2.a,-l:librte_crypto_openssl.a,-l:librte_crypto_scheduler.a,-l:librte_crypto_virtio.a,-l:librte_compress_octeontx.a,-l:librte_compress_zlib.a,-l:librte_regex_octeontx2.a,-l:librte_vdpa_ifc.a,-l:librte_vdpa_sfc.a,-l:librte_event_cnxk.a,-l:librte_event_dlb2.a,-l:librte_event_dpaa.a,-l:librte_event_dpaa2.a,-l:librte_event_dsw.a,-l:librte_event_octeontx2.a,-l:librte_event_opdl.a,-l:librte_event_skeleton.a,-l:librte_event_sw.a,-l:librte_event_octeontx.a,-l:librte_baseband_acc100.a,-l:librte_baseband_fpga_5gnr_fec.a,-l:librte_baseband_fpga_lte_fec.a,-l:librte_baseband_la12xx.a,-l:librte_baseband_null.a,-l:librte_baseband_turbo_sw.a,-l:librte_node.a,-l:librte_graph.a,-l:librte_flow_classify.a,-l:librte_pipeline.a,-l:librte_table.a,-l:librte_pdump.a,-l:librte_port.a,-l:librte_fib.a,-l:librte_ipsec.a,-l:librte_vhost.a,-l:librte_stack.a,-l:librte_security.a,-l:librte_sched.a,-l:librte_reorder.a,-l:librte_rib.a,-l:librte_dmadev.a,-l:librte_regexdev.a,-l:librte_rawdev.a,-l:librte_power.a,-l:librte_pcapng.a,-l:librte_member.a,-l:librte_lpm.a,-l:librte_latencystats.a,-l:librte_kni.a,-l:librte_jobstats.a,-l:librte_ip_frag.a,-l:librte_gso.a,-l:librte_gro.a,-l:librte_gpudev.a,-l:librte_eventdev.a,-l:librte_efd.a,-l:librte_distributor.a,-l:librte_cryptodev.a,-l:librte_compressdev.a,-l:librte_cfgfile.a,-l:librte_bpf.a,-l:librte_bitratestats.a,-l:librte_bbdev.a,-l:librte_acl.a,-l:librte_timer.a,-l:librte_hash.a,-l:librte_metrics.a,-l:librte_cmdline.a,-l:librte_pci.a,-l:librte_ethdev.a,-l:librte_meter.a,-l:librte_net.a,-l:librte_mbuf.a,-l:librte_mempool.a,-l:librte_rcu.a,-l:librte_ring.a,-l:librte_eal.a,-l:librte_telemetry.a,-l:librte_kvargs.a");
    println!("cargo:rustc-link-arg=-Wl,--no-whole-archive,--export-dynamic,-latomic,-lcrypto,-ldl,-lpthread,-lz");
    println!("cargo:rustc-link-arg=-Wl,--as-needed,-lc,-lrte_node,-lrte_graph,-lrte_flow_classify,-lrte_pipeline,-lrte_table,-lrte_pdump,-lrte_port,-lrte_fib,-lrte_ipsec,-lrte_vhost,-lrte_stack,-lrte_security,-lrte_sched,-lrte_reorder,-lrte_rib,-lrte_dmadev,-lrte_regexdev,-lrte_rawdev,-lrte_power,-lrte_pcapng,-lrte_member,-lrte_lpm,-lrte_latencystats,-lrte_kni,-lrte_jobstats,-lrte_ip_frag,-lrte_gso,-lrte_gro,-lrte_gpudev,-lrte_eventdev,-lrte_efd,-lrte_distributor,-lrte_cryptodev,-lrte_compressdev,-lrte_cfgfile,-lrte_bpf,-lrte_bitratestats,-lrte_bbdev,-lrte_acl,-lrte_timer,-lrte_hash,-lrte_metrics,-lrte_cmdline,-lrte_pci,-lrte_ethdev,-lrte_meter,-lrte_net,-lrte_mbuf,-lrte_mempool,-lrte_rcu,-lrte_ring,-lrte_eal,-lrte_telemetry,-lrte_kvargs,-lpthread,-lm,-ldl,-lnuma");
    println!("cargo:rustc-link-arg=-Wl,--whole-archive,-lfstack,--no-whole-archive");
    println!("cargo:rustc-link-arg=-Wl,--no-whole-archive,-lrt,-lm,-ldl,-lcrypto,-lpthread,-lnuma");
    pkg_config::Config::new()
        .probe("libdpdk")
        .expect("dpdk lib");
    println!("cargo:rustc-link-lib=c");
    println!("cargo:rustc-link-lib=rt");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=crypto");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=numa");

    // println!("cargo:rerun-if-changed=include_wraper.h");

    // panic!("dpdk. libs");
}
