BIPs that are implemented by Zcash (up-to-date up to **v1.1.0**):

* Numerous historic BIPs were present in **v1.0.0** at launch; see [the protocol spec](https://github.com/zcash/zips/blob/master/protocol/protocol.pdf) for details.
* [`BIP 111`](https://github.com/bitcoin/bips/blob/master/bip-0111.mediawiki): `NODE_BLOOM` service bit added, but only enforced for peer versions `>=170004` as of **v1.1.0** ([PR #2814](https://github.com/zcash/zcash/pull/2814)).
* [`BIP 130`](https://github.com/bitcoin/bips/blob/master/bip-0130.mediawiki): direct headers announcement is negotiated with peer versions `>=770009` as of **v0.12.0** ([PR 6494](https://github.com/bitcoin/bitcoin/pull/6494)).
* [`BIP 133`](https://github.com/bitcoin/bips/blob/master/bip-0133.mediawiki): feefilter messages are respected and sent for peer versions `>=770009` as of **v0.13.0** ([PR 7542](https://github.com/bitcoin/bitcoin/pull/7542)).
