import { toNano } from '@ton/core';
import { Counter } from '../build/Counter/Counter_Counter';
import { NetworkProvider } from '@ton/blueprint';

export async function run(provider: NetworkProvider) {
    const counter = provider.open(await Counter.fromInit(BigInt(Math.floor(Math.random() * 10000)), 0n));

    await counter.send(
        provider.sender(),
        {
            value: toNano('0.05'),
        },
        null,
    );

    await provider.waitForDeploy(counter.address);

    console.log('ID', await counter.getId());
}
