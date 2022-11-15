# Anchor program interface

Lightweight interface for anchor programs and tools for generating it from IDL.

Crate [anchor-interface](interface/) provides the necessary traits
to serialize/deserialize accounts, etc.

Crate [anchor-interface-gen](generator/) provides proc-macros
for generate [anchor-interface](interface/) based program interface
(instruction enum and builders, accounts, etc).

Helper crate [anchor-interface-syn](syn/) provides generators implementation
for use in proc-macros.

For more info, see the readme in the right crate.