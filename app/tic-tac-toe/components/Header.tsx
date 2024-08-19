"use client";

import React from "react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import Image from "next/image";

function Header() {
  return (
    <header className="flex flex-col lg:flex-row justify-between h-32 items-center p-5">
      <div className="flex h-full lg:w-1/2 p-3 items-center">
        <Image
          src={"/images/tic-tac-toe-logo.png"}
          width={80}
          height={80}
          alt="Tic-Tac-Toe logo"
        />
        <h1 className="ml-5 text-3xl md:text-4xl lg:text-5xl font-extrabold text-transparent bg-clip-text bg-gradient-to-br from-white to-[#4A79FC]">
          Tic-Tac-Toe
        </h1>
      </div>
      <div className="h-min-full hover:zinc-900 rounded mr-5 mt-5 md:mt-0">
        <WalletMultiButton style={{ backgroundColor: "#4A79FC" }} />
      </div>
    </header>
  );
}

export default Header;
