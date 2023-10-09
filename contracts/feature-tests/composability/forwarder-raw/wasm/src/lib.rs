// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           25
// Async Callback:                       1
// Total number of exported functions:  27

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    forwarder_raw
    (
        init => init
        callback_args => callback_args
        callback_payments => callback_payments
        callback_payments_triples => callback_payments_triples
        clear_callback_info => clear_callback_info
        callback_args_at_index => callback_args_at_index
        callback_payment_at_index => callback_payment_at_index
        forward_payment => forward_payment
        forward_direct_dct_via_transf_exec => forward_direct_dct_via_transf_exec
        forward_direct_dct_multi => forward_direct_dct_multi
        forward_async_call => forward_async_call
        forward_async_call_half_payment => forward_async_call_half_payment
        forward_transf_exec_egld => forward_transf_exec_egld
        forward_transf_exec_dct => forward_transf_exec_dct
        forward_transf_exec => forward_transf_exec
        forward_transf_exec_twice => forward_transf_exec_twice
        forward_async_retrieve_multi_transfer_funds => forward_async_retrieve_multi_transfer_funds
        forwarder_async_send_and_retrieve_multi_transfer_funds => forwarder_async_send_and_retrieve_multi_transfer_funds
        call_execute_on_dest_context => call_execute_on_dest_context
        call_execute_on_dest_context_twice => call_execute_on_dest_context_twice
        call_execute_on_same_context => call_execute_on_same_context
        call_execute_on_dest_context_readonly => call_execute_on_dest_context_readonly
        deploy_contract => deploy_contract
        deploy_from_source => deploy_from_source
        call_upgrade => call_upgrade
        call_upgrade_from_source => call_upgrade_from_source
    )
}

dharitri_sc_wasm_adapter::async_callback! { forwarder_raw }
