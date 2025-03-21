import { drizzle } from "drizzle-orm/postgres-js";
import postgres from "postgres";
import { sql } from "drizzle-orm";
import fs from "fs";
import path from "path";

interface Borrower {
  address: string;
  collateral: Set<string>;
  debt: Set<string>;
}

async function generateBorrowersFile() {
  const connectionString = process.env.DATABASE_URL!;
  const client = postgres(connectionString);
  const db = drizzle(client);

  // Get the latest block number from _ponder_status
  const latestBlock = await db
    .select({ blockNumber: sql<number>`block_number` })
    .from(sql`_ponder_status`)
    .limit(1)
    .then((result) => result[0]?.blockNumber || 0);

  const borrowers: Record<string, Borrower> = {};
  let borrowerCount = 0;

  const borrowEvents = await db
    .select({
      onBehalfOf: sql<string>`on_behalf_of`,
      reserve: sql<string>`reserve`,
      time: sql<number>`time`,
    })
    .from(sql`"borrowEvent"`)
    .orderBy(sql`time`);

  for (const event of borrowEvents) {
    const user = event.onBehalfOf.toLowerCase();

    if (borrowers[user]) {
      borrowers[user].debt.add(event.reserve.toLowerCase());
    } else {
      borrowers[user] = {
        address: user,
        collateral: new Set(),
        debt: new Set([event.reserve.toLowerCase()]),
      };
      borrowerCount++;
    }
  }

  const supplyEvents = await db
    .select({
      onBehalfOf: sql<string>`on_behalf_of`,
      asset: sql<string>`asset`,
      time: sql<number>`time`,
    })
    .from(sql`"supplyEvent"`)
    .orderBy(sql`time`);

  for (const event of supplyEvents) {
    const user = event.onBehalfOf.toLowerCase();

    if (borrowers[user]) {
      borrowers[user].collateral.add(event.asset.toLowerCase());
    }
  }

  const serializedBorrowers: Record<
    string,
    {
      address: string;
      collateral: string[];
      debt: string[];
    }
  > = {};

  for (const [address, borrower] of Object.entries(borrowers)) {
    serializedBorrowers[address] = {
      address: borrower.address,
      collateral: Array.from(borrower.collateral),
      debt: Array.from(borrower.debt),
    };
  }

  const stateCache = {
    last_block_number: latestBlock,
    borrowers: serializedBorrowers,
  };

  const outputPath = path.join(
    process.cwd(),
    "borrowers-hyperevm-mainnet.json"
  );
  fs.writeFileSync(outputPath, JSON.stringify(stateCache, null, 2));

  console.log(`Generated borrowers file with ${borrowerCount} borrowers`);
  console.log(`Latest block: ${latestBlock}`);
  console.log(`Output saved to: ${outputPath}`);

  await client.end();
}

generateBorrowersFile().catch(console.error);
