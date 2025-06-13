# Migration Guides

- [Stacks.js (\>=5.x.x) → (7.x.x)](#stacksjs-5xx--7xx)
  - [Breaking Changes](#breaking-changes)
  - [Reducing Wrapper Types](#reducing-wrapper-types)
  - [Clarity Version](#clarity-version)
  - [Stacks Network](#stacks-network)
    - [Impacts](#impacts)
  - [Clarity Representation](#clarity-representation)
  - [Post-conditions](#post-conditions)
  - [Fetch Methods](#fetch-methods)
  - [`serialize` methods](#serialize-methods)
  - [Asset Helper Methods](#asset-helper-methods)
  - [CLI](#cli)
  - [Triplesec](#triplesec)
  - [Advanced: WireType](#advanced-wiretype)
  - [Advanced: Signed BigInt](#advanced-signed-bigint)
  - [Advanced: Refactorings](#advanced-refactorings)
- [Stacks.js (\<=4.x.x) → (5.x.x)](#stacksjs-4xx--5xx)
  - [Breaking Changes](#breaking-changes-1)
    - [Buffer to Uint8Array](#buffer-to-uint8array)
    - [Message Signing Prefix](#message-signing-prefix)
- [blockstack.js → Stacks.js (1.x.x)](#blockstackjs--stacksjs-1xx)
  - [Using blockstack.js](#using-blockstackjs)
    - [Using Blockstack Connect](#using-blockstack-connect)
  - [Storage](#storage)
    - [Using blockstack.js](#using-blockstackjs-1)
    - [Using @stacks/storage](#using-stacksstorage)
  - [Encryption](#encryption)
    - [Using blockstack.js](#using-blockstackjs-2)
    - [Using @stacks/encryption or @stacks/auth](#using-stacksencryption-or-stacksauth)

## Stacks.js (&gt;=5.x.x) → (7.x.x)

### Breaking Changes

- The `@stacks/network` `new StacksNetwork()` objects were removed. Instead `@stacks/network` now exports the objects `STACKS_MAINNET`, `STACKS_TESNET`, and `STACKS_DEVNET`, which are static (and shouldn't be changed for most use-cases). [Read more...](#stacks-network)
- Contract Deploys now default to Clarity version 3. [Read more...](#clarity-version)
- Most `fetch` (aka networking) methods were renamed to indicate they send HTTP requests. The new methods are named `fetchXyz` and are compatible with the old `Xyz` interfaces. [Read more...](#fetch-methods)
- Reducing wrapper types, which create annoyances for the developer, rather than being able to use values directly. [Read more...](#reducing-wrapper-types)
- The `ClarityType` enum was replaced by a human-readable version. The previous (wire format compatible) enum is still available as `ClarityWireType`. [Read more...](#clarity-representation)
- The previous post-conditions types and `create..` methods were replaced with a human-readable representation. [Read more...](#post-conditions)
- `StacksTransaction.serialize` and other `serializeXyz` methods were changed to return `string` (hex-encoded) instead of `Uint8Array`. Compatible `serializeXzyBytes` methods were added to ease the migration. [Read more...](#serialize-methods)
- The `AssetInfo` type was renamed to `Asset` for accuracy. The `Asset` helper methods were also renamed to to remove the `Info` suffix. [Read more...](#asset-helper-methods)
- Remove legacy CLI methods. [Read more...](#cli)
- Disable legacy `triplesec` mnemonic encryption support. [Read more...](#triplesec)
- **Advanced:** Rename `MessageType` and related concepts to `WireType`. [Read more...](#advanced-wiretype)
- **Advanced:** Removes two's complement compatibilty from `intToBigInt` parser method. [Read more...](#advanced-signed-bigint)
- **Advanced:** Refactorings and less visible updates. [Read more...](#advanced-refactorings)

### Reducing Wrapper Types

With this release we are aiming to reduce unnecessary "wrapper" types, which are used in the internals of the codebase, but shouldn't be pushed onto the user/developer.

This breaks the signatures of many functions:

- `signMessageHashRsv`, `signWithKey` now return the message signature as a `string` directly.
- `nextSignature`, `nextVerification`, `publicKeyFromSignatureVrs`, `publicKeyFromSignatureRsv` now take in the message signature as a `string`.

### Clarity Version

Contract Deploys now default to Clarity version `3`, which is the latest version introduced with the Stacks Nakamoto update.

### Stacks Network

From now on "network" objects are static (aka constants) and don't require instantiation.

The `@stacks/network` package exports the following network objects:

- `STACKS_MAINNET`
- `STACKS_TESTNET`
- `STACKS_DEVNET`
- `STACKS_MOCKNET` (alias for `STACKS_DEVNET`)

```ts
import { STACKS_MAINNET } from '@stacks/network';
import { STACKS_TESTNET } from '@stacks/network';
import { STACKS_DEVNET } from '@stacks/network';

console.log(STACKS_MAINNET);
// {
//   chainId: 1,
//   transactionVersion: 0,
//   peerNetworkId: 385875968,
//   magicBytes: 'X2',
//   bootAddress: 'SP000000000000000000002Q6VF78',
//   addressVersion: { singleSig: 22, multiSig: 20 },
//   client: { baseUrl: 'https://api.mainnet.hiro.so' }
// }
```

After importing the network object (e.g. `STACKS_MAINNET` here), you can use it in other functions as the `network` parameter.
Most of the time it's easier to just set the `network` parameter to a string literal (`'mainnet'`, `'testnet'`, or `'devnet'`).

As part of the network object, the `client` property was added.
This contains a `baseUrl` property and can optionally contain a `fetch` property.

For easing the transition, the functions which depended on a network instance now accept an optional `client` parameter.
The `client` parameter can be any object containing a `baseUrl` and `fetch` property.

- The `baseUrl` property should be a string containing the base URL of the Stacks node you want to use.
- The `fetch` property can be any (fetch)[https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API] compatible function.

The following diffs show examples of how to migrate to the new pattern.

```diff
import { makeSTXTokenTransfer } from '@stacks/transactions';

- import { StacksTestnet } from '@stacks/network';
+ import { STACKS_TESTNET } from '@stacks/network';

const transaction = await makeSTXTokenTransfer({
  // ...
- network: new StacksTestnet(),
+ network: STACKS_TESTNET,
});
```

> [!NOTE]
> String literal network names are still supported and the recommended way to specify the network.

```diff
const transaction = await makeSTXTokenTransfer({
  // ...
- network: new StacksTestnet(),
+ network: 'testnet',
});
```

> [!NOTE]
> Custom URLs and fetch functions are still supported via the `client` parameter or via `network.client`.

```diff
const transaction = await makeSTXTokenTransfer({
  // ...
- network: new StacksTestnet({ url: "mynode-optional.com", fetchFn: myFetch }), // optional options
+ network: STACKS_TESTNET,
+ client: { baseUrl: "mynode-optional.com", fetch: myFetch } // optional params
});
```

```diff
const transaction = await makeSTXTokenTransfer({
  // ...
- network: new StacksTestnet({ url: "mynode-optional.com", fetchFn: myFetch }), // optional options
+ network: {
+  ...STACKS_TESTNET,
+  client: { baseUrl: "mynode-optional.com", fetch: myFetch } // optional params
+ },
});
```

#### Impacts

- @stacks/bns: `BnsContractAddress` was removed, since `.bootAddress` is now a part of the network objects.
- @stacks/transactions: `AddressVersion` was moved to `@stacks/network`.

### Clarity Representation

The `ClarityType` enum was replaced by a readable version.
The previous (wire format compatible) enum is still available as `ClarityWireType`.
These types are considered somewhat internal and shouldn't cause breaking changes for most use-cases.

The property holding the value of the data type is now called `value` in all cases.
Previously, there was a mix of `value`, `list`, `buffer` etc.
For `bigint` values, the type of the `value` property is a now `string`, for better serialization compatibility.

```diff
{
-  type: 1,
+  type: "uint",
-  value: 12n,
+  value: "12",
}
```

```diff
{
-  type: 11,
+  type: "list",
-  list: [ ... ],
+  value: [ ... ],
}
```

### Post-conditions

The old `PostCondition` type was renamed to `PostConditionWire`.
A new human-readable `PostCondition` type was introduced in its place.

The previous builders (`makeStandardSTXPostCondition`, `makeStandardFungiblePostCondition`, `makeStandardNonFungiblePostCondition`, `makeContractSTXPostCondition`, `makeContractFungiblePostCondition`, and `makeContractNonFungiblePostCondition` using the enums were removed.
Use the `Pc` post-condition builder instead or construct post-condition object (of the type `PostCondition`) manually using strings and string literals.

Below are examples of the new `PostCondition` types.

```ts
// STX post-condition
const stxPostCondition: StxPostCondition = {
  type: 'stx-postcondition',
  address: 'SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B',
  condition: 'gte',
  amount: '100',
};

// Fungible token post-condition
const ftPostCondition: FungiblePostCondition = {
  type: 'ft-postcondition',
  address: 'SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B',
  condition: 'eq',
  amount: '100',
  asset: 'SP3D6PV2ACBPEKYJTCMH7HEN02KP87QSP8KTEH335.my-ft-token::my-token',
};

// Non-fungible token post-condition
const nftPostCondition: NonFungiblePostCondition = {
  type: 'nft-postcondition',
  address: 'SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B',
  condition: 'sent',
  asset: 'SP3D6PV2ACBPEKYJTCMH7HEN02KP87QSP8KTEH335.my-nft::my-asset',
  assetId: Cl.uint(602),
};
```

### Fetch Methods

The following methods were renamed:

- `estimateFee` → `fetchFeeEstimate`
- `estimateTransfer` → `fetchFeeEstimateTransfer`
- `estimateTransaction` → `fetchFeeEstimateTransaction`
- `getAbi` → `fetchAbi`
- `getNonce` → `fetchNonce`
- `getContractMapEntry` → `fetchContractMapEntry`
- `callReadOnlyFunction` → `fetchCallReadOnlyFunction`

`broadcastTransaction` wasn't renamed to highlight the uniqueness of the method.
Namely, the node/API it is sent to will "broadcast" the transaction to the mempool.

### `serialize` methods

Most users shouldn't need to use `serializeXyz` methods.
Concepts in Stacks.js have helpers like `postConditionToHex` instead.
Serialization is meant for internal representations in transactions, mostly not for user-facing data.

Existing methods now take or return **hex-encoded strings** _instead_ of `Uint8Array`s.

> If you were already converting returned bytes to hex-strings in your code, you can now skip the conversion step — hex-strings are the new default.

For easier migrating, renaming the following methods is possible to keep the previous behavior:

- `StacksTransaction.serialize` → `StacksTransaction.serializeBytes`
- `serializeAddress` → `serializeAddressBytes`
- `serializeAuthorization` → `serializeAuthorizationBytes`
- `serializeCV` → `serializeCVBytes`
- `serializeLPList` → `serializeLPListBytes`
- `serializeLPString` → `serializeLPStringBytes`
- `serializeMemoString` → `serializeMemoStringBytes`
- `serializeMessageSignature` → `serializeMessageSignatureBytes`
- `serializeMultiSigSpendingCondition` → `serializeMultiSigSpendingConditionBytes`
- `serializePayload` → `serializePayloadBytes`
- `serializePostCondition` → `serializePostConditionBytes`
- `serializePublicKey` → `serializePublicKeyBytes`
- `serializeSingleSigSpendingCondition` → `serializeSingleSigSpendingConditionBytes`
- `serializeSpendingCondition` → `serializeSpendingConditionBytes`
- `serializeStacksMessage` → `serializeStacksMessageBytes`
- `serializeStacksMessage` → `serializeStacksWireBytes`
- `serializeTransactionAuthField` → `serializeTransactionAuthFieldBytes`

### Asset Helper Methods

The following interfaces and methods were renamed:

- `AssetInfo` → `Asset`
- `StacksWireType.AssetInfo` → `StacksWireType.Asset`
- `createAssetInfo` → `createAsset`
- `parseAssetInfoString` → `parseAssetString`

### CLI

- Removed the `authenticator` method for legacy Blockstack authentication.

### Triplesec

Support for encrypting/decrypting mnemonics with `triplesec` was removed.
This impacts the methods: `decrypt`, `decryptMnemonic`, and `decryptLegacy`.
Make sure to update your code to if mnemonics are stored somewhere encrypted using the legacy method.

### Advanced: WireType

Renamed internals to avoid confusion between "message" and wire-format for serialization.
This is only used for advanced serialization use-cases internally and should not be needed for most users.

- `StacksMessage` → `StacksWire`
- `StacksMessageType` → `StacksWireType`
- `serializeStacksMessage` → `serializeStacksWireBytes`

More types were renamed to indicate use for serialization to _wire-format_:

- `MessageSignature` → `MessageSignatureWire`
- `StacksPublicKey` → `PublicKeyWire`
- `TransactionAuthField` → `TransactionAuthFieldWire`
- `Asset` → `AssetWire`
- `Address` → `AddressWire`
- `PostCondition` → `PostConditionWire`
- `PostConditionPrincipal` → `PostConditionPrincipalWire`
- `STXPostCondition` → `STXPostConditionWire`
- `FungiblePostCondition` → `FungiblePostConditionWire`
- `NonFungiblePostCondition` → `NonFungiblePostConditionWire`
- `LengthPrefixedString` → `LengthPrefixedStringWire`
- `CoinbasePayload` → `CoinbasePayloadWire`
- `PoisonPayload` → `PoisonPayloadWire`
- `SmartContractPayload` → `SmartContractPayloadWire`
- `TokenTransferPayload` → `TokenTransferPayloadWire`
- `VersionedSmartContractPayload` → `VersionedSmartContractPayloadWire`
- `NakamotoCoinbasePayload` → `NakamotoCoinbasePayloadWire`
- `TenureChangePayload` → `TenureChangePayloadWire`
- `StandardPrincipal` → `StandardPrincipalWire`
- `ContractPrincipal` → `ContractPrincipalWire`

### Advanced: Signed BigInt

The `intToBigInt` method no longer supports two's complement signed integers and removed the `signed` boolean parameter.
This likely was a misunderstood and unused feature.

### Advanced: Refactorings

- The `StacksTransaction` was renamed to `StacksTransactionWire` and its class constructor was updated to take in an options object instead of individual parameters. In the future a new `StacksTransaction` may be introduced to replace it, with more human-readable properties.
- `postConditionMode` now also accepts string literals: `'deny'`, `'allow'` (in addition to the imported `PostConditionMode` enum).
- `anchorMode` and related items were deprecated, since they no longer change behavior on-chain.
- `pubKeyfromPrivKey` was renamed `privateKeyToPublic`
- `encodeStructuredData` now returns a hex-encoded string instead of a Uint8Array (use `encodeStructuredDataBytes` if you need the raw bytes).
- `decodeStructuredDataSignature` now returns a hex-encoded string instead of a Uint8Array (use `decodeStructuredDataSignatureBytes` if you need the raw bytes). Also fixes a bug that previously tried to parse input strings as UTF-8 bytes instead of hex-encoded strings.
- `hashStructuredData` now returns a hex-encoded string instead of a Uint8Array (use `hashStructuredDataBytes` if you need the raw bytes).
- `AddressHashMode`: The `Serialize` prefixes were removed for brevity.
- `makeRandomPrivKey` was renamed to `randomPrivateKey` and now returns a compressed private key.
- `generateSecretKey` was renamed to `randomSeedPhrase`.
- `nextYear`, `nextMonth`, `nextHour`, `makeUUID4`, `updateQueryStringParameter`, `getAesCbcOutputLength`, `getAPIUsageErrorMessage`, `isSameOriginAbsoluteUrl`, `isLaterVersion`, `getBase64OutputLength`, were marked as deprecated.
- `encrypt` and `decrypt` in `@stacks/wallet-sdk` (aliases of `encryptMnemonic` and `decryptMnemonic` in the `@stacks/encryption` package respectively) were removed.
- `makeECPrivateKey` in `@stacks/encryption` was deprecated, use `randomPrivateKey` instead.
- `makeSigHashPreSign` was renamed to `sigHashPreSign` and marked as internal.
- `makeSigHashPostSign` was renamed to `sigHashPostSign` and marked as internal.
- `@stacks/wallet-sdk` `WalletConfig` types and helpers were marked as deprecated.

## Stacks.js (&lt;=4.x.x) → (5.x.x)

### Breaking Changes

- To reduce the bundle sizes of applications using Stacks.js, we are switching from Buffer (a polyfill to match Node.js APIs) to Uint8Arrays (which Buffers use in the background anyway). [Read more...](#buffer-to-uint8array)
- To allow message signing on Ledger hardware wallets, we are changing the message signing prefix. [Read more...](#message-signing-prefix)
- Post-conditions for NFTs were renamed to be more clear: `Owns` to `DoesNotSend`, `DoesNotOwn` to `Sends`.

#### Buffer to Uint8Array

To make the switch easier we have introduced a bunch of methods for converting between strings and Uint8Arrays: `hexToBytes`, `bytesToHex`, `utf8ToBytes`, `bytesToUtf8`, `asciiToBytes`, `bytesToAscii`, and `concatBytes`.
To migrate, switch `Buffer` code to instead use `Uint8Array`.
The following code segments are the equivalent calls using Uint8Array rather than Buffers and assuming imports from `@stacks/common` — `import { hexToBytes, bytesToHex, utf8ToBytes, bytesToUtf8, asciiToBytes, bytesToAscii, concatBytes } from "@stacks/common"`

```ts
// old:
Buffer.from('stacks Ӿ'); // <Buffer 73 74 61 63 6b 73 20 d3 be>

// new:
utf8ToBytes('stacks Ӿ'); // Uint8Array(9) [ 115, 116, 97, 99, 107, 115, 32, 211, 190 ];
```

```ts
// old:
Buffer.from([115, 116, 97, 99, 107, 115, 32, 211, 190]).toString(); // 'stacks Ӿ'

// new:
bytesToUtf8(Uint8Array.from([115, 116, 97, 99, 107, 115, 32, 211, 190])); // 'stacks Ӿ'
```

```ts
// old:
Buffer.from('stacks $', 'ascii'); // <Buffer 73 74 61 63 6b 73 20 24>

// new:
asciiToBytes('stacks $'); // Uint8Array(8) [ 115, 116, 97, 99, 107, 115, 32, 36 ]
```

```ts
// old:
Buffer.from([115, 116, 97, 99, 107, 115, 32, 36]).toString('ascii'); // 'stacks $'

// new:
bytesToAscii(Uint8Array.from([115, 116, 97, 99, 107, 115, 32, 36])); // 'stacks $'
```

```ts
// old:
Buffer.from('deadbeef', 'hex'); // <Buffer de ad be ef>

// new:
hexToBytes('deadbeef'); // Uint8Array(4) [ 222, 173, 190, 239 ]
```

```ts
// old:
Buffer.from([222, 173, 190, 239]).toString('hex'); // 'deadbeef'

// new:
bytesToHex(Uint8Array.from([222, 173, 190, 239])); // 'deadbeef'
```

#### Message Signing Prefix

The message signing prefix was changed from `Stacks Message Signing` to `Stacks Signed Message`.
The change relates to the functions `verifyMessageSignature`, `encodeMessage`, `decodeMessage`, and `hashMessage`.
The `verifyMessageSignature` functions was updated to verify against both the old and the new prefix (for unhashed message-input).
This will generate a different hash/signature from the same input compared to previous versions of Stacks.js.
If you have previously stored messages/signatures and compare to freshly generated ones, the messages/signatures will not match to previously stored.

---

## blockstack.js → Stacks.js (1.x.x)

This guide will help migrate your Blockstack app from blockstack.js to the new Stacks.js packages and Connect.

### Auth <!-- omit from toc -->

The main change for auth is that the Stacks Connect library has replaced the `redirectToSignIn` function from blockstack.js.
Instead of redirecting to the now deprecated Blockstack Browser, the authentication flow is completed within a popup window using
the new authenticator app.
You can still use the API in `@stacks/auth` to create custom auth requests manually if desired.

### Using blockstack.js

```typescript
import { UserSession, AppConfig } from 'blockstack';

// Configuring your app
const appConfig = new AppConfig();
const userSession = new UserSession({ appConfig });

// Initiating auth flow
if (!userSession.isUserSignedIn()) {
  userSession.redirectToSignIn();
}

// Handling sign in
if (userSession.isSignInPending()) {
  userSession.handlePendingSignIn().then(userData => {
    window.history.replaceState({}, document.title, '/');
    this.setState({ userData: userData });
  });
}
```

#### Using Blockstack Connect

```typescript
// Configuring your app
const authOptions = {
  redirectTo: '/',
  finished: ({ userSession }) => {
    console.log(userSession.loadUserData());
  },
  appDetails: {
    name: 'My Cool App',
    icon: 'https://example.com/icon.png',
  },
};

import { showBlockstackConnect } from '@stacks/connect';
import { UserSession, AppConfig } from '@stacks/auth';
import { Connect } from '@stacks/connect';

// Initiating auth flow - using the Connect component
const App = () => <Connect authOptions={authOptions}>// the rest of your app's components</Connect>;

// Initiating auth flow - alternatively
showBlockstackConnect(authOptions);

// Handling sign in
const appConfig = new AppConfig();
const userSession = new UserSession({ appConfig });

// ... call this code on page load
if (userSession.isSignInPending()) {
  const userData = await userSession.handlePendingSignIn();
  // your user is now logged in.
}
```

### Storage

In Stacks.js, storage is now a separate package.

#### Using blockstack.js

```typescript
import { UserSession, AppConfig } from 'blockstack';

const appConfig = new AppConfig();
const userSession = new UserSession({ appConfig });

userSession.putFile('my_file.json', my_content);
userSession.getFile('my_file.json').then(file => {});
```

#### Using @stacks/storage

```typescript
import { UserSession } from '@stacks/auth';
import { Storage } from '@stacks/storage';

const appConfig = new AppConfig();
const userSession = new UserSession({ appConfig });
const storage = new Storage({userSession});

storage.putFile('my_file.json', my_content));
storage.getFile('my_file.json').then((file) => {

});

```

### Encryption

Encryption/Decryption functions have been moved into a separate `@stacks/encryption` library.

#### Using blockstack.js

```typescript
import { encryptContent, decryptContent } from 'blockstack';

encryptContent(userSession, content, options);
decryptContent(userSession, encryptedContent, options);
```

#### Using @stacks/encryption or @stacks/auth

```typescript
import { encryptContent, decryptContent } from '@stacks/encryption';
import { UserSession } from '@stacks/auth';

encryptContent(content, { privateKey });
decryptContent(encryptedContent, { privateKey });

// Using userSession
const appConfig = new AppConfig();
const userSession = new UserSession({ appConfig });
const storage = new Storage(userSession);

userSession.encryptContent(content);
userSession.decryptContent(encryptedContent);
```
