"use client"; // Ensures it's treated as a Client Component

import { FC, useMemo } from "react";
import {
  ConnectionProvider,
  WalletProvider
} from "@solana/wallet-adapter-react";
import {
  WalletModalProvider,
  WalletMultiButton,
} from "@solana/wallet-adapter-react-ui";
import { clusterApiUrl } from "@solana/web3.js";

const Home: FC = () => {
  // Ensure the wallet initialization only runs on the client side
  const endpoint = clusterApiUrl("devnet");
  const wallets = useMemo(() => [], []); // Use memoization for performance optimization

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets}>
        <WalletModalProvider>
          <WalletMultiButton />
          <p>Put the rest of your app here</p>
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
};

export default Home;
