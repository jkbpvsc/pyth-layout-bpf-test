use std::mem::{align_of_val, size_of, size_of_val};

use solana_nostd_entrypoint::{
    solana_program::{self, entrypoint::ProgramResult, pubkey::Pubkey},
    NoStdAccountInfo,
};
use type_layout::TypeLayout;

solana_nostd_entrypoint::entrypoint_nostd!(processor, 4);
solana_program::declare_id!("PYTHBPFTESTPYTHBPFTESTPYTHBPFTESTPYTHBPFTST");

solana_program::custom_heap_default!();
solana_program::custom_panic_default!();

pub mod pyth_09;
pub mod pyth_10;

use crate::pyth_09::PriceAccount as PriceAccountV0_9_0;
use crate::pyth_10::SolanaPriceAccount as PriceAccountV0_10_1;
use solana_program::{log, msg};

pub fn processor(
    _program_id: &Pubkey,
    _accounts: &[NoStdAccountInfo],
    _data: &[u8],
) -> ProgramResult {
    log::sol_log("start");

    msg!(
        "old size {} new size {}",
        size_of::<PriceAccountV0_9_0>(),
        size_of::<PriceAccountV0_10_1>()
    );

    // Initialize price accounts
    #[rustfmt::skip]
    let old = Box::new(old_gen());
    log::sol_log("declared old");

    msg!("Pyth 0.9 {}", PriceAccountV0_9_0::type_layout());
    msg!("Pyth 1.0 {}", PriceAccountV0_10_1::type_layout());

    #[rustfmt::skip]
    let new = Box::new(PriceAccountV0_10_1 {
        magic:          1,
        ver:            2,
        atype:          3,
        size:           4,
        ptype:          crate::pyth_10::PriceType::Price,
        expo:           5,
        num:            6,
        num_qt:         7,
        last_slot:      8,
        valid_slot:     9,
        ema_price:      crate::pyth_10::Rational {
            val:   10,
            numer: 11,
            denom: 12,
        },
        ema_conf:       crate::pyth_10::Rational {
            val:   13,
            numer: 14,
            denom: 15,
        },
        timestamp:      16,
        min_pub:        17,
        drv2:           18,
        drv3:           19,
        drv4:           20,
        prod:           Pubkey::new_from_array([204; 32]),
        next:           Pubkey::new_from_array([205; 32]),
        prev_slot:      21,
        prev_price:     22,
        prev_conf:      23,
        prev_timestamp: 24,
        agg:            crate::pyth_10::PriceInfo {
            price:    25,
            conf:     26,
            status:   crate::pyth_10::PriceStatus::Trading,
            corp_act: crate::pyth_10::CorpAction::NoCorpAct,
            pub_slot: 27,
        },
        comp:           [Default::default(); 32],
        extended:       (),
    });
    log::sol_log("declared new");

    // Equal Fields?
    macro_rules! check_field {
        ($field:ident) => {
            assert!(
                *$field == new.$field,
                // "field {} not equal",
                stringify!($field)
            );
        };
        ($field:ident, $new:expr) => {
            assert!(
                *$field == $new.$field,
                // "field {} not equal",
                stringify!($field)
            );
        };
    }
    macro_rules! check_field_offset {
        ($field:tt) => {{
            let (old_offset, new_offset) = offsets_of(&*old, &old.$field, &*new, &new.$field);
            assert_eq!(old_offset, new_offset, stringify!($field));
        }};
    }
    check_field_offset!(magic);
    check_field_offset!(ver);
    check_field_offset!(atype);
    check_field_offset!(size);
    check_field_offset!(expo);
    check_field_offset!(num);
    check_field_offset!(num_qt);
    check_field_offset!(last_slot);
    check_field_offset!(valid_slot);
    check_field_offset!(timestamp);
    check_field_offset!(min_pub);
    check_field_offset!(drv2);
    check_field_offset!(drv3);
    check_field_offset!(drv4);
    check_field_offset!(prod);
    check_field_offset!(next);
    check_field_offset!(prev_slot);
    check_field_offset!(prev_price);
    check_field_offset!(prev_conf);
    check_field_offset!(prev_timestamp);
    check_field_offset!(ptype);
    check_field_offset!(ema_price);
    check_field_offset!(ema_conf);
    check_field_offset!(agg);
    check_field_offset!(comp);

    let &PriceAccountV0_9_0 {
        ref magic,
        ref ver,
        ref atype,
        ref size,
        ref ptype,
        ref expo,
        ref num,
        ref num_qt,
        ref last_slot,
        ref valid_slot,
        ref ema_price,
        ref ema_conf,
        ref timestamp,
        ref min_pub,
        ref drv2,
        ref drv3,
        ref drv4,
        ref prod,
        ref next,
        ref prev_slot,
        ref prev_price,
        ref prev_conf,
        ref prev_timestamp,
        ref agg,
        ref comp,
    } = bytemuck::from_bytes(bytemuck::bytes_of(&*new));

    // } = unsafe { core::mem::transmute(&*new) };
    #[rustfmt::skip]
    let _ = {
        check_field!(magic); 
        check_field!(ver); 
        check_field!(atype); 
        check_field!(size); 
        check_field!(expo); 
        check_field!(num); 
        check_field!(num_qt); 
        check_field!(last_slot); 
        check_field!(valid_slot); 
        check_field!(timestamp); 
        check_field!(min_pub); 
        check_field!(drv2); 
        check_field!(drv3); 
        check_field!(drv4); 
        check_field!(prod); 
        check_field!(next); 
        check_field!(prev_slot); 
        check_field!(prev_price); 
        check_field!(prev_conf); 
        check_field!(prev_timestamp);  
    };
    log::sol_log("checked fields part 1");

    // The following require special handling
    // check_field!(ptype);
    // check_field!(ema_price);
    // check_field!(ema_conf);
    // check_field!(agg);
    // check_field!(comp);

    assert_eq!(*ptype as i32, new.ptype as u8 as i32);

    // ema
    let crate::pyth_09::Rational { val, numer, denom } = ema_price;
    {
        check_field!(val, new.ema_price);
        check_field!(numer, new.ema_price);
        check_field!(denom, new.ema_price);
    }

    // ema_conf
    let crate::pyth_09::Rational { val, numer, denom } = ema_conf;
    {
        check_field!(val, new.ema_conf);
        check_field!(numer, new.ema_conf);
        check_field!(denom, new.ema_conf);
    }

    // agg
    let crate::pyth_09::PriceInfo {
        price,
        conf,
        status,
        corp_act,
        pub_slot,
    } = agg;
    {
        check_field!(price, new.agg);
        check_field!(conf, new.agg);
        check_field!(pub_slot, new.agg);

        assert_eq!(*status as i32, new.agg.status as u8 as i32);
        assert_eq!(*corp_act as i32, new.agg.corp_act as u8 as i32);
    }

    // comp
    for (c, new_c) in comp.iter().zip(new.comp) {
        let crate::pyth_09::PriceComp {
            publisher,
            agg,
            latest,
        } = c;
        check_field!(publisher, new_c);

        // agg
        let crate::pyth_09::PriceInfo {
            price,
            conf,
            status,
            corp_act,
            pub_slot,
        } = agg;
        {
            check_field!(price, new_c.agg);
            check_field!(conf, new_c.agg);
            check_field!(pub_slot, new_c.agg);

            assert_eq!(*status as i32, new_c.agg.status as u8 as i32);
            assert_eq!(*corp_act as i32, new_c.agg.corp_act as u8 as i32);
        }

        // agg
        let crate::pyth_09::PriceInfo {
            price,
            conf,
            status,
            corp_act,
            pub_slot,
        } = latest;
        {
            check_field!(price, new_c.latest);
            check_field!(conf, new_c.latest);
            check_field!(pub_slot, new_c.latest);

            assert_eq!(*status as i32, new_c.latest.status as u8 as i32);
            assert_eq!(*corp_act as i32, new_c.latest.corp_act as u8 as i32);
        }
    }

    // Equal Byte Representation?
    assert!(bytemuck::bytes_of(&*old) == bytemuck::bytes_of(&*new));
    log::sol_log("checked binary representation");

    // equal size, align
    assert_eq!(
        core::mem::size_of::<PriceAccountV0_9_0>(),
        core::mem::size_of::<PriceAccountV0_10_1>(),
    );
    assert_eq!(
        core::mem::align_of::<PriceAccountV0_9_0>(),
        core::mem::align_of::<PriceAccountV0_10_1>(),
    );

    Ok(())
}

#[inline(never)]
fn old_gen() -> PriceAccountV0_9_0 {
    PriceAccountV0_9_0 {
        magic: 1,
        ver: 2,
        atype: 3,

        size: 4,
        ptype: crate::pyth_09::PriceType::Price,
        expo: 5,
        num: 6,
        num_qt: 7,
        last_slot: 8,
        valid_slot: 9,
        ema_price: crate::pyth_09::Rational {
            val: 10,
            numer: 11,
            denom: 12,
        },
        ema_conf: crate::pyth_09::Rational {
            val: 13,
            numer: 14,
            denom: 15,
        },
        timestamp: 16,
        min_pub: 17,
        drv2: 18,
        drv3: 19,
        drv4: 20,
        prod: Pubkey::new_from_array([204; 32]),
        next: Pubkey::new_from_array([205; 32]),
        prev_slot: 21,
        prev_price: 22,
        prev_conf: 23,
        prev_timestamp: 24,
        agg: crate::pyth_09::PriceInfo {
            price: 25,
            conf: 26,
            status: crate::pyth_09::PriceStatus::Trading,
            corp_act: crate::pyth_09::CorpAction::NoCorpAct,
            pub_slot: 27,
        },
        comp: [Default::default(); 32],
    }
}

pub fn offsets_of<Old, New, OldField, NewField>(
    old: &Old,
    old_field: &OldField,
    new: &New,
    new_field: &NewField,
) -> (usize, usize) {
    let old_addr: usize = old as *const Old as usize;
    let old_field_addr: usize = old_field as *const OldField as usize;
    let old_field_offset = old_field_addr - old_addr;
    msg!(
        "old addr {} old field addr {} offset {} size {} align {}",
        old_addr,
        old_field_addr,
        old_field_offset,
        size_of_val(old_field),
        align_of_val(old_field)
    );

    let new_addr: usize = new as *const New as usize;
    let new_field_addr: usize = new_field as *const NewField as usize;
    let new_field_offset = new_field_addr - new_addr;
    msg!(
        "new addr {} new field addr {} offset {} size {} align {}",
        new_addr,
        new_field_addr,
        new_field_offset,
        size_of_val(new_field),
        align_of_val(new_field)
    );

    (old_field_offset, new_field_offset)
}
