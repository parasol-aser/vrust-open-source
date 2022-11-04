const fs = require("fs");
["lib/esm", "lib/cjs"].forEach((buildPath) => {
  fs.copyFileSync(
    `src/solana/core/bridge_bg.wasm`,
    `${buildPath}/solana/core/bridge_bg.wasm`
  );
  fs.copyFileSync(
    `src/solana/core-node/bridge_bg.wasm`,
    `${buildPath}/solana/core-node/bridge_bg.wasm`
  );
  fs.copyFileSync(
    `src/solana/core/bridge_bg.wasm.d.ts`,
    `${buildPath}/solana/core/bridge_bg.wasm.d.ts`
  );
  fs.copyFileSync(
    `src/solana/core-node/bridge_bg.wasm.d.ts`,
    `${buildPath}/solana/core-node/bridge_bg.wasm.d.ts`
  );
  fs.copyFileSync(
    `src/solana/nft/nft_bridge_bg.wasm`,
    `${buildPath}/solana/nft/nft_bridge_bg.wasm`
  );
  fs.copyFileSync(
    `src/solana/nft-node/nft_bridge_bg.wasm`,
    `${buildPath}/solana/nft-node/nft_bridge_bg.wasm`
  );
  fs.copyFileSync(
    `src/solana/nft/nft_bridge_bg.wasm.d.ts`,
    `${buildPath}/solana/nft/nft_bridge_bg.wasm.d.ts`
  );
  fs.copyFileSync(
    `src/solana/nft-node/nft_bridge_bg.wasm.d.ts`,
    `${buildPath}/solana/nft-node/nft_bridge_bg.wasm.d.ts`
  );
  fs.copyFileSync(
    `src/solana/token/token_bridge_bg.wasm`,
    `${buildPath}/solana/token/token_bridge_bg.wasm`
  );
  fs.copyFileSync(
    `src/solana/token-node/token_bridge_bg.wasm`,
    `${buildPath}/solana/token-node/token_bridge_bg.wasm`
  );
  fs.copyFileSync(
    `src/solana/token/token_bridge_bg.wasm.d.ts`,
    `${buildPath}/solana/token/token_bridge_bg.wasm.d.ts`
  );
  fs.copyFileSync(
    `src/solana/token-node/token_bridge_bg.wasm.d.ts`,
    `${buildPath}/solana/token-node/token_bridge_bg.wasm.d.ts`
  );
  fs.copyFileSync(
    `src/solana/migration/wormhole_migration_bg.wasm`,
    `${buildPath}/solana/migration/wormhole_migration_bg.wasm`
  );
  fs.copyFileSync(
    `src/solana/migration-node/wormhole_migration_bg.wasm`,
    `${buildPath}/solana/migration-node/wormhole_migration_bg.wasm`
  );
  fs.copyFileSync(
    `src/solana/migration/wormhole_migration_bg.wasm.d.ts`,
    `${buildPath}/solana/migration/wormhole_migration_bg.wasm.d.ts`
  );
  fs.copyFileSync(
    `src/solana/migration-node/wormhole_migration_bg.wasm.d.ts`,
    `${buildPath}/solana/migration-node/wormhole_migration_bg.wasm.d.ts`
  );
});