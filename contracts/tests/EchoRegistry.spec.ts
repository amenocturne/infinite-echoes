import { Blockchain, SandboxContract, TreasuryContract } from '@ton/sandbox';
import { Address, beginCell, Cell, toNano } from '@ton/core';
import '@ton/test-utils';

import { EchoRegistry } from '../build/EchoRegistry/EchoRegistry_EchoRegistry';

describe('EchoRegistry', () => {
    let blockchain: Blockchain;
    let deployer: SandboxContract<TreasuryContract>;
    let registry: SandboxContract<EchoRegistry>;
    let vaultCode: Cell;

    beforeEach(async () => {
        blockchain = await Blockchain.create();
        deployer = await blockchain.treasury('deployer');

        vaultCode = beginCell().endCell();
        registry = blockchain.openContract(await EchoRegistry.fromInit(vaultCode));
        // do transaction to activate contract
        await registry.send(
            deployer.getSender(),
            { value: toNano('0.1') },
            {
                $$type: 'CreateVault',
            },
        );
    });

    it('should deploy', async () => {
        expect(registry.address).toBeDefined();
        const contract = await blockchain.getContract(registry.address);
        expect(contract).toBeDefined();
    });

    it('should create a vault for a user and return its address', async () => {
        const user = await blockchain.treasury('user1');

        const createVaultResult = await registry.send(
            user.getSender(),
            { value: toNano('0.2') },
            { $$type: 'CreateVault' },
        );

        expect(createVaultResult.transactions).toHaveTransaction({
            from: user.address,
            to: registry.address,
            success: true,
        });

        const vaultAddress = await registry.getGetVaultAddress(user.address);
        expect(Address.isAddress(vaultAddress)).toBe(true);
    });

    it('should not allow creating a vault twice for the same user', async () => {
        const user = await blockchain.treasury('user2');

        // First creation should succeed
        const firstResult = await registry.send(
            user.getSender(),
            {
                value: toNano('0.2'),
            },
            { $$type: 'CreateVault' },
        );

        expect(firstResult.transactions).toHaveTransaction({
            from: user.address,
            to: registry.address,
            success: true,
        });

        // Second creation - check that the transaction exists but fails with an error
        const secondResult = await registry.send(
            user.getSender(),
            {
                value: toNano('0.2'),
            },
            { $$type: 'CreateVault' },
        );

        // Instead of expecting a rejection, check that the transaction exists but fails
        expect(secondResult.transactions).toHaveTransaction({
            from: user.address,
            to: registry.address,
            success: false,
        });
    });

    it('should return undefined for users without a vault', async () => {
        const user = await blockchain.treasury('user3');
        const vaultAddress = await registry.getGetVaultAddress(user.address);
        expect(vaultAddress === null).toBe(true);
    });
});
