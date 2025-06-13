#!/usr/bin/env python3
import secrets
import hashlib
import base58

def generate_stacks_keypair():
    """Generate a Stacks private key and address"""
    
    # Generate 32-byte private key
    private_key_bytes = secrets.token_bytes(32)
    private_key_hex = private_key_bytes.hex()
    
    # For Stacks addresses, we use a simplified approach
    # In production, this would use proper secp256k1 key derivation
    
    # Generate a mock public key hash (in production, derive from private key)
    pubkey_hash = hashlib.sha256(private_key_bytes).digest()[:20]
    
    # Stacks mainnet address version (22 = 'S')
    version_byte = 22
    
    # Create address payload
    address_payload = bytes([version_byte]) + pubkey_hash
    
    # Calculate checksum (double SHA256)
    checksum = hashlib.sha256(hashlib.sha256(address_payload).digest()).digest()[:4]
    
    # Create full address
    full_address = address_payload + checksum
    
    # Encode in base58
    stacks_address = base58.b58encode(full_address).decode('utf-8')
    
    return private_key_hex, stacks_address

def generate_bitcoinz_address(private_key_hex):
    """Generate corresponding BitcoinZ address for rewards"""
    
    # For BitcoinZ mainnet, version byte is 0x1C (t1...)
    version_byte = 0x1C
    
    # Use private key to generate pubkey hash
    private_key_bytes = bytes.fromhex(private_key_hex)
    pubkey_hash = hashlib.sha256(private_key_bytes).digest()[:20]
    
    # Create address payload
    address_payload = bytes([version_byte]) + pubkey_hash
    
    # Calculate checksum
    checksum = hashlib.sha256(hashlib.sha256(address_payload).digest()).digest()[:4]
    
    # Create full address
    full_address = address_payload + checksum
    
    # Encode in base58
    bitcoinz_address = base58.b58encode(full_address).decode('utf-8')
    
    return bitcoinz_address

if __name__ == "__main__":
    # Generate keypair
    private_key, stacks_address = generate_stacks_keypair()
    bitcoinz_address = generate_bitcoinz_address(private_key)
    
    print(f"PRIVATE_KEY={private_key}")
    print(f"STACKS_ADDRESS={stacks_address}")
    print(f"BITCOINZ_ADDRESS={bitcoinz_address}")
