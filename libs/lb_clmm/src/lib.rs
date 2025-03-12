use anchor_lang::prelude::*;

anchor_gen::generate_cpi_interface!(
    idl_path = "../../idls/lb_clmm.json",
    zero_copy(
        PositionV2,
        Position,
        BinArray,
        LbPair,
        Bin,
        StaticParameters,
        VariableParameters,
        BinArrayBitmapExtension,
        ProtocolFee,
        RewardInfo,
        Observation,
        Oracle,
        FeeInfo,
        UserRewardInfo,
    )
);
declare_id!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo");
