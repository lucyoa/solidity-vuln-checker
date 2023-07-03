# Solidity Vuln Checker

Iterates over known vulns in solidity compiler.

### Update
```
cargo run update
Updating...
[*] Downloading to db/bugs_by_version.json
[*] Downloading to db/bugs.json
```

### Vuln Check
```
cargo run version 0.8.12
Results...

# SOL-2022-6 - AbiReencodingHeadOverflowWithStaticArrayCleanup
### Severity: medium
### Versions: 0.5.8 - 0.8.16
### Summary
ABI-encoding a tuple with a statically-sized calldata array in the last component would corrupt 32 leading bytes of its first dynamically encoded component.
### Description
When ABI-encoding a statically-sized calldata array, the compiler always pads the data area to a multiple of 32-bytes and ensures that the padding bytes are zeroed. In some cases, this cleanup used to be performed by always writing exactly 32 bytes, regardless of how many needed to be zeroed. This was done with the assumption that the data that would eventually occupy the area past the end of the array had not yet been written, because the encoder processes tuple components in the order they were given. While this assumption is mostly true, there is an important corner case: dynamically encoded tuple components are stored separately from the statically-sized ones in an area called the *tail* of the encoding and the tail immediately follows the *head*, which is where the statically-sized components are placed. The aforementioned cleanup, if performed for the last component of the head would cross into the tail and overwrite up to 32 bytes of the first component stored there with zeros. The only array type for which the cleanup could actually result in an overwrite were arrays with ``uint256`` or ``bytes32`` as the base element type and in this case the size of the corrupted area was always exactly 32 bytes. The problem affected tuples at any nesting level. This included also structs, which are encoded as tuples in the ABI. Note also that lists of parameters and return values of functions, events and errors are encoded as tuples
(..)
```
