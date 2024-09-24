#![no_std]

use bern_units::memory_size::Byte;
use bern_conf_type::*;

pub const CONF: Conf<0> = Conf {
    kernel: Kernel {
        priorities: 8,
        memory_size: Byte::from_kB(1),
    },

    shared: Shared {
        size: Byte::from_kB(10),
    },

    memory_map: MemoryMap {
        flash: Memory {
            link_name: "FLASH",
            start_address: 0x0800_0000,
            size: Byte::from_MB(1),
        },
        sram: Memory {
            link_name: "RAM",
            start_address: 0x2000_0000,
            size: Byte::from_kB(128),
        },
        peripheral: Memory {
            link_name: "",
            start_address: 0x4000_0000,
            size: Byte::from_MB(512),
        },
        additional: [],
    },

    data_placement: DataPlacement {
        kernel: "RAM",
        processes: "RAM",
        shared: "RAM"
    }
};