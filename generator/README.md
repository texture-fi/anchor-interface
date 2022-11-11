# Anchor program interface generator

Lightweight interface generator for anchor programs from JSON IDL.

## Usage

In a new crate or module, write:
```
#![doc = gen_crate_docs!()]

anchor_interface_gen::program!("idl.json");

solana_program::declare_id!("...");
```
> WARNING: also you will need to add all required dependencies to the manifest file

This will generate lightweight interface for anchor program
([anchor-interface](../interface/) based).

### Build instructions

`program!(..)` macro generate instruction builder with same name.

For example, for idl-instruction
```
...
{
  "name": "initializeState",
  "accounts": [
    {
      "name": "authority",
      "isMut": true,
      "isSigner": true
    },
    {
      "name": "someMint",
      "isMut": false,
      "isSigner": false
    },
    {
      "name": "systemProgram",
      "isMut": false,
      "isSigner": false
    },
    ...
  ],
  "args": [
    {
      "name": "x",
      "type": "u64"
    },
    {
      "name": "y",
      "type": "u64"
    },
    ...
  ]
},
...
```

The macros will generate builder struct
```
pub struct InitializeState {
    pub authority: Pubkey,
    pub some_mint: Pubkey,
    pub system_program: Pubkey,
    ...
    pub x: u64,
    pub y: u64,
    ...
}
```

For build this instruction, call `into_instruction()` method of builder struct
```
let mut transaction = Transaction::new_with_payer(
    &[some_program::instruction::InitializeState {
        authority,
        some_mint,
        system_program,
        ...
        x,
        y,
        ...
    }
    .into_instruction()],
    Some(payer.pubkey()),
);
```

For known constants (such as `system_prgram`), you can implement `Default` trait

```
impl Default for InitializeState {
    fn default() -> Self {
        Self {
            authority: Pubkey::new_from_array([255; 32]),
            some_mint: Pubkey::new_from_array([255; 32]),
            system_program: solana_program::system_program::id(),
            ...
            x: 0,
            y: 0,
            ...
        }
    }
}

...

let mut transaction = Transaction::new_with_payer(
    &[some_program::instruction::InitializeState {
        authority,
        some_mint,
        ...,
        x,
        y,
        ...,
        ..Default::default()
    }
    .into_instruction()],
    Some(payer.pubkey()),
);
```

### Serialize/Deserialize accounts

The macros will generate account structs with implemented
traits `AccountSerialize` and `AccountDeserialize` from [anchor-interface](../interface/) crate.
Use them ;).
