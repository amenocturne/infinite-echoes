import { Blockchain, SandboxContract, TreasuryContract } from '@ton/sandbox';
import { Address, beginCell, Cell, toNano, TupleItemInt } from '@ton/core';
import '@ton/test-utils';

import { EchoRegistry } from '../build/EchoRegistry/EchoRegistry_EchoRegistry';

describe('EchoRegistry', () => {
    let blockchain: Blockchain;
    let deployer: SandboxContract<TreasuryContract>;
    let registry: SandboxContract<EchoRegistry>;
    let vaultCode: Cell;
    let pieceCode: Cell;

    beforeEach(async () => {
        blockchain = await Blockchain.create();
        deployer = await blockchain.treasury('deployer');

        vaultCode = beginCell().endCell();
        pieceCode = beginCell().endCell();
        registry = blockchain.openContract(await EchoRegistry.fromInit(vaultCode, pieceCode));
        const user = await blockchain.treasury('user1');
        const result = await registry.send(user.getSender(), { value: toNano('0.2') }, null);
    });

    it('should deploy', async () => {
        expect(registry.address).toBeDefined();
        const contract = await blockchain.getContract(registry.address);
        expect(contract).toBeDefined();
    });

    it('should create vault and piece on first createPiece', async () => {
        const user = await blockchain.treasury('user1');
        const pieceData = beginCell().storeUint(123, 256).endCell();

        const result = await registry.send(
            user.getSender(),
            { value: toNano('0.2') },
            {
                $$type: 'CreatePiece',
                pieceData: pieceData,
                remixedFrom: null,
            },
        );

        // Check registry transaction success
        expect(result.transactions).toHaveTransaction({
            from: user.address,
            to: registry.address,
            success: true,
        });

        // Verify vault was created
        const vaultAddress = await registry.getGetVaultAddress(user.address);
        expect(Address.isAddress(vaultAddress)).toBe(true);

        // Verify piece was created and added to vault
        const vaultContract = await blockchain.getContract(vaultAddress!);
        const vaultState = await vaultContract.get('getPieceCount');
        expect((vaultState.stack[0] as TupleItemInt).value).toBeGreaterThan(0n);
    });

    it('should reuse existing vault for subsequent pieces', async () => {
        const user = await blockchain.treasury('user2');
        const pieceData1 = beginCell().storeUint(111, 256).endCell();
        const pieceData2 = beginCell().storeUint(222, 256).endCell();

        // First piece creates vault
        await registry.send(
            user.getSender(),
            { value: toNano('0.2') },
            {
                $$type: 'CreatePiece',
                pieceData: pieceData1,
                remixedFrom: null,
            },
        );

        const firstVaultAddress = await registry.getGetVaultAddress(user.address);

        // Second piece should reuse vault
        const result = await registry.send(
            user.getSender(),
            { value: toNano('0.2') },
            {
                $$type: 'CreatePiece',
                pieceData: pieceData2,
                remixedFrom: null,
            },
        );

        // Verify same vault address
        const secondVaultAddress = await registry.getGetVaultAddress(user.address);
        expect(secondVaultAddress!.toString()).toEqual(firstVaultAddress!.toString());

        // Verify vault has 2 pieces
        const vaultContract = await blockchain.getContract(secondVaultAddress!);
        const vaultState = await vaultContract.get('getPieceCount');
        expect((vaultState.stack[0] as TupleItemInt).value).toBeGreaterThan(1n);
    });

    it('should not return vault for users without pieces', async () => {
        const user = await blockchain.treasury('user3');
        const vaultAddress = await registry.getGetVaultAddress(user.address);
        expect(vaultAddress).toBeNull();
    });
});
