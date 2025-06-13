#include <chainparams.h>
#include <key_io.h>
#include <zcash/Address.hpp>

#include "utiltest.h"

#include <variant>

#include <gtest/gtest.h>

TEST(Keys, EncodeAndDecodeSapling)
{
    SelectParams(CBaseChainParams::MAIN);

    auto m = GetTestMasterSaplingSpendingKey();

    for (uint32_t i = 0; i < 1000; i++) {
        auto sk = m.Derive(i);
        {
            std::string sk_string = EncodeSpendingKey(sk);
            EXPECT_EQ(
                sk_string.substr(0, 24),
                Params().Bech32HRP(CChainParams::SAPLING_EXTENDED_SPEND_KEY));

            auto spendingkey2 = DecodeSpendingKey(sk_string);
            EXPECT_TRUE(IsValidSpendingKey(spendingkey2));

            ASSERT_TRUE(std::get_if<libzcash::SaplingExtendedSpendingKey>(&spendingkey2) != nullptr);
            auto sk2 = std::get<libzcash::SaplingExtendedSpendingKey>(spendingkey2);
	    EXPECT_EQ(sk, sk2);
        }
        {
            auto extfvk = sk.ToXFVK();
            std::string vk_string = EncodeViewingKey(extfvk);
            EXPECT_EQ(
                vk_string.substr(0, 7),
                Params().Bech32HRP(CChainParams::SAPLING_EXTENDED_FVK));

            auto viewingkey2 = DecodeViewingKey(vk_string);
            EXPECT_TRUE(IsValidViewingKey(viewingkey2));

            ASSERT_TRUE(std::get_if<libzcash::SaplingExtendedFullViewingKey>(&viewingkey2) != nullptr);
            auto extfvk2 = std::get<libzcash::SaplingExtendedFullViewingKey>(viewingkey2);
            EXPECT_EQ(extfvk, extfvk2);
        }
        {
            auto addr = sk.DefaultAddress();

            std::string addr_string = EncodePaymentAddress(addr);
            EXPECT_EQ(
                addr_string.substr(0, 2),
                Params().Bech32HRP(CChainParams::SAPLING_PAYMENT_ADDRESS));

            auto paymentaddr2 = DecodePaymentAddress(addr_string);
            EXPECT_TRUE(IsValidPaymentAddress(paymentaddr2));

            ASSERT_TRUE(std::get_if<libzcash::SaplingPaymentAddress>(&paymentaddr2) != nullptr);
            auto addr2 = std::get<libzcash::SaplingPaymentAddress>(paymentaddr2);
            EXPECT_EQ(addr, addr2);
        }
    }
}
