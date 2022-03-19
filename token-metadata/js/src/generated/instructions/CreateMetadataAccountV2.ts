/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as definedTypes from '../types';
import * as beet from '@metaplex-foundation/beet';
import * as web3 from '@solana/web3.js';

/**
 * @category Instructions
 * @category CreateMetadataAccountV2
 * @category generated
 */
export type CreateMetadataAccountV2InstructionArgs = {
  createMetadataAccountArgsV2: definedTypes.CreateMetadataAccountArgsV2;
};
/**
 * @category Instructions
 * @category CreateMetadataAccountV2
 * @category generated
 */
const CreateMetadataAccountV2Struct = new beet.FixableBeetArgsStruct<
  CreateMetadataAccountV2InstructionArgs & {
    instructionDiscriminator: number;
  }
>(
  [
    ['instructionDiscriminator', beet.u8],
    ['createMetadataAccountArgsV2', definedTypes.createMetadataAccountArgsV2Beet],
  ],
  'CreateMetadataAccountV2InstructionArgs',
);
/**
 * Accounts required by the _CreateMetadataAccountV2_ instruction
 *
 * @property [_writable_] metadata Metadata key (pda of ['metadata', program id, mint id])
 * @property [] mint Mint of token asset
 * @property [**signer**] mintAuthority Mint authority
 * @property [**signer**] payer payer
 * @property [] updateAuthority update authority info
 * @category Instructions
 * @category CreateMetadataAccountV2
 * @category generated
 */
export type CreateMetadataAccountV2InstructionAccounts = {
  metadata: web3.PublicKey;
  mint: web3.PublicKey;
  mintAuthority: web3.PublicKey;
  payer: web3.PublicKey;
  updateAuthority: web3.PublicKey;
};

const createMetadataAccountV2InstructionDiscriminator = 16;

/**
 * Creates a _CreateMetadataAccountV2_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category CreateMetadataAccountV2
 * @category generated
 */
export function createCreateMetadataAccountV2Instruction(
  accounts: CreateMetadataAccountV2InstructionAccounts,
  args: CreateMetadataAccountV2InstructionArgs,
) {
  const { metadata, mint, mintAuthority, payer, updateAuthority } = accounts;

  const [data] = CreateMetadataAccountV2Struct.serialize({
    instructionDiscriminator: createMetadataAccountV2InstructionDiscriminator,
    ...args,
  });
  const keys: web3.AccountMeta[] = [
    {
      pubkey: metadata,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: mint,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: mintAuthority,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: payer,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: updateAuthority,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: web3.SYSVAR_RENT_PUBKEY,
      isWritable: false,
      isSigner: false,
    },
  ];

  const ix = new web3.TransactionInstruction({
    programId: new web3.PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s'),
    keys,
    data,
  });
  return ix;
}
