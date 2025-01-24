import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey, SystemProgram } from '@solana/web3.js';
import { FavoritesProgram } from '../target/types/favorites_program';

describe('favorites-program', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .FavoritesProgram as Program<FavoritesProgram>;

  it('Set Favorites!', async () => {
    // Add your test here.
    let number = new anchor.BN(30);
    let color = 'Blue';
    let hobbies = ['reading', 'coding', 'learning'];

    const user = program.provider.publicKey;

    const [favoritesPda, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from('favorites'), user.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .setFavorites(number, color, hobbies)
      .accounts({
        user,
      })
      .rpc();
    console.log('Your transaction signature', tx);
  });
});
